use super::*;

/// This is a recover set for rule, FIRST set is used to recover from error
/// This will probably also needs to be tweaked, adjusted and extended in the future
pub(super) const RULE_RECOVERY_SET: TokenSet = TokenSet::new(
    // imports could be  here when it is supported
    &[T![rule]],
);

/// Process the content of a file, Stop on `EOF` token or `}` if `stop_on_r_brace` is true
pub(super) fn mod_content(p: &mut Parser, stop_on_r_brace: bool) {
    while !(p.at(EOF) || p.at(T!['}']) && stop_on_r_brace) {
        process_top_level(p, stop_on_r_brace);
    }
}

// process either rule, import or include
pub(super) fn process_top_level(p: &mut Parser, stop_on_r_brace: bool) {
    let m = p.start();

    // Parse imports
    if p.at(IMPORT_KW) {
        p.bump(IMPORT_KW);
        p.expect(STRING_LIT);
        m.complete(p, IMPORT_STMT);
        return;
    }

    // Parse includes
    if p.at(INCLUDE_KW) {
        p.bump(INCLUDE_KW);
        p.expect(STRING_LIT);
        m.complete(p, INCLUDE_STMT);
        return;
    }

    // Parse rules
    let m = match opt_rule(p, m) {
        Ok(()) => {
            return;
        }
        Err(m) => m,
    };

    // On top level we can right now only have rules
    // So if rules are not successfully parsed, we can just abandon the marker
    // and either create an error block and try to parse it as a rule body and throw
    // an error or just throw an error
    m.abandon(p);
    match p.current() {
        T!['{'] => {
            error_block(p, "expected an import statement, include statement or a rule");
        }
        T!['}'] if !stop_on_r_brace => {
            let e = p.start();
            p.error("unmatched }");
            p.bump(T!['}']);
            e.complete(p, ERROR);
        }
        EOF | T!['}'] => p.error("invalid rule body"),
        _ => p.err_and_bump("expected an import statement, include statement or a rule"),
    }
}

// Parse rule
pub(super) fn opt_rule(p: &mut Parser, m: Marker) -> Result<(), Marker> {
    // add rule modifiers to match current and lookahead next with p.nth(1) for RULE or ERROR
    while p.at_ts(TokenSet::new(&[T![private], T![global]])) {
        let m = p.start();
        p.bump_any();
        m.complete(p, MODIFIER);
    }
    if p.at(T![rule]) {
        rule(p, m);
    } else {
        return Err(m);
    }
    Ok(())
}

// Parse a rule
// It consists of rule name [`IDENTIFIER`] and a body [`block_expr`]
fn rule(p: &mut Parser, m: Marker) {
    assert!(p.at(T![rule]));
    p.bump(T![rule]);
    if p.at(IDENTIFIER) {
        p.bump(IDENTIFIER);
    } else {
        p.err_recover("expected a name", RULE_RECOVERY_SET);
    }
    // add optional support for rule tags
    if p.at(T![:]) {
        p.bump(T![:]);
        while p.at(IDENTIFIER) {
            let m = p.start();
            p.bump(IDENTIFIER);
            m.complete(p, TAG);
        }
    }
    expressions::block_expr(p);
    m.complete(p, RULE);
}
