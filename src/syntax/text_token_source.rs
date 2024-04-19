use crate::{
    lexer::Token,
    parser::{self, SyntaxKind::EOF, TokenSource},
};
use text_size::{TextRange, TextSize};

/// A source of tokens for the parser.
/// It takes tokens from a source text and store them into token-offset pairs
pub(crate) struct TextTokenSource<'t> {
    text: &'t str,

    token_offset_pairs: Vec<(Token, TextSize)>,

    curr: (parser::Token, usize),
}

impl<'t> TokenSource for TextTokenSource<'t> {
    /// Returns the current token
    fn current(&self) -> parser::Token {
        self.curr.0
    }

    /// Lookahead `n` tokens
    fn lookahead_nth(&self, n: usize) -> parser::Token {
        mk_token(self.curr.1 + n, &self.token_offset_pairs)
    }

    /// Bumps the current token
    fn bump(&mut self) {
        if self.curr.0.kind == EOF {
            return;
        }

        let pos = self.curr.1 + 1;
        self.curr = (mk_token(pos, &self.token_offset_pairs), pos);
    }

    /// Check if the current token is specific `SyntaxKind` kind
    fn is_keyword(&self, kw: &str) -> bool {
        self.token_offset_pairs
            .get(self.curr.1)
            .map(|(token, offset)| &self.text[TextRange::at(*offset, token.len)] == kw)
            .unwrap_or(false)
    }
}

/// Create a token from a position
fn mk_token(pos: usize, token_offset_pairs: &[(Token, TextSize)]) -> parser::Token {
    let (kind, is_jointed_to_next) = match token_offset_pairs.get(pos) {
        Some((token, offset)) => (
            token.kind,
            token_offset_pairs
                .get(pos + 1)
                .map(|(_, next_offset)| offset + token.len == *next_offset)
                .unwrap_or(false),
        ),
        None => (EOF, false),
    };
    parser::Token { kind, is_jointed_to_next }
}

/// Generate token-offset pairs
impl<'t> TextTokenSource<'t> {
    pub(crate) fn new(text: &'t str, raw_tokens: &'t [Token]) -> TextTokenSource<'t> {
        let token_offset_pairs: Vec<_> = raw_tokens
            .iter()
            .filter_map({
                let mut len = 0.into();
                move |token| {
                    let pair = if token.kind.is_trivia() { None } else { Some((*token, len)) };
                    len += token.len;
                    pair
                }
            })
            .collect();

        let first = mk_token(0, &token_offset_pairs);
        TextTokenSource { text, token_offset_pairs, curr: (first, 0) }
    }
}
