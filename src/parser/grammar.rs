mod expressions;
mod items;

use crate::parser::{
    grammar::expressions::rule_body,
    parser::{CompletedMarker, Marker, Parser},
    token_set::TokenSet,
    SyntaxKind::{self, *},
};

pub(crate) fn parse_source_file(p: &mut Parser) {
    let m = p.start();

    items::mod_content(p, false);
    m.complete(p, SOURCE_FILE);
}

fn error_block(p: &mut Parser, message: &str) {
    assert!(p.at(LBRACE));
    let m = p.start();
    p.error(message);
    p.bump(LBRACE);
    rule_body(p);
    p.eat(RBRACE);
    m.complete(p, ERROR);
}

fn name_r(p: &mut Parser<'_>, recovery: TokenSet) {
    if p.at(IDENTIFIER) {
        let m = p.start();
        p.bump(IDENTIFIER);
        m.complete(p, IDENTIFIER);
    } else {
        p.err_recover("expected a name", recovery);
    }
}
