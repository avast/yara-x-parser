/// This module provides a way to process the events from the parser
/// It is decoupled from the parser
///
/// The `TreeSink` trait is used to connect parser and tree builder
/// Parser produces a stream of `Event`s and they are converted to a real tree
use std::mem;

use crate::parser::{
    ParseError,
    SyntaxKind::{self, *},
    TreeSink,
};

#[derive(Debug)]
pub(crate) enum Event {
    /// This event specifies the start of a new node
    /// It is either abandoned or completed with `Finish` event
    ///
    /// All children that are consumed between `Start` and `Finish` are attached to this node
    Start {
        kind: SyntaxKind,
        forward_parent: Option<u32>,
    },

    /// Complete the current node
    Finish,

    /// Add a new token to the current node
    /// `n_raw_tokens` is used for consuming multiple tokens at once that should be glued together
    /// this is not supported in YARA subset, but will be used in the future
    Token {
        kind: SyntaxKind,
        n_raw_tokens: u8,
    },

    Error {
        msg: ParseError,
    },
}

impl Event {
    pub(crate) fn tombstone() -> Self {
        Event::Start { kind: TOMBSTONE, forward_parent: None }
    }
}

/// Generate the syntax tree by processing the events
pub(crate) fn process(sink: &mut dyn TreeSink, mut events: Vec<Event>) {
    let mut forward_parents = Vec::new();

    for i in 0..events.len() {
        match mem::replace(&mut events[i], Event::tombstone()) {
            Event::Start { kind, forward_parent } => {
                forward_parents.push(kind);
                let mut idx = i;
                let mut fp = forward_parent;
                while let Some(fwd) = fp {
                    idx += fwd as usize;
                    fp = match mem::replace(&mut events[idx], Event::tombstone()) {
                        Event::Start { kind, forward_parent } => {
                            forward_parents.push(kind);
                            forward_parent
                        }
                        _ => unreachable!(),
                    };
                }

                for kind in forward_parents.drain(..).rev() {
                    if kind != TOMBSTONE {
                        sink.start_node(kind);
                    }
                }
            }
            Event::Finish => sink.finish_node(),
            Event::Token { kind, n_raw_tokens } => sink.token(kind, n_raw_tokens),
            Event::Error { msg } => sink.error(msg),
        }
    }
}
