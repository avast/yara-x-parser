use super::*;

// So far the only literals we support are true, false and variables
// numbers will be added later
pub(crate) const LITERAL_FIRST: TokenSet = TokenSet::new(&[TRUE, FALSE, VARIABLE]);

pub(crate) fn literal(p: &mut Parser) -> Option<CompletedMarker> {
    if !p.at_ts(LITERAL_FIRST) {
        return None;
    }
    let m = p.start();
    p.bump_any();
    Some(m.complete(p, LITERAL))
}

const EXPR_RECOVERY_SET: TokenSet = TokenSet::new(&[VARIABLE, TRUE, FALSE]);

// add support for while/for loops, if/else statements, etc.
pub(super) fn atom_expr(p: &mut Parser) -> Option<CompletedMarker> {
    if let Some(m) = literal(p) {
        return Some(m);
    }

    match p.current() {
        _ => {
            p.err_recover("expected expression", EXPR_RECOVERY_SET);
            return None;
        }
    };
}
