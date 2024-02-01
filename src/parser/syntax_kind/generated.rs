//! Generated by `sourcegen_ast`, do not edit by hand.

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENTIFIER`, `RULE_KW`, or `AND`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    COLON,
    L_PAREN,
    R_PAREN,
    L_BRACE,
    R_BRACE,
    COMMA,
    ASSIGN,
    AND_KW,
    OR_KW,
    NOT_KW,
    TRUE_KW,
    FALSE_KW,
    RULE_KW,
    STRINGS_KW,
    CONDITION_KW,
    STRING,
    NUMBER,
    IDENTIFIER,
    VARIABLE,
    WHITESPACE,
    COMMENT,
    ERROR,
    RULE,
    STRINGS,
    CONDITION,
    SOURCE_FILE,
    BLOCK_EXPR,
    PREFIX_EXPR,
    LITERAL,
    EXPRESSION,
    EXPRESSION_STMT,
    VARIABLE_STMT,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        match self {
            AND_KW | OR_KW | NOT_KW | TRUE_KW | FALSE_KW | RULE_KW | STRINGS_KW | CONDITION_KW => {
                true
            }
            _ => false,
        }
    }
    pub fn is_punct(self) -> bool {
        match self {
            COLON | L_PAREN | R_PAREN | L_BRACE | R_BRACE | COMMA | ASSIGN => true,
            _ => false,
        }
    }
    pub fn is_literal(self) -> bool {
        match self {
            STRING | NUMBER => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "and" => AND_KW,
            "or" => OR_KW,
            "not" => NOT_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            "rule" => RULE_KW,
            "strings" => STRINGS_KW,
            "condition" => CONDITION_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            ':' => COLON,
            '(' => L_PAREN,
            ')' => R_PAREN,
            '{' => L_BRACE,
            '}' => R_BRACE,
            ',' => COMMA,
            '=' => ASSIGN,
            _ => return None,
        };
        Some(tok)
    }
}
#[macro_export]
macro_rules ! T { [:] => { $ crate :: SyntaxKind :: COLON } ; ['('] => { $ crate :: SyntaxKind :: L_PAREN } ; [')'] => { $ crate :: SyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: SyntaxKind :: L_BRACE } ; ['}'] => { $ crate :: SyntaxKind :: R_BRACE } ; [,] => { $ crate :: SyntaxKind :: COMMA } ; [=] => { $ crate :: SyntaxKind :: ASSIGN } ; [and] => { $ crate :: SyntaxKind :: AND_KW } ; [or] => { $ crate :: SyntaxKind :: OR_KW } ; [not] => { $ crate :: SyntaxKind :: NOT_KW } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [rule] => { $ crate :: SyntaxKind :: RULE_KW } ; [strings] => { $ crate :: SyntaxKind :: STRINGS_KW } ; [condition] => { $ crate :: SyntaxKind :: CONDITION_KW } ; [identifier] => { $ crate :: SyntaxKind :: IDENTIFIER } ; [variable] => { $ crate :: SyntaxKind :: VARIABLE } ; }
pub use T;
