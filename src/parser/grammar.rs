use std::process::exit;

use crate::parser::{
    parser::{CompletedMarker, Marker, Parser},
    token_set::TokenSet,
    SyntaxKind::{self, *},
};

pub(crate) fn parse_source_file(p: &mut Parser) {
    let m = p.start();

    while !p.at(EOF) {
        println!("{:?}", p.current());
        p.bump_any();
    }
    m.complete(p, SOURCE_FILE);
}
