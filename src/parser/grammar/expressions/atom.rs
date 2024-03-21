use super::*;

/// Recover set for expressions, FIRST set is used
const EXPR_RECOVERY_SET: TokenSet = TokenSet::new(&[T![variable], T![bool_lit], T![not]]);

// So far the only literals we support are true, false and variables
// numbers will be added later
pub(crate) const LITERAL_FIRST: TokenSet =
    TokenSet::new(&[T![bool_lit], T![variable], T![string_lit], T![int_lit], T![float_lit]]);

/// Parse a literal
/// Literal right now is only: true, false, variable, string_lit or number
//pub(crate) fn literal(p: &mut Parser) -> () {
//    if !p.at_ts(LITERAL_FIRST) {
//        return None;
//    }
//    p.bump_any();
//}

/// Add support for while/for loops, if/else statements, etc.
/// Right now the only atom in expression is literal
pub(super) fn atom_expr(p: &mut Parser) {
    p.bump_any();

    // This will be extended to support more expressions later
    #[allow(clippy::match_single_binding)]
    match p.current() {
        _ => {
            p.err_recover("unsupported expression", EXPR_RECOVERY_SET);
        }
    };
}
