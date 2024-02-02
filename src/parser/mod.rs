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

pub trait TokenSource {
    fn current(&self) -> Token;

    fn lookahead_nth(&self, n: usize) -> Token;

    fn bump(&mut self);

    fn is_keyword(&self, kw: &str) -> bool;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Token {
    pub kind: SyntaxKind,

    pub is_jointed_to_next: bool,
}

pub trait TreeSink {
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8);

    fn start_node(&mut self, kind: SyntaxKind);

    fn finish_node(&mut self);

    fn error(&mut self, error: ParseError);
}

pub fn parse(token_source: &mut dyn TokenSource, tree_sink: &mut dyn TreeSink) {
    let mut p = parser::Parser::new(token_source);
    parse_source_file(&mut p);
    let events = p.finish();
    event::process(tree_sink, events)
}
