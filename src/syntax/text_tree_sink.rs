use std::mem;
use text_size::{TextRange, TextSize};

use crate::{
    lexer::Token,
    parser::{ParseError, SyntaxKind, TreeSink},
    syntax::{syntax_error::SyntaxError, syntax_node::GreenNode, syntax_node::SyntaxTreeBuilder},
};

/// Used to connect parser and specific SyntaxTree representation
/// It also handles attaching trivia (whitespaces and comments) to the nodes
pub(crate) struct TextTreeSink<'a> {
    text: &'a str,
    tokens: &'a [Token],
    text_pos: TextSize,
    token_pos: usize,
    state: State,
    inner: SyntaxTreeBuilder,
}

enum State {
    PendingStart,
    Normal,
    PendingFinish,
}

impl<'a> TreeSink for TextTreeSink<'a> {
    /// Attach a token to the current node
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => {
                self.inner.finish_node();
            }
            State::Normal => (),
        }
        self.eat_trivias();
        let n_tokens = n_tokens as usize;
        let len = self.tokens[self.token_pos..self.token_pos + n_tokens]
            .iter()
            .map(|it| it.len)
            .sum::<TextSize>();
        self.do_token(kind, len, n_tokens);
    }

    /// Start a new node
    /// This method also handles attaching trivia to the node
    fn start_node(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => {
                self.inner.start_node(kind);
                return;
            }
            State::PendingFinish => {
                self.inner.finish_node();
            }
            State::Normal => (),
        }

        let n_trivias =
            self.tokens[self.token_pos..].iter().take_while(|it| it.kind.is_trivia()).count();
        let leading_trivias = &self.tokens[self.token_pos..self.token_pos + n_trivias];
        let mut trivia_end =
            self.text_pos + leading_trivias.iter().map(|it| it.len).sum::<TextSize>();

        let n_attached_trivias = {
            let leading_trivias = leading_trivias.iter().rev().map(|it| {
                let next_end = trivia_end - it.len;
                let range = TextRange::new(next_end, trivia_end);
                trivia_end = next_end;
                (it.kind, &self.text[range])
            });
            n_attached_trivias(kind, leading_trivias)
        };
        self.eat_n_trivias(n_trivias - n_attached_trivias);
        self.inner.start_node(kind);
        self.eat_n_trivias(n_attached_trivias);
    }

    /// Finish the current node
    fn finish_node(&mut self) {
        match mem::replace(&mut self.state, State::PendingFinish) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => {
                self.inner.finish_node();
            }
            State::Normal => (),
        }
    }

    fn error(&mut self, error: ParseError) {
        self.inner.error(error, self.text_pos)
    }
}

impl<'a> TextTreeSink<'a> {
    pub(crate) fn new(text: &'a str, tokens: &'a [Token]) -> Self {
        Self {
            text,
            tokens,
            text_pos: 0.into(),
            token_pos: 0,
            state: State::PendingStart,
            inner: SyntaxTreeBuilder::default(),
        }
    }

    pub(crate) fn finish(mut self) -> (GreenNode, Vec<SyntaxError>) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingFinish => {
                self.eat_trivias();
                self.inner.finish_node()
            }
            State::PendingStart | State::Normal => unreachable!(),
        }

        self.inner.finish_raw()
    }

    /// Consumes trivias until the next non-trivia token
    fn eat_trivias(&mut self) {
        while let Some(&token) = self.tokens.get(self.token_pos) {
            if !token.kind.is_trivia() {
                break;
            }
            self.do_token(token.kind, token.len, 1);
        }
    }

    /// Consumes n trivias
    fn eat_n_trivias(&mut self, n: usize) {
        for _ in 0..n {
            let token = self.tokens[self.token_pos];
            assert!(token.kind.is_trivia());
            self.do_token(token.kind, token.len, 1);
        }
    }

    /// Consumes a token and attaches it to the current node
    fn do_token(&mut self, kind: SyntaxKind, len: TextSize, n_tokens: usize) {
        let range = TextRange::at(self.text_pos, len);
        let text = &self.text[range];
        self.text_pos += len;
        self.token_pos += n_tokens;
        self.inner.token(kind, text);
    }
}

/// Returns the number of attached trivias for the given node kind
/// Trivias are attached to the node if they are directly before the node
/// and there is no empty line between the trivia and the node
fn n_attached_trivias<'a>(
    kind: SyntaxKind,
    trivias: impl Iterator<Item = (SyntaxKind, &'a str)>,
) -> usize {
    match kind {
        // Nodes that are supported to have attached trivias
        SyntaxKind::RULE | SyntaxKind::BLOCK_EXPR | SyntaxKind::STRINGS | SyntaxKind::CONDITION => {
            let mut res = 0;
            let trivias = trivias.enumerate().peekable();

            for (i, (kind, text)) in trivias {
                match kind {
                    SyntaxKind::WHITESPACE if text.contains("\n\n") => {
                        break;
                    }
                    SyntaxKind::COMMENT => {
                        res = i + 1;
                    }
                    _ => (),
                }
            }
            res
        }
        _ => 0,
    }
}
