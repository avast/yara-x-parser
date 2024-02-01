use super::*;

// So far the only literals we support are true, false and variables
// numbers will be added later
pub(crate) const LITERAL_FIRST: TokenSet =
    TokenSet::new(&[T![true], T![false], T![variable], STRING, NUMBER]);

pub(crate) fn literal(p: &mut Parser) -> Option<CompletedMarker> {
    if !p.at_ts(LITERAL_FIRST) {
        return None;
    }
    let m = p.start();
    p.bump_any();
    Some(m.complete(p, LITERAL))
}

const EXPR_RECOVERY_SET: TokenSet = TokenSet::new(&[T![variable], T![true], T![false], T![not]]);

// add support for while/for loops, if/else statements, etc.
pub(super) fn atom_expr(p: &mut Parser) -> Option<CompletedMarker> {
    if let Some(m) = literal(p) {
        return Some(m);
    }

    // This will be extended to support more expressions later
    #[allow(clippy::match_single_binding)]
    match p.current() {
        _ => {
            p.err_recover("expected expression", EXPR_RECOVERY_SET);
            #[allow(clippy::needless_return)]
            return None;
        }
    };
}
