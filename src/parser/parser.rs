use std::cell::Cell;

use drop_bomb::DropBomb;

use crate::parser::{
    event::Event,
    token_set::TokenSet,
    ParseError,
    SyntaxKind::{self, EOF, ERROR, TOMBSTONE},
    TokenSource,
};

/// This structure provides API for navigating through the token stream
/// and constructs a parse tree.
/// The parsing process is in `grammar/` module.
/// The result of parsing is a stream of `Event`s
pub(crate) struct Parser<'t> {
    token_source: &'t mut dyn TokenSource,
    events: Vec<Event>,
    steps: Cell<u32>,
}

impl<'t> Parser<'t> {
    pub(crate) fn new(token_source: &'t mut dyn TokenSource) -> Parser<'t> {
        Parser { token_source, events: Vec::new(), steps: Cell::new(0) }
    }

    pub(crate) fn finish(self) -> Vec<Event> {
        self.events
    }

    /// Returns the current token
    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    /// Lookahead `n` tokens
    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        let steps = self.steps.get();
        assert!(steps <= 10000000, "infinite loop detected");
        self.steps.set(steps + 1);

        self.token_source.lookahead_nth(n).kind
    }

    /// Check if the current token is specific `SyntaxKind` kind
    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        // currently we don't need support for composite tokens (e.g. `>>`)
        self.token_source.lookahead_nth(0).kind == kind
    }

    /// Consume the next token if it is of expected kind
    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }

        // currently we don't need support for composite tokens (e.g. `>>`)
        let n_raw_tokens = 1;
        self.do_bump(kind, n_raw_tokens);
        true
    }

    /// Check if current token is in the given set of tokens
    pub(crate) fn at_ts(&self, kinds: TokenSet) -> bool {
        kinds.contains(self.current())
    }

    /// Starts a new node in the syntax tree
    /// All nodes that are consumed between the start and finish of the `Marker`
    /// belongs to the same node
    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.push_event(Event::tombstone());
        Marker::new(pos)
    }

    /// Cosumes the next token if it is of expected kind
    pub(crate) fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    /// Consume any token
    pub(crate) fn bump_any(&mut self) {
        let kind = self.nth(0);
        if kind == EOF {
            return;
        }
        self.do_bump(kind, 1);
    }

    /// Create an Token event
    fn do_bump(&mut self, kind: SyntaxKind, n_raw_tokens: u8) {
        for _ in 0..n_raw_tokens {
            self.token_source.bump();
        }

        self.push_event(Event::Token { kind, n_raw_tokens });
    }

    fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Report an error with specified message
    /// This can be in future extended to support also range
    /// Right now `ParseError` is just converted to `SyntaxError`
    /// after the parsing is done and uses just token offset (not range)
    pub(crate) fn error<T: Into<String>>(&mut self, message: T) {
        let msg = ParseError(message.into());
        self.push_event(Event::Error { msg });
    }

    /// Consume the next token if it is of expected kind, otherwise report an error
    pub(crate) fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.error(format!("expected {:?}", kind));
        false
    }

    /// Create an error node and consume the next token
    /// This token belongs to the `Error` node
    pub(crate) fn err_and_bump(&mut self, message: &str) {
        self.err_recover(message, TokenSet::EMPTY)
    }

    /// Create an error node and consume the next token if it is of expected kind
    /// If the current token belongs to given recovery set, it just reports and error
    /// and tries to recover
    pub(crate) fn err_recover(&mut self, message: &str, recovery: TokenSet) {
        if self.at_ts(recovery) {
            self.error(message);
            return;
        }

        let m = self.start();
        self.error(message);
        self.bump_any();
        m.complete(self, ERROR);
    }
}

/// Marker that is used to mark the start of a new node in the syntax tree
/// It groups specific node/tokens that belongs to this node
pub(crate) struct Marker {
    pos: u32,
    bomb: DropBomb,
}

impl Marker {
    fn new(pos: u32) -> Marker {
        Marker { pos, bomb: DropBomb::new("Marker must be either completed or abandoned") }
    }

    /// Finish the syntax tree node and assign specific kind to it
    pub(crate) fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();
        let idx = self.pos as usize;
        match &mut p.events[idx] {
            Event::Start { kind: slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        p.push_event(Event::Finish);
        CompletedMarker::new(self.pos, kind)
    }

    /// Abandon the syntax tree node
    /// all the children are then attached to the parent of this node
    pub(crate) fn abandon(mut self, p: &mut Parser) {
        self.bomb.defuse();
        let idx = self.pos as usize;
        if idx == p.events.len() - 1 {
            match p.events.pop() {
                Some(Event::Start { kind: TOMBSTONE, forward_parent: None }) => (),
                _ => unreachable!(),
            }
        }
    }
}

pub(crate) struct CompletedMarker {
    pos: u32,
    kind: SyntaxKind,
}

impl CompletedMarker {
    fn new(pos: u32, kind: SyntaxKind) -> Self {
        CompletedMarker { pos, kind }
    }

    /// This allows us to create a new node which should precede the current node
    /// Parser could start node `A`, complete it. Afterwards it decides that it should
    /// have started node `B` before `A` was started.  This allows exactly that.
    pub(crate) fn precede(self, p: &mut Parser) -> Marker {
        let new_pos = p.start();
        let idx = self.pos as usize;
        match &mut p.events[idx] {
            Event::Start { forward_parent, .. } => {
                *forward_parent = Some(new_pos.pos - self.pos);
            }
            _ => unreachable!(),
        }
        new_pos
    }

    pub(crate) fn extend_to(self, p: &mut Parser, mut m: Marker) -> CompletedMarker {
        m.bomb.defuse();
        let idx = m.pos as usize;
        match &mut p.events[idx] {
            Event::Start { forward_parent, .. } => {
                *forward_parent = Some(self.pos - m.pos);
            }
            _ => unreachable!(),
        }
        self
    }

    pub(crate) fn kind(&self) -> SyntaxKind {
        self.kind
    }
}
