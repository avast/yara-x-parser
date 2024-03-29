//! A bit set for `SyntaxKind`
//! This can be used to create a new set of tokens
//! mainly for Error recovery FIRST/FOLLOW sets

use crate::parser::SyntaxKind;

#[derive(Clone, Copy)]
pub(crate) struct TokenSet(u128);

impl TokenSet {
    pub(crate) const EMPTY: TokenSet = TokenSet(0);

    pub(crate) const fn new(kinds: &[SyntaxKind]) -> TokenSet {
        let mut res = 0u128;
        let mut i = 0;
        while i < kinds.len() {
            res |= mask(kinds[i]);
            i += 1;
        }
        TokenSet(res)
    }

    pub(crate) const fn contains(&self, kind: SyntaxKind) -> bool {
        self.0 & mask(kind) != 0
    }
}

const fn mask(kind: SyntaxKind) -> u128 {
    1u128 << (kind as usize)
}
