//! This module contains lexer for YARA language.
//! The lexer is implemented using `logos` crate.
//! The lexer is used to convert the input text into a stream of tokens.
//!
//! Logos tokens are converted to `SyntaxKind` which is used in the parser to build the syntax tree.

use crate::{
    parser::syntax_kind::{SyntaxKind, T},
    syntax::syntax_error::SyntaxError,
};
use logos::Logos;
use std::fmt;
use text_size::{TextRange, TextSize};

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) enum LexingError {
    #[default]
    InvalidCharacter,
}

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexingError::InvalidCharacter => write!(f, "Invalid character"),
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
    #[token("import")]
    Import,
    #[token("include")]
    Include,
    #[token("rule")]
    Rule,
    #[token("private")]
    Private,
    #[token("global")]
    Global,
    #[token("meta")]
    Meta,
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
    #[regex(r"true|false", |lex| lex.slice().to_string())]
    Bool(String),
    #[token("contains")]
    Contains,
    #[token("icontains")]
    IContains,
    #[token("startswith")]
    StartsWith,
    #[token("istartswith")]
    IStartsWith,
    #[token("endswith")]
    EndsWith,
    #[token("iendswith")]
    IEndsWith,
    #[token("iequals")]
    IEquals,
    #[token("matches")]
    Matches,

    // Patterns
    #[regex(r"/(([^\\/\n])|(\\.))+/[a-zA-Z0-9]*", |lex| lex.slice().to_string())]
    Regexp(String),
    // Hexadecimal string
    #[regex(r"\{[0-9A-Fa-f?~()|\[\] -]*\}", |lex| lex.slice().to_string())]
    HexString(String),
    // Strings
    #[regex(r#""([^"\n\\]|\\["\\])*""#, |lex| lex.slice().to_string())]
    String(String),

    // Identifiers
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    // Variables
    #[regex(r"\$_?[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Variable(String),
    // Integer
    #[regex(r"0x[a-fA-F0-9]+|0o[0-7]+|[0-9]+(KB|MB)?", |lex| lex.slice().to_string())]
    Integer(String),
    // Float
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().to_string())]
    Float(String),

    // Modifiers
    #[token("ascii")]
    Ascii,
    #[token("wide")]
    Wide,
    #[token("nocase")]
    Nocase,
    #[token("fullword")]
    Fullword,
    #[token("xor")]
    Xor,
    #[token("base64")]
    Base64,
    #[token("base64wide")]
    Base64Wide,

    // Chars
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
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(",")]
    Comma,
    #[token("-")]
    Hyphen,
    #[token("|")]
    Pipe,
    #[token("~")]
    Tilde,
    #[token("?")]
    QuestionMark,
    #[token("+")]
    Plus,
    #[token("*")]
    Star,
    #[token("\\")]
    Backslash,
    #[token("%")]
    Percent,
    #[token("<<")]
    ShiftLeft,
    #[token(">>")]
    ShiftRight,
    #[token("&")]
    Ampersand,
    #[token("^")]
    Caret,
    #[token(".")]
    Dot,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanEqual,

    // Whitespace - I want to preserve whitespace tokens to implement full fidelity
    // and error resilience
    #[regex(r"[ \t\n\r]+")]
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
            Ok(token) => {
                if let LogosToken::HexString(hex_string) = token {
                    let detailed_tokens = process_hex_string_token(hex_string);
                    for (kind, len) in detailed_tokens {
                        tokens.push(Token { kind, len: TextSize::from(len as u32) });
                    }
                    continue;
                } else if let LogosToken::Regexp(regex) = token {
                    let detailed_tokens = process_regex_string_token(regex);
                    for (kind, len) in detailed_tokens {
                        tokens.push(Token { kind, len: TextSize::from(len as u32) });
                    }
                    continue;
                } else {
                    logos_tokenkind_to_syntaxkind(token)
                }
            }
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
        LogosToken::Import => SyntaxKind::IMPORT_KW,
        LogosToken::Include => SyntaxKind::INCLUDE_KW,
        LogosToken::Rule => SyntaxKind::RULE_KW,
        LogosToken::Private => SyntaxKind::PRIVATE_KW,
        LogosToken::Global => SyntaxKind::GLOBAL_KW,
        LogosToken::Meta => SyntaxKind::META_KW,
        LogosToken::Strings => SyntaxKind::STRINGS_KW,
        LogosToken::Condition => SyntaxKind::CONDITION_KW,
        LogosToken::And => SyntaxKind::AND_KW,
        LogosToken::Or => SyntaxKind::OR_KW,
        LogosToken::Not => SyntaxKind::NOT_KW,
        LogosToken::Contains => SyntaxKind::CONTAINS_KW,
        LogosToken::IContains => SyntaxKind::ICONTAINS_KW,
        LogosToken::StartsWith => SyntaxKind::STARTSWITH_KW,
        LogosToken::IStartsWith => SyntaxKind::ISTARTSWITH_KW,
        LogosToken::EndsWith => SyntaxKind::ENDSWITH_KW,
        LogosToken::IEndsWith => SyntaxKind::IENDSWITH_KW,
        LogosToken::IEquals => SyntaxKind::IEQUALS_KW,
        LogosToken::Matches => SyntaxKind::MATCHES_KW,
        LogosToken::Identifier(_) => SyntaxKind::IDENTIFIER,
        LogosToken::Variable(_) => SyntaxKind::VARIABLE,
        LogosToken::String(_) => SyntaxKind::STRING_LIT,
        LogosToken::Ascii => SyntaxKind::ASCII_KW,
        LogosToken::Wide => SyntaxKind::WIDE_KW,
        LogosToken::Nocase => SyntaxKind::NOCASE_KW,
        LogosToken::Fullword => SyntaxKind::FULLWORD_KW,
        LogosToken::Xor => SyntaxKind::XOR_KW,
        LogosToken::Base64 => SyntaxKind::BASE64_KW,
        LogosToken::Base64Wide => SyntaxKind::BASE64WIDE_KW,
        LogosToken::Assign => T![=],
        LogosToken::Colon => T![:],
        LogosToken::LBrace => T!['{'],
        LogosToken::RBrace => T!['}'],
        LogosToken::LParen => T!['('],
        LogosToken::RParen => T![')'],
        LogosToken::LBracket => T!['['],
        LogosToken::RBracket => T![']'],
        LogosToken::Comma => T![,],
        LogosToken::Hyphen => T![-],
        LogosToken::Pipe => T![|],
        LogosToken::Tilde => T![~],
        LogosToken::QuestionMark => T![?],
        LogosToken::Plus => T![+],
        LogosToken::Star => T![*],
        LogosToken::Backslash => T![backslash],
        LogosToken::Percent => T![%],
        LogosToken::ShiftLeft => T![<<],
        LogosToken::ShiftRight => T![>>],
        LogosToken::Ampersand => T![&],
        LogosToken::Caret => T![^],
        LogosToken::Dot => T![.],
        LogosToken::Equal => T![==],
        LogosToken::NotEqual => T![!=],
        LogosToken::LessThan => T![<],
        LogosToken::LessThanEqual => T![<=],
        LogosToken::GreaterThan => T![>],
        LogosToken::GreaterThanEqual => T![>=],
        LogosToken::Integer(_) => SyntaxKind::INT_LIT,
        LogosToken::Float(_) => SyntaxKind::FLOAT_LIT,
        LogosToken::Bool(_) => SyntaxKind::BOOL_LIT,
        LogosToken::Whitespace => SyntaxKind::WHITESPACE,
        LogosToken::Comment | LogosToken::MultilineComment => SyntaxKind::COMMENT,
        LogosToken::HexString(_) => {
            unreachable!("This should be handled in process_hex_string_token")
        }
        LogosToken::Regexp(_) => {
            unreachable!("This should be handled in process_regex_string_token")
        }
    }
}

