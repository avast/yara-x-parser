use crate::parser::{
    parser::{CompletedMarker, Marker, Parser},
    token_set::TokenSet,
    SyntaxKind::{self, *},
};

pub(crate) fn parse_source_file(p: &mut Parser) {
    let m = p.start();
    m.complete(p, SOURCE_FILE);
}
