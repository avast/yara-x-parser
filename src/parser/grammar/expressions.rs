mod atom;

use super::*;

const PATTERN_MODIFIERS_SET: TokenSet = TokenSet::new(&[
    T![ascii],
    T![wide],
    T![private],
    T![fullword],
    T![nocase],
    T![xor],
    T![base64],
    T![base64wide],
]);

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
    let mut has_meta = false;

    while !p.at(EOF) && !p.at(T!['}']) {
        match p.current() {
            T![meta] => {
                if has_meta {
                    p.error("only one meta block is allowed");
                }
                if has_condition || has_strings {
                    p.error("meta block must come before strings and condition blocks");
                }
                meta(p);
                has_meta = true;
            }
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
                p.err_and_bump("expected meta, strings or condition keyword");
            }
        }
    }
}

/// Parse a `meta` block
/// It consists of `meta` keyword, `:` token and meta body
fn meta(p: &mut Parser) {
    assert!(p.at(T![meta]));
    let m = p.start();
    p.bump(T![meta]);
    p.expect(T![:]);
    meta_body(p);
    m.complete(p, META);
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

/// Parse a `meta` body
/// It consists of a list of `variable` and `=` token and a string
pub(super) fn meta_body(p: &mut Parser) {
    while !p.at(EOF) && !p.at(T![strings]) && !p.at(T![condition]) && !p.at(T!['}']) {
        let m = p.start();
        if p.at(T![identifier]) {
            p.bump(T![identifier]);
        } else {
            p.err_and_bump("expected an identifier");
        }
        p.expect(T![=]);
        match p.current() {
            STRING_LIT | BOOL_LIT | INT_LIT | FLOAT_LIT => {
                p.bump(p.current());
            }
            _ => {
                p.error("expected a valid metadata value");
                return;
            }
        }
        m.complete(p, META_STMT);
    }
}

/// Parse a `strings` body
/// It consists of a list of `variable` and `=` token and a string
pub(super) fn strings_body(p: &mut Parser) {
    while !p.at(EOF) && !p.at(T![condition]) && !p.at(T!['}']) {
        let m = p.start();

        if p.at(T![variable]) {
            p.bump(T![variable]);
        } else {
            p.err_and_bump("expected a variable");
        }
        p.expect(T![=]);

        // so far only strings are supported, later add match for hex strings and regex
        let n = p.start();
        match p.current() {
            STRING_LIT => p.bump(STRING_LIT),
            L_BRACE => hex_pattern(p),
            SLASH => regex_pattern(p),
            _ => {
                p.err_and_bump("expected a valid string");
                while !p.at(T!['}']) {
                    p.bump_any();
                }
                p.bump_any();
            }
        }
        if p.at_ts(PATTERN_MODIFIERS_SET) {
            string_modifiers(p);
        }
        n.complete(p, PATTERN);

        m.complete(p, VARIABLE_STMT);
    }
}

/// Parse a regex pattern
fn regex_pattern(p: &mut Parser) {
    let m = p.start();

    // Parse a regex pattern that starts with `/` and ends with `/`
    p.expect(SLASH);
    p.expect(REGEX_LIT);
    p.expect(SLASH);

    // after regex pattern there can be some regex specific modifiers
    while p.at(CASE_INSENSITIVE) || p.at(DOT_MATCHES_ALL) {
        let n = p.start();
        p.bump_any();
        n.complete(p, REGEX_MOD);
    }
    m.complete(p, REGEX_PATTERN);
}

/// Parse a hex string pattern
fn hex_pattern(p: &mut Parser) {
    let m = p.start();
    p.expect(T!['{']);
    if !p.at(T!['}']) {
        hex_tokens(p);
    }
    p.expect(T!['}']);
    m.complete(p, HEX_PATTERN);
}

/// Parse a hex tokens
fn hex_tokens(p: &mut Parser) {
    let m = p.start();
    hex_byte_or_alternative(p);
    while !p.at(EOF) && !p.at(T!['}']) && !p.at(T![|]) && !p.at(T![')']) {
        let n = p.start();
        hex_jump(p);
        hex_byte_or_alternative(p);
        n.complete(p, HEX_TOKEN_TAIL);
    }
    m.complete(p, HEX_TOKEN);
}

/// Parse a hex byte or alternative
fn hex_byte_or_alternative(p: &mut Parser) {
    if p.at(HEX_LIT) {
        hex_byte(p);
    } else if p.at(T!['(']) {
        hex_alternative(p);
    } else {
        p.err_and_bump("expected a hex byte or alternative");
    }
}

/// Parse a hex byte
/// It can be a hex literal, wildcard, integer, identifier or tilde
fn hex_byte(p: &mut Parser) {
    let m = p.start();
    p.bump(HEX_LIT);
    m.complete(p, HEX_BYTE);
}

/// Parse a hex jump
/// It consists of an integer range
fn hex_jump(p: &mut Parser) {
    while p.at(T!['[']) {
        let m = p.start();
        p.expect(T!['[']);
        int_range(p);
        p.expect(T![']']);
        m.complete(p, HEX_JUMP);
    }
}

/// Parse a hex alternative
/// It consists of hex tokens separated by `|`
fn hex_alternative(p: &mut Parser) {
    let m = p.start();
    p.expect(T!['(']);
    hex_tokens(p);
    while p.at(T![|]) {
        let n = p.start();
        p.bump(T![|]);
        hex_tokens(p);
        n.complete(p, HEX_PIPE);
    }
    p.expect(T![')']);
    m.complete(p, HEX_ALTERNATIVE);
}

/// Parse string modifiers
fn string_modifiers(p: &mut Parser) {
    while p.at_ts(PATTERN_MODIFIERS_SET) {
        let m = p.start();
        if p.current() == T![base64] || p.current() == T![base64wide] {
            p.bump_any();
            if p.at(T!['(']) {
                base64_body(p);
            }
        } else if p.current() == T![xor] {
            p.bump_any();
            if p.at(T!['(']) {
                xor_body(p);
            }
        } else {
            p.bump_any();
        }
        m.complete(p, PATTERN_MOD);
    }
}

/// Parse a base64 string pattern
fn base64_body(p: &mut Parser) {
    let m = p.start();
    p.expect(T!['(']);
    p.expect(STRING_LIT);
    p.expect(T![')']);
    m.complete(p, BASE_ALPHABET);
}

/// Parse a xor range pattern
fn xor_body(p: &mut Parser) {
    let m = p.start();
    p.expect(T!['(']);
    int_range(p);
    p.expect(T![')']);
    m.complete(p, XOR_RANGE);
}

/// Parse an integer range
/// used in xor and hex jumps
fn int_range(p: &mut Parser) {
    // parse LHS of range
    p.expect(INT_LIT);
    if p.at(HYPHEN) {
        p.bump(HYPHEN);
        // Parse RHS of range
        p.expect(INT_LIT);
    }
}

/// Parse a `condition` body
/// It consists of a list of expressions
/// Pratt parser is used to parse expressions
pub(super) fn condition_body(p: &mut Parser) {
    boolean_expr(p, None, 1);
}

enum Associativity {
    Left,
    Right,
}

/// Binding powers of operators for a Pratt parser.
fn current_op(p: &mut Parser) -> (u8, SyntaxKind, Associativity) {
    match p.current() {
        T![and] => (4, T![and], Associativity::Left),
        T![or] => (2, T![or], Associativity::Left),
        _ => (0, ERROR, Associativity::Left),
    }
}

fn expr_op(p: &mut Parser) -> (u8, SyntaxKind, Associativity) {
    match p.current() {
        T![|] => (10, T![|], Associativity::Left),
        T![^] => (12, T![^], Associativity::Left),
        T![&] => (14, T![&], Associativity::Left),
        T![<<] => (16, T![<<], Associativity::Left),
        T![>>] => (16, T![>>], Associativity::Left),
        T![+] => (18, T![+], Associativity::Left),
        T![-] => (18, T![-], Associativity::Left),
        T![*] => (20, T![*], Associativity::Left),
        T![backslash] => (20, T![backslash], Associativity::Left),
        T![%] => (20, T![%], Associativity::Left),
        T![.] => (22, T![.], Associativity::Left),
        _ => (0, ERROR, Associativity::Left),
    }
}

fn expr_stmt_op(p: &mut Parser) -> (u8, SyntaxKind, Associativity) {
    match p.current() {
        T![==] => (6, T![==], Associativity::Left),
        T![!=] => (6, T![!=], Associativity::Left),
        T![contains] => (6, T![contains], Associativity::Left),
        T![icontains] => (6, T![icontains], Associativity::Left),
        T![startswith] => (6, T![startswith], Associativity::Left),
        T![istartswith] => (6, T![istartswith], Associativity::Left),
        T![endswith] => (6, T![endswith], Associativity::Left),
        T![iendswith] => (6, T![iendswith], Associativity::Left),
        T![iequals] => (6, T![iequals], Associativity::Left),
        T![matches] => (6, T![matches], Associativity::Left),
        T![<] => (8, T![<], Associativity::Left),
        T![<=] => (8, T![<=], Associativity::Left),
        T![>] => (8, T![>], Associativity::Left),
        T![>=] => (8, T![>=], Associativity::Left),
        _ => (0, ERROR, Associativity::Left),
    }
}

/// Parse an expression using a Pratt parser.
///
/// Expression can be binary, unary or literal
/// This is also used to reflect operator precedence and associativity
/// It is inspired by Pratt parser used in rust-analyter
/// <https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html>
fn boolean_expr(p: &mut Parser, m: Option<Marker>, bp: u8) -> Option<CompletedMarker> {
    let m = m.unwrap_or_else(|| p.start());
    let mut lhs = match boolean_term(p) {
        Some(lhs) => lhs.extend_to(p, m),
        None => {
            m.abandon(p);
            return None;
        }
    };

    loop {
        let (op_bp, op, associativity) = current_op(p);
        if op_bp <= bp {
            break;
        }
        let m = lhs.precede(p);
        p.bump(op);

        let op_bp = match associativity {
            Associativity::Left => op_bp + 1,
            Associativity::Right => op_bp,
        };
        boolean_expr(p, None, op_bp);
        lhs = m.complete(p, BOOLEAN_EXPR);
    }
    Some(lhs)
}

fn boolean_term(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    if p.at(T![not]) {
        p.bump(T![not]);
        boolean_term(p);
    } else if p.at(T!['(']) {
        p.bump(T!['(']);
        boolean_expr(p, None, 1);
        p.bump(T![')']);
    } else if p.at(T![variable]) {
        p.bump(T![variable]);
    } else if p.at(T![bool_lit]) {
        p.bump(T![bool_lit]);
    } else if p.at(T![defined]) {
        p.bump(T![defined]);
        boolean_term(p);
    } else {
        expr_stmt(p, None, 1);
    }
    let cm = m.complete(p, BOOLEAN_TERM);
    Some(cm)
}

fn expr_stmt(p: &mut Parser, m: Option<Marker>, bp: u8) -> Option<CompletedMarker> {
    let m = m.unwrap_or_else(|| p.start());
    let mut lhs = match expr(p, None, bp) {
        Some(lhs) => lhs.extend_to(p, m),
        None => {
            m.abandon(p);
            return None;
        }
    };

    loop {
        let (op_bp, op, associativity) = expr_stmt_op(p);
        if op_bp < bp {
            break;
        }
        let m = lhs.precede(p);
        p.bump(op);

        let op_bp = match associativity {
            Associativity::Left => op_bp + 1,
            Associativity::Right => op_bp,
        };
        expr_stmt(p, None, op_bp);
        lhs = m.complete(p, EXPRESSION);
    }
    Some(lhs)
}

fn expr(p: &mut Parser, m: Option<Marker>, bp: u8) -> Option<CompletedMarker> {
    let m = m.unwrap_or_else(|| p.start());
    let mut lhs = match term(p) {
        Some(lhs) => lhs.extend_to(p, m),
        None => {
            m.abandon(p);
            return None;
        }
    };

    loop {
        let (op_bp, op, associativity) = expr_op(p);
        if op_bp < bp {
            break;
        }
        let m = lhs.precede(p);
        p.bump(op);

        let op_bp = match associativity {
            Associativity::Left => op_bp + 1,
            Associativity::Right => op_bp,
        };
        expr(p, None, op_bp);
        lhs = m.complete(p, EXPRESSION);
    }
    Some(lhs)
}

fn term(p: &mut Parser) -> Option<CompletedMarker> {
    primary_expr(p)
}

fn primary_expr(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    match p.current() {
        T![float_lit] => {
            p.bump(T![float_lit]);
        }
        T![int_lit] => {
            p.bump(T![int_lit]);
        }
        T![string_lit] => {
            p.bump(T![string_lit]);
        }
        T![identifier] => {
            p.bump(T![identifier]);
        }
        T![filesize] => {
            p.bump(T![filesize]);
        }
        T![entrypoint] => {
            p.bump(T![entrypoint]);
        }
        T![-] => {
            p.bump(T![-]);
            term(p);
        }
        T!['('] => {
            p.bump(T!['(']);
            expr(p, None, 1);
            p.bump(T![')']);
        }
        _ => {
            m.abandon(p);
            return None;
        }
    };
    let cm = m.complete(p, PRIMARY_EXPR);
    Some(cm)
}
