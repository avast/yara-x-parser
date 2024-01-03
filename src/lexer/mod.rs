use logos::Logos;
use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) enum LexingError {
    InvalidInteger(String),
    #[default]
    InvalidCharacter,
}

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexingError::InvalidInteger("overflow error".to_owned()),
            _ => LexingError::InvalidInteger("other error".to_owned()),
        }
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexingError)]
pub(crate) enum Token {
    // For now it is desired to support only small subset of YARA language
    // so just something like:
    // rule foo {
    //   strings:
    //     $a = "foo"
    //     $b = "bar"
    //   condition:
    //     $a and $b
    // }
    // will be supported

    // Keywords
    #[token("rule")]
    Rule,
    #[token("strings")]
    Strings,
    #[token("condition")]
    Condition,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,
    // Identifiers
    #[regex("[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    // Variables
    #[regex(r"\$[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Variable(String),
    // Strings
    #[regex(r#""[^"]*""#, |lex| lex.slice().to_string())]
    String(String),
    // Operators
    #[token("=")]
    Assign,
    #[token(":")]
    Colon,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(",")]
    Comma,
    // Numbers
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(i64),
    // Booleans
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Whitespace - I want to preserve whitespace tokens to implement full fidelity
    // and error resilience
    #[regex(r"[ \t\n\f]+")]
    Whitespace,

    // Comments
    #[regex(r"//.*")]
    Comment,
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    MultilineComment,
}
