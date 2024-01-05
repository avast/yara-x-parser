use crate::{parser::syntaxkind::SyntaxKind, syntax::syntax_error::SyntaxError};
use logos::Logos;
use std::fmt;
use std::num::ParseIntError;
use text_size::{TextRange, TextSize};

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) enum LexingError {
    InvalidInteger(String),
    #[default]
    InvalidCharacter,
}

// Implement Display trait for LexingError
impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexingError::InvalidInteger(msg) => write!(f, "Invalid integer: {}", msg),
            LexingError::InvalidCharacter => write!(f, "Invalid character"),
        }
    }
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
pub(crate) enum LogosToken {
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

/// A token of Rust source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    /// The kind of token.
    pub kind: SyntaxKind,
    /// The length of the token.
    pub len: TextSize,
}

pub fn tokenize(text: &str) -> (Vec<Token>, Vec<SyntaxError>) {
    if text.is_empty() {
        return Default::default();
    }

    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut offset = 0;

    let logos_tokens: Vec<_> = LogosToken::lexer(&text).spanned().collect();

    // Loop over all tokens, convert them to syntaxkind and push them into tokens vector
    // also push errors into errors vector
    for (token, range) in logos_tokens {
        let token_len = range.len().try_into().unwrap();
        let token_range = TextRange::at(offset.try_into().unwrap(), token_len);
        let syntaxkind;
        match token {
            Ok(token) => {
                syntaxkind = logos_tokenkind_to_syntaxkind(token);
            }
            Err(err) => {
                errors.push(SyntaxError::new(err.to_string(), token_range));
                syntaxkind = SyntaxKind::ERROR;
            }
        }
        tokens.push(Token {
            kind: syntaxkind,
            len: token_len,
        });
        offset += range.len();
    }

    // Add EOF token at the end
    tokens.push(Token {
        kind: SyntaxKind::EOF,
        len: 0.into(),
    });

    (tokens, errors)
}

// Convert LogosToken to SyntaxKind
fn logos_tokenkind_to_syntaxkind(token: LogosToken) -> SyntaxKind {
    match token {
        LogosToken::Rule => SyntaxKind::RULE,
        LogosToken::Strings => SyntaxKind::STRINGS,
        LogosToken::Condition => SyntaxKind::CONDITION,
        LogosToken::And => SyntaxKind::AND,
        LogosToken::Or => SyntaxKind::OR,
        LogosToken::Not => SyntaxKind::NOT,
        LogosToken::Identifier(_) => SyntaxKind::IDENTIFIER,
        LogosToken::Variable(_) => SyntaxKind::VARIABLE,
        LogosToken::String(_) => SyntaxKind::STRING,
        LogosToken::Assign => SyntaxKind::ASSIGN,
        LogosToken::Colon => SyntaxKind::COLON,
        LogosToken::LBrace => SyntaxKind::LBRACE,
        LogosToken::RBrace => SyntaxKind::RBRACE,
        LogosToken::LParen => SyntaxKind::LPAREN,
        LogosToken::RParen => SyntaxKind::RPAREN,
        LogosToken::Comma => SyntaxKind::COMMA,
        LogosToken::Number(_) => SyntaxKind::NUMBER,
        LogosToken::True => SyntaxKind::TRUE,
        LogosToken::False => SyntaxKind::FALSE,
        LogosToken::Whitespace => SyntaxKind::WHITESPACE,
        LogosToken::Comment => SyntaxKind::COMMENT,
        LogosToken::MultilineComment => SyntaxKind::MULTILINECOMMENT,
    }
}
