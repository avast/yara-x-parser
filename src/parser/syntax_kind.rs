//! SyntaxKind is the main enum for the syntax tree.
//! It represents the kind of a node in the syntax tree
//! for YARA language
//!
//! all the variants are generated and located in `syntax_kind/generated.rs`

mod generated;

#[allow(unreachable_pub)]
pub use self::generated::{SyntaxKind, T};

impl From<u16> for SyntaxKind {
    #[inline]
    fn from(d: u16) -> SyntaxKind {
        assert!(d <= (SyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(k: SyntaxKind) -> u16 {
        k as u16
    }
}

impl SyntaxKind {
    #[inline]
    pub fn is_trivia(self) -> bool {
        matches!(self, SyntaxKind::WHITESPACE | SyntaxKind::COMMENT)
    }
}
