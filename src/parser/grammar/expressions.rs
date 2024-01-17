use super::*;

pub(crate) fn block_expr(p: &mut Parser) {
    if !p.at(LBRACE) {
        p.error("expected a block expression");
        return;
    }
    let m = p.start();
    p.bump(LBRACE);
    rule_body(p);
    p.expect(RBRACE);
    m.complete(p, BLOCK_EXPR);
}

pub(super) fn rule_body(p: &mut Parser) {
    while !p.at(EOF) && !p.at(RBRACE) {
        match p.current() {
            // add metadata later
            STRINGS => strings(p),
            CONDITION => condition(p),
            _ => {
                p.err_and_bump("expected strings or condition");
            }
        }
    }
}

fn strings(p: &mut Parser) {
    assert!(p.at(STRINGS));
    let m = p.start();
    p.bump(STRINGS);
    p.expect(COLON);
    m.complete(p, STRINGS);
}

fn condition(p: &mut Parser) {
    assert!(p.at(CONDITION));
    let m = p.start();
    p.bump(CONDITION);
    p.expect(COLON);
    m.complete(p, CONDITION);
}
