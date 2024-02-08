/// This is the hand-written parser and `grammar` of YARA language

mod expressions;
mod items;

use crate::parser::{
    grammar::expressions::rule_body,
    parser::{CompletedMarker, Marker, Parser},
    syntax_kind::T,
    token_set::TokenSet,
    SyntaxKind::{self, *},
};

/// Parse a source file
/// Each YARA file is a SOURCE_FILE that has some content
pub(crate) fn parse_source_file(p: &mut Parser) {
    let m = p.start();

    items::mod_content(p, false);
    m.complete(p, SOURCE_FILE);
}

/// To recover from error, we can parse block of a rule on its own
fn error_block(p: &mut Parser, message: &str) {
    assert!(p.at(T!['{']));
    let m = p.start();
    p.error(message);
    p.bump(T!['{']);
    rule_body(p);
    p.eat(T!['}']);
    m.complete(p, ERROR);
}
