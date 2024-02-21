mod atom;

use super::*;

/// Recovery set for `strings` block. This also should be adjusted and tweaked to
/// better represents recovery set later on
const VARIABLE_RECOVERY_SET: TokenSet = TokenSet::new(&[T![strings]]);

/// Parse a rule body
/// A rule body consists `{`, rule_body and `}`
/// This can probably be later simplified to not have both
/// `rule_body` and `block_expr`.
pub(crate) fn block_expr(p: &mut Parser) {
    if !p.at(T!['{']) {
        p.error("expected a block expression");
        return;
    }
    let m = p.start();
    p.bump(T!['{']);
    rule_body(p);
    p.expect(T!['}']);
    m.complete(p, BLOCK_EXPR);
}

/// Parse a rule body
/// A rule body consists of `strings` and `condition` blocks
/// `strings` part is optional but condition is required
/// but each of them can be defined only once and have to be in order
pub(super) fn rule_body(p: &mut Parser) {
    let mut has_strings = false;
    let mut has_condition = false;
    while !p.at(EOF) && !p.at(T!['}']) {
        match p.current() {
            // add metadata support later
            T![strings] => {
                if has_strings {
                    p.error("only one strings block is allowed");
                }
                if has_condition {
                    p.error("strings block must come before condition block");
                }
                strings(p);
                has_strings = true;
            }
            T![condition] => {
                if has_condition {
                    p.error("only one condition block is allowed");
                }
                condition(p);
                has_condition = true;
            }
            _ => {
                // It did not contain strings or condition in valid form
                // but we can still try to parse their body and throw an error for parent
                // for now it just looks at next 2 tokens to differenciate between valid strings
                // body or condition body. This should probably be adjusted later
                p.err_and_bump("expected strings or condition");
                if p.current() == T![:] {
                    p.eat(T![:]);
                    if p.current() == T![variable] && p.nth(1) == T![=] {
                        strings_body(p)
                    } else {
                        condition_body(p);
                    }
                }
            }
        }
    }
}

/// Parse a `strings` block
/// It consists of `strings` keyword,`:` token and strings body
fn strings(p: &mut Parser) {
    assert!(p.at(T![strings]));
    let m = p.start();
    p.bump(T![strings]);
    p.expect(T![:]);
    strings_body(p);
    m.complete(p, STRINGS);
}

/// Parse a `condition` block
/// It consists of `condition` keyword,`:` token and condition body
fn condition(p: &mut Parser) {
    assert!(p.at(T![condition]));
    let m = p.start();
    p.bump(T![condition]);
    p.expect(T![:]);
    condition_body(p);
    m.complete(p, CONDITION);
}

/// Parse a `strings` body
/// It consists of a list of `variable` and `=` token and a string
pub(super) fn strings_body(p: &mut Parser) {
    // add support for meta also
    while !p.at(EOF) && !p.at(T![condition]) && !p.at(T!['}']) {
        let m = p.start();
        if p.at(T![variable]) {
            p.bump(T![variable]);
        } else {
            p.err_recover("expected a variable", VARIABLE_RECOVERY_SET);
        }
        p.expect(T![=]);
        // so far only strings are supported, later add match for hex strings and regex
        pattern(p);
        m.complete(p, VARIABLE_STMT);
    }
}

/// Parse a string. For now string can be only basic plaintext string
// add support for hex and regex strings later on
fn pattern(p: &mut Parser) {
    let m = p.start();
    match p.current() {
        STRING_LIT => p.bump(STRING_LIT),
        _ => p.err_and_bump("expected a string"),
    }
    // add string modifiers
    m.complete(p, PATTERN);
}

/// Parse a `condition` body
/// It consists of a list of expressions
/// Pratt parser is used to parse expressions
pub(super) fn condition_body(p: &mut Parser) {
    // add support for meta also
    while !p.at(EOF) && !p.at(T!['}']) {
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
        T![and] => (4, T![and], Associativity::Left),
        T![or] => (3, T![or], Associativity::Left),
        _ => (0, ERROR, Associativity::Left),
    }
}

/// Parse an expression using a Pratt parser.
///
/// Expression can be binary, unary or literal
/// This is also used to reflect operator precedence and associativity
/// It is inspired by Pratt parser used in rust-analyter
/// <https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html>
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

/// Left hand side of an expression.
fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let m;
    let kind = match p.current() {
        // unary operators
        T![not] => {
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