fn process_regex_string_token(regex: String) -> Vec<(SyntaxKind, usize)> {
    let mut tokens = Vec::new();
    let mut chars = regex.chars().peekable();

    // Consume the first '/' character
    chars.next();
    tokens.push((SyntaxKind::SLASH, 1));

    // now store whole regex as a single token until next '/'
    let mut regex_str = String::new();
    let mut prev_char = None;
    for ch in chars.by_ref() {
        if ch == '/' && prev_char != Some('\\') {
            tokens.push((SyntaxKind::REGEX_LIT, regex_str.len()));
            tokens.push((SyntaxKind::SLASH, 1));
            break;
        } else {
            regex_str.push(ch);
            prev_char = Some(ch);
        }
    }

    // rest is handled as modifier token for each modifier
    // only valid modifiers are: 'i' for case insensitive and 's' for dot matches all
    for ch in chars.by_ref() {
        match ch {
            'i' => tokens.push((SyntaxKind::CASE_INSENSITIVE, 1)),
            's' => tokens.push((SyntaxKind::DOT_MATCHES_ALL, 1)),
            _ => {}
        }
    }

    tokens
}

// Process hexadecimal string token to generate detailed tokens
fn process_hex_string_token(hex_string: String) -> Vec<(SyntaxKind, usize)> {
    let mut tokens = Vec::new();
    let mut chars = hex_string.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            ' ' => tokens.push((SyntaxKind::WHITESPACE, 1)),
            '-' => tokens.push((SyntaxKind::HYPHEN, 1)),
            '{' => tokens.push((SyntaxKind::L_BRACE, 1)),
            '}' => tokens.push((SyntaxKind::R_BRACE, 1)),
            '(' => tokens.push((SyntaxKind::L_PAREN, 1)),
            ')' => tokens.push((SyntaxKind::R_PAREN, 1)),
            '[' => {
                tokens.push((SyntaxKind::L_BRACKET, 1));
                let mut num_str = String::new();
                while let Some(&peeked) = chars.peek() {
                    if peeked.is_ascii_digit() {
                        num_str.push(chars.next().unwrap());
                    } else if peeked == '-' {
                        if !num_str.is_empty() {
                            tokens.push((SyntaxKind::INT_LIT, num_str.len()));
                            num_str.clear();
                        }
                        tokens.push((SyntaxKind::HYPHEN, 1));
                        chars.next();
                    } else if peeked == ']' {
                        if !num_str.is_empty() {
                            tokens.push((SyntaxKind::INT_LIT, num_str.len()));
                            num_str.clear();
                        }
                        tokens.push((SyntaxKind::R_BRACKET, 1));
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            ']' => tokens.push((SyntaxKind::R_BRACKET, 1)),
            '|' => tokens.push((SyntaxKind::PIPE, 1)),
            '~' => {
                // Consume the next two characters to form the hex byte
                let mut len = 1;
                for _ in 0..2 {
                    if let Some(&peeked) = chars.peek() {
                        if peeked.is_ascii_hexdigit() || peeked == '?' {
                            chars.next();
                            len += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                tokens.push((SyntaxKind::HEX_LIT, len));
            }
            _ => {
                // If it's a hexadecimal character or '?', add as HexByte
                if ch.is_ascii_hexdigit() || ch == '?' {
                    let mut len = 1;
                    if let Some(&peeked) = chars.peek() {
                        if peeked.is_ascii_hexdigit() || peeked == '?' {
                            chars.next();
                            len += 1;
                        }
                    }
                    tokens.push((SyntaxKind::HEX_LIT, len));
                }
            }
        }
    }
    tokens
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
                    $b = §
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
        assert_eq!(tokens[14].kind, SyntaxKind::STRING_LIT);
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
