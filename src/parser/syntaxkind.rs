#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    TOMBSTONE,
    EOF,
    RULE,
    STRINGS,
    CONDITION,
    AND,
    OR,
    NOT,
    IDENTIFIER,
    VARIABLE,
    STRING,
    ASSIGN,
    COLON,
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    COMMA,
    NUMBER,
    TRUE,
    FALSE,
    WHITESPACE,
    COMMENT,
    ERROR,
    SOURCE_FILE,
    BLOCK_EXPR,
    PREFIX_EXPR,
    LITERAL,
    EXPRESSION,
    EXPRESSION_STMT,
    __LAST,
}

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
