mod atom;

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
    strings_body(p);
    m.complete(p, STRINGS);
}

fn condition(p: &mut Parser) {
    assert!(p.at(CONDITION));
    let m = p.start();
    p.bump(CONDITION);
    p.expect(COLON);
    condition_body(p);
    m.complete(p, CONDITION);
}

pub(super) fn strings_body(p: &mut Parser) {
    // add support for meta also
    while !p.at(EOF) && !p.at(STRINGS) && !p.at(CONDITION) && !p.at(RBRACE) {
        assert!(p.at(VARIABLE));
        let m = p.start();
        p.bump(VARIABLE);
        p.expect(ASSIGN);
        // so far only strings are supported, later add match for hex strings and regex
        string(p);
        m.complete(p, VARIABLE);
    }
}

// do the same for hex and regex strings
fn string(p: &mut Parser) {
    assert!(p.at(STRING));
    let m = p.start();
    p.bump(STRING);
    // add plain string modifiers
    m.complete(p, STRING);
}

pub(super) fn condition_body(p: &mut Parser) {
    // add support for meta also
    while !p.at(EOF) && !p.at(STRINGS) && !p.at(CONDITION) && !p.at(RBRACE) {
        let m = p.start();
        if let Some(cm) = expression(p, Some(m), 1) {
            let m = cm.precede(p);
            m.complete(p, EXPRESSION_STMT);
        }
    }
}

enum Associativity {
    Left,
    Right,
}

/// Binding powers of operators for a Pratt parser.
fn current_op(p: &mut Parser) -> (u8, SyntaxKind, Associativity) {
    match p.current() {
        // add support for other operators
        AND => (4, AND, Associativity::Left),
        OR => (3, OR, Associativity::Left),
        _ => (0, ERROR, Associativity::Left),
    }
}

fn expression(p: &mut Parser, m: Option<Marker>, bp: u8) -> Option<CompletedMarker> {
    let m = m.unwrap_or_else(|| p.start());
    let mut lhs = match lhs(p) {
        Some(lhs) => lhs.extend_to(p, m),
        None => {
            m.abandon(p);
            return None;
        }
    };

    loop {
        let (op_bp, op, associativity) = current_op(p);
        if op_bp < bp {
            break;
        }
        let m = lhs.precede(p);
        p.bump(op);

        let op_bp = match associativity {
            Associativity::Left => op_bp + 1,
            Associativity::Right => op_bp,
        };
        expression(p, None, op_bp);
        lhs = m.complete(p, EXPRESSION);
    }
    Some(lhs)
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let m;
    let kind = match p.current() {
        // unary operators
        NOT => {
            m = p.start();
            p.bump_any();
            PREFIX_EXPR
        }
        // all other operators
        _ => {
            let lhs = atom::atom_expr(p)?;
            return Some(lhs);
        }
    };
    // parse unary operators interior
    expression(p, None, 255);
    let cm = m.complete(p, kind);
    Some(cm)
}
