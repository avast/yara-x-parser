//! Generated by `sourcegen_ast`, do not edit by hand.

#![allow(bad_style, missing_docs, unreachable_pub, clippy::upper_case_acronyms)]
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
    META_KW,
    PRIVATE_KW,
    GLOBAL_KW,
    IMPORT_KW,
    INCLUDE_KW,
    STRING_LIT,
    INT_LIT,
    FLOAT_LIT,
    IDENTIFIER,
    VARIABLE,
    WHITESPACE,
    COMMENT,
    ERROR,
    RULE,
    MODIFIER,
    TAG,
    STRINGS,
    META,
    CONDITION,
    SOURCE_FILE,
    BLOCK_EXPR,
    PREFIX_EXPR,
    LITERAL,
    EXPRESSION,
    EXPRESSION_STMT,
    VARIABLE_STMT,
    IMPORT_STMT,
    INCLUDE_STMT,
    META_STMT,
    PATTERN,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            AND_KW
                | OR_KW
                | NOT_KW
                | TRUE_KW
                | FALSE_KW
                | RULE_KW
                | STRINGS_KW
                | CONDITION_KW
                | META_KW
                | PRIVATE_KW
                | GLOBAL_KW
                | IMPORT_KW
                | INCLUDE_KW
        )
    }
    pub fn is_punct(self) -> bool {
        matches!(self, COLON | L_PAREN | R_PAREN | L_BRACE | R_BRACE | COMMA | ASSIGN)
    }
    pub fn is_literal(self) -> bool {
        matches!(self, STRING_LIT | INT_LIT | FLOAT_LIT)
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
            "meta" => META_KW,
            "private" => PRIVATE_KW,
            "global" => GLOBAL_KW,
            "import" => IMPORT_KW,
            "include" => INCLUDE_KW,
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
macro_rules ! T { [:] => { $ crate :: SyntaxKind :: COLON } ; ['('] => { $ crate :: SyntaxKind :: L_PAREN } ; [')'] => { $ crate :: SyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: SyntaxKind :: L_BRACE } ; ['}'] => { $ crate :: SyntaxKind :: R_BRACE } ; [,] => { $ crate :: SyntaxKind :: COMMA } ; [=] => { $ crate :: SyntaxKind :: ASSIGN } ; [and] => { $ crate :: SyntaxKind :: AND_KW } ; [or] => { $ crate :: SyntaxKind :: OR_KW } ; [not] => { $ crate :: SyntaxKind :: NOT_KW } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [rule] => { $ crate :: SyntaxKind :: RULE_KW } ; [strings] => { $ crate :: SyntaxKind :: STRINGS_KW } ; [condition] => { $ crate :: SyntaxKind :: CONDITION_KW } ; [meta] => { $ crate :: SyntaxKind :: META_KW } ; [private] => { $ crate :: SyntaxKind :: PRIVATE_KW } ; [global] => { $ crate :: SyntaxKind :: GLOBAL_KW } ; [import] => { $ crate :: SyntaxKind :: IMPORT_KW } ; [include] => { $ crate :: SyntaxKind :: INCLUDE_KW } ; [identifier] => { $ crate :: SyntaxKind :: IDENTIFIER } ; [variable] => { $ crate :: SyntaxKind :: VARIABLE } ; [string_lit] => { $ crate :: SyntaxKind :: STRING_LIT } ; [int_lit] => { $ crate :: SyntaxKind :: INT_LIT } ; [float_lit] => { $ crate :: SyntaxKind :: FLOAT_LIT } ; }
pub use T;
