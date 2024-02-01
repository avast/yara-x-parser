use crate::{
    parser::syntax_kind::{SyntaxKind, T},
    syntax::syntax_error::SyntaxError,
};
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

    let logos_tokens: Vec<_> = LogosToken::lexer(text).spanned().collect();

    // Loop over all tokens, convert them to syntaxkind and push them into tokens vector
    // also push errors into errors vector
    for (token, range) in logos_tokens {
        let token_len = range.len().try_into().unwrap();
        let token_range = TextRange::at(offset.try_into().unwrap(), token_len);
        let syntaxkind = match token {
            Ok(token) => logos_tokenkind_to_syntaxkind(token),
            Err(err) => {
                errors.push(SyntaxError::new(err.to_string(), token_range));
                SyntaxKind::ERROR
            }
        };
        tokens.push(Token { kind: syntaxkind, len: token_len });
        offset += range.len();
    }

    // Add EOF token at the end
    tokens.push(Token { kind: SyntaxKind::EOF, len: 0.into() });

    (tokens, errors)
}

// Convert LogosToken to SyntaxKind
fn logos_tokenkind_to_syntaxkind(token: LogosToken) -> SyntaxKind {
    match token {
        LogosToken::Rule => SyntaxKind::RULE_KW,
        LogosToken::Strings => SyntaxKind::STRINGS_KW,
        LogosToken::Condition => SyntaxKind::CONDITION_KW,
        LogosToken::And => SyntaxKind::AND_KW,
        LogosToken::Or => SyntaxKind::OR_KW,
        LogosToken::Not => SyntaxKind::NOT_KW,
        LogosToken::Identifier(_) => SyntaxKind::IDENTIFIER,
        LogosToken::Variable(_) => SyntaxKind::VARIABLE,
        LogosToken::String(_) => SyntaxKind::STRING,
        LogosToken::Assign => T![=],
        LogosToken::Colon => T![:],
        LogosToken::LBrace => T!['{'],
        LogosToken::RBrace => T!['}'],
        LogosToken::LParen => T!['('],
        LogosToken::RParen => T![')'],
        LogosToken::Comma => T![,],
        LogosToken::Number(_) => SyntaxKind::NUMBER,
        LogosToken::True => SyntaxKind::TRUE_KW,
        LogosToken::False => SyntaxKind::FALSE_KW,
        LogosToken::Whitespace => SyntaxKind::WHITESPACE,
        LogosToken::Comment | LogosToken::MultilineComment => SyntaxKind::COMMENT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_empty() {
        let input = "";
        let (tokens, errors) = tokenize(input);
        assert!(errors.is_empty());
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_tokenize_whitespace() {
        let input = " ";
        let (tokens, errors) = tokenize(input);
        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[1].kind, SyntaxKind::EOF);
    }

    #[test]
    fn test_tokenize_rule() {
        let input = r#"
            rule foo {
                condition:
                    $a
            }
        "#;
        let (tokens, errors) = tokenize(input);
        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 15);
        assert_eq!(tokens[0].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[1].kind, SyntaxKind::RULE_KW);
        assert_eq!(tokens[2].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[3].kind, SyntaxKind::IDENTIFIER);
        assert_eq!(tokens[4].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[5].kind, SyntaxKind::L_BRACE);
        assert_eq!(tokens[6].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[7].kind, SyntaxKind::CONDITION_KW);
        assert_eq!(tokens[8].kind, SyntaxKind::COLON);
        assert_eq!(tokens[9].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[10].kind, SyntaxKind::VARIABLE);
        assert_eq!(tokens[11].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[12].kind, SyntaxKind::R_BRACE);
        assert_eq!(tokens[13].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[14].kind, SyntaxKind::EOF);
    }

    #[test]
    fn tokenize_error() {
        let input = r#"
            rule foo {
                condition:
                    $a = "test"
                    $b = 1234567890123456789012345678901234567890
            }
        "#;
        let (tokens, errors) = tokenize(input);
        assert_eq!(errors.len(), 1);
        assert_eq!(tokens.len(), 25);
        assert_eq!(tokens[0].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[1].kind, SyntaxKind::RULE_KW);
        assert_eq!(tokens[2].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[3].kind, SyntaxKind::IDENTIFIER);
        assert_eq!(tokens[4].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[5].kind, SyntaxKind::L_BRACE);
        assert_eq!(tokens[6].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[7].kind, SyntaxKind::CONDITION_KW);
        assert_eq!(tokens[8].kind, SyntaxKind::COLON);
        assert_eq!(tokens[9].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[10].kind, SyntaxKind::VARIABLE);
        assert_eq!(tokens[11].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[12].kind, SyntaxKind::ASSIGN);
        assert_eq!(tokens[13].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[14].kind, SyntaxKind::STRING);
        assert_eq!(tokens[15].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[16].kind, SyntaxKind::VARIABLE);
        assert_eq!(tokens[17].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[18].kind, SyntaxKind::ASSIGN);
        assert_eq!(tokens[19].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[20].kind, SyntaxKind::ERROR);
        assert_eq!(tokens[21].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[22].kind, SyntaxKind::R_BRACE);
        assert_eq!(tokens[23].kind, SyntaxKind::WHITESPACE);
        assert_eq!(tokens[24].kind, SyntaxKind::EOF);
    }
}
