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
    let mut has_strings = false;
    let mut has_condition = false;
    while !p.at(EOF) && !p.at(RBRACE) {
        match p.current() {
            // add metadata later
            STRINGS => {
                if has_strings {
                    p.error("only one strings block is allowed");
                }
                if has_condition {
                    p.error("strings block must come before condition block");
                }
                strings(p);
                has_strings = true;
            }
            CONDITION => {
                if has_condition {
                    p.error("only one condition block is allowed");
                }
                condition(p);
                has_condition = true;
            }
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

const VARIABLE_RECOVERY_SET: TokenSet = TokenSet::new(&[VARIABLE]);

pub(super) fn strings_body(p: &mut Parser) {
    // add support for meta also
    while !p.at(EOF) && !p.at(STRINGS) && !p.at(CONDITION) && !p.at(RBRACE) {
        let m = p.start();
        if p.at(VARIABLE) {
            let m = p.start();
            p.bump(VARIABLE);
            m.complete(p, VARIABLE);
        } else {
            p.err_recover("expected a variable", VARIABLE_RECOVERY_SET);
        }
        p.expect(ASSIGN);
        // so far only strings are supported, later add match for hex strings and regex
        string(p);
        m.complete(p, VARIABLE_STMT);
    }
}

// add support for hex and regex strings later on
fn string(p: &mut Parser) {
    let m = p.start();
    match p.current() {
        STRING => p.bump(STRING),
        _ => p.err_and_bump("expected a string"),
    }
    // add string modifiers
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
