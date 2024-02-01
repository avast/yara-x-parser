use super::*;

pub(super) const RULE_RECOVERY_SET: TokenSet = TokenSet::new(
    // Add import here when it is supported
    &[
        T![rule], // rule
    ],
);

pub(super) fn mod_content(p: &mut Parser, stop_on_r_brace: bool) {
    while !(p.at(EOF) || p.at(T!['}']) && stop_on_r_brace) {
        process_top_level(p, stop_on_r_brace);
    }
}

// process either rule, import or include
pub(super) fn process_top_level(p: &mut Parser, stop_on_r_brace: bool) {
    let m = p.start();
    let m = match opt_rule_import_include(p, m) {
        Ok(()) => {
            return;
        }
        Err(m) => m,
    };
    m.abandon(p);
    match p.current() {
        T!['{'] => {
            error_block(p, "expected an item");
        }
        T!['}'] if !stop_on_r_brace => {
            let e = p.start();
            p.error("unmatched }");
            p.bump(T!['}']);
            e.complete(p, ERROR);
        }
        EOF | T!['}'] => p.error("expected an item"),
        _ => p.err_and_bump("expected an item"),
    }
}

// So far in this prototype, we only have one kind of item: a rule.
// In the future, also imports and includes will be supported here
pub(super) fn opt_rule_import_include(p: &mut Parser, m: Marker) -> Result<(), Marker> {
    // add rule modifiers to match current and lookahead next with p.nth(1) for RULE or ERROR
    match p.current() {
        T![rule] => rule(p, m),
        _ => return Err(m),
    }
    Ok(())
}

fn rule(p: &mut Parser, m: Marker) {
    p.bump(T![rule]);
    name_r(p, RULE_RECOVERY_SET);
    // add optional support for rule tags
    expressions::block_expr(p);
    m.complete(p, RULE);
}
