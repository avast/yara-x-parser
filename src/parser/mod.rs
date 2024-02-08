//! YARA parser
//!
//! It uses abstract `TokenSource` and `TreeSink` traits.
//! It is cursor into the sequence of tokens. Parsing happens in
//! `grammar` module.

pub mod syntax_kind;

pub use syntax_kind::SyntaxKind;
mod event;
mod grammar;
#[allow(clippy::module_inception)]
mod parser;
mod token_set;

use grammar::parse_source_file;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseError(pub String);

/// `TokenSource` abstracts the source of the tokens parser uses.
///
/// This allows us to treat text and token trees in the same way.
pub trait TokenSource {
    /// Returns the current token
    fn current(&self) -> Token;

    /// Lookahead `n` tokens
    fn lookahead_nth(&self, n: usize) -> Token;

    /// Advance the cursor to the next token
    fn bump(&mut self);

    /// Check if the current token is keyword
    fn is_keyword(&self, kw: &str) -> bool;
}

/// `Token` abstracts the cursor for `TokenSource`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Token {
    pub kind: SyntaxKind,

    pub is_jointed_to_next: bool,
}

/// `TreeSink` abstracts detail of syntax tree creation.
pub trait TreeSink {
    /// Adds new token to specific node
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8);

    /// Start new node
    fn start_node(&mut self, kind: SyntaxKind);

    /// Finish the current node and return control to the parent
    fn finish_node(&mut self);

    /// Create an error with message
    fn error(&mut self, error: ParseError);
}

pub fn parse(token_source: &mut dyn TokenSource, tree_sink: &mut dyn TreeSink) {
    let mut p = parser::Parser::new(token_source);
    parse_source_file(&mut p);
    let events = p.finish();
    event::process(tree_sink, events)
}
