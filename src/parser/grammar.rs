mod items;

use crate::parser::{
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
    // Change this to parse expression content
    while !p.at(RBRACE) {
        p.bump_any();
    }
    p.eat(RBRACE);
    m.complete(p, ERROR);
}
