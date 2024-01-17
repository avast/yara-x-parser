use super::*;

pub(super) const RULE_RECOVERY_SET: TokenSet = TokenSet::new(
    // Add import here when it is supported
    &[
        RULE, // rule
    ],
);

pub(super) fn mod_content(p: &mut Parser, stop_on_r_brace: bool) {
    while !p.at(EOF) && !(p.at(RBRACE) && stop_on_r_brace) {
        import_or_rule(p, stop_on_r_brace);
    }
}

// So far in this prototype, we only have one kind of item: a rule.
// In the future, also imports will be supported here
pub(super) fn import_or_rule(p: &mut Parser, stop_on_r_brace: bool) {
    let m = p.start();
    let m = match opt_rule(p, m) {
        Ok(()) => {
            return;
        }
        Err(m) => m,
    };
    m.abandon(p);
    match p.current() {
        LBRACE => {
            error_block(p, "expected an item");
        }
        RBRACE if !stop_on_r_brace => {
            let e = p.start();
            p.error("unmatched }");
            p.bump(RBRACE);
            e.complete(p, ERROR);
        }
        EOF | RBRACE => p.error("expected an item"),
        _ => p.err_and_bump("expected an item"),
    }
}

pub(super) fn opt_rule(p: &mut Parser, m: Marker) -> Result<(), Marker> {
    match p.current() {
        RULE => rule(p, m),
        _ => return Err(m),
    }
    Ok(())
}

fn rule(p: &mut Parser, m: Marker) {
    p.bump(RULE);
    name_r(p, RULE_RECOVERY_SET);
    expressions::block_expr(p);
    m.complete(p, RULE);
}
