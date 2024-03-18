//! Generated by `sourcegen_ast`, do not edit by hand.

#![allow(clippy::enum_variant_names)]
use crate::{
    syntax::ast::{self, support, AstChildren, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, T,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasComments for SourceFile {}
impl SourceFile {
    pub fn import_stmts(&self) -> AstChildren<ImportStmt> {
        support::children(&self.syntax)
    }
    pub fn include_stmts(&self) -> AstChildren<IncludeStmt> {
        support::children(&self.syntax)
    }
    pub fn rules(&self) -> AstChildren<Rule> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ImportStmt {
    pub fn import_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![import])
    }
    pub fn string_lit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![string_lit])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncludeStmt {
    pub(crate) syntax: SyntaxNode,
}
impl IncludeStmt {
    pub fn include_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![include])
    }
    pub fn string_lit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![string_lit])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasComments for Rule {}
impl Rule {
    pub fn modifiers(&self) -> AstChildren<Modifier> {
        support::children(&self.syntax)
    }
    pub fn rule_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![rule])
    }
    pub fn identifier_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![identifier])
    }
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![:])
    }
    pub fn tags(&self) -> AstChildren<Tag> {
        support::children(&self.syntax)
    }
    pub fn body(&self) -> Option<BlockExpr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Modifier {
    pub(crate) syntax: SyntaxNode,
}
impl Modifier {
    pub fn private_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![private])
    }
    pub fn global_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![global])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag {
    pub(crate) syntax: SyntaxNode,
}
impl Tag {
    pub fn identifier_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![identifier])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasComments for BlockExpr {}
impl BlockExpr {
    pub fn l_brace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn meta(&self) -> Option<Meta> {
        support::child(&self.syntax)
    }
    pub fn strings(&self) -> Option<Strings> {
        support::child(&self.syntax)
    }
    pub fn condition(&self) -> Option<Condition> {
        support::child(&self.syntax)
    }
    pub fn r_brace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Meta {
    pub(crate) syntax: SyntaxNode,
}
impl Meta {
    pub fn meta_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![meta])
    }
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![:])
    }
    pub fn meta_stmts(&self) -> AstChildren<MetaStmt> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Strings {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasComments for Strings {}
impl Strings {
    pub fn strings_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![strings])
    }
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![:])
    }
    pub fn variable_stmts(&self) -> AstChildren<VariableStmt> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Condition {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasComments for Condition {}
impl Condition {
    pub fn condition_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![condition])
    }
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![:])
    }
    pub fn expression_stmt(&self) -> Option<ExpressionStmt> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaStmt {
    pub(crate) syntax: SyntaxNode,
}
impl MetaStmt {
    pub fn identifier_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![identifier])
    }
    pub fn assign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn bool_lit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![bool_lit])
    }
    pub fn string_lit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![string_lit])
    }
    pub fn int_lit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![int_lit])
    }
    pub fn float_lit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![float_lit])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableStmt {
    pub(crate) syntax: SyntaxNode,
}
impl VariableStmt {
    pub fn variable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![variable])
    }
    pub fn assign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn pattern(&self) -> Option<Pattern> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pattern {
    pub(crate) syntax: SyntaxNode,
}
impl Pattern {
    pub fn string_lit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![string_lit])
    }
    pub fn pattern_mods(&self) -> AstChildren<PatternMod> {
        support::children(&self.syntax)
    }
    pub fn hex_pattern(&self) -> Option<HexPattern> {
        support::child(&self.syntax)
    }
    pub fn regex_pattern(&self) -> Option<RegexPattern> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatternMod {
    pub(crate) syntax: SyntaxNode,
}
impl PatternMod {
    pub fn ascii_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ascii])
    }
    pub fn wide_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![wide])
    }
    pub fn nocase_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![nocase])
    }
    pub fn private_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![private])
    }
    pub fn fullword_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![fullword])
    }
    pub fn base64wide_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![base64wide])
    }
    pub fn base64_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![base64])
    }
    pub fn xor_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![xor])
    }
    pub fn base_alphabet(&self) -> Option<BaseAlphabet> {
        support::child(&self.syntax)
    }
    pub fn xor_range(&self) -> Option<XorRange> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexPattern {
    pub(crate) syntax: SyntaxNode,
}
impl HexPattern {
    pub fn l_brace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn hex_token(&self) -> Option<HexToken> {
        support::child(&self.syntax)
    }
    pub fn r_brace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegexPattern {
    pub(crate) syntax: SyntaxNode,
}
impl RegexPattern {
    pub fn regex_lit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![regex_lit])
    }
    pub fn regex_mods(&self) -> AstChildren<RegexMod> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegexMod {
    pub(crate) syntax: SyntaxNode,
}
impl RegexMod {
    pub fn case_insensitive_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![case_insensitive])
    }
    pub fn dot_matches_all_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![dot_matches_all])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexToken {
    pub(crate) syntax: SyntaxNode,
}
impl HexToken {
    pub fn hex_byte(&self) -> Option<HexByte> {
        support::child(&self.syntax)
    }
    pub fn hex_alternative(&self) -> Option<HexAlternative> {
        support::child(&self.syntax)
    }
    pub fn hex_token_tails(&self) -> AstChildren<HexTokenTail> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexByte {
    pub(crate) syntax: SyntaxNode,
}
impl HexByte {
    pub fn hex_lit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![hex_lit])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexAlternative {
    pub(crate) syntax: SyntaxNode,
}
impl HexAlternative {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn hex_token(&self) -> Option<HexToken> {
        support::child(&self.syntax)
    }
    pub fn hex_pipes(&self) -> AstChildren<HexPipe> {
        support::children(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexTokenTail {
    pub(crate) syntax: SyntaxNode,
}
impl HexTokenTail {
    pub fn hex_jumps(&self) -> AstChildren<HexJump> {
        support::children(&self.syntax)
    }
    pub fn hex_byte(&self) -> Option<HexByte> {
        support::child(&self.syntax)
    }
    pub fn hex_alternative(&self) -> Option<HexAlternative> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexJump {
    pub(crate) syntax: SyntaxNode,
}
impl HexJump {
    pub fn l_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['['])
    }
    pub fn hyphen_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![-])
    }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![']'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexPipe {
    pub(crate) syntax: SyntaxNode,
}
impl HexPipe {
    pub fn pipe_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![|])
    }
    pub fn hex_token(&self) -> Option<HexToken> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}
impl Literal {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BaseAlphabet {
    pub(crate) syntax: SyntaxNode,
}
impl BaseAlphabet {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn string_lit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![string_lit])
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XorRange {
    pub(crate) syntax: SyntaxNode,
}
impl XorRange {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn hyphen_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![-])
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExpressionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ExpressionStmt {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expression {
    pub(crate) syntax: SyntaxNode,
}
impl Expression {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixExpr {
    pub(crate) syntax: SyntaxNode,
}
impl PrefixExpr {
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![not])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Expression(Expression),
    PrefixExpr(PrefixExpr),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyHasComments {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasComments for AnyHasComments {}
impl AstNode for SourceFile {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SOURCE_FILE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ImportStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == IMPORT_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IncludeStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == INCLUDE_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Rule {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Modifier {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MODIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Tag {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TAG
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for BlockExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == BLOCK_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Meta {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == META
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Strings {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == STRINGS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Condition {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for MetaStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == META_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for VariableStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VARIABLE_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Pattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PatternMod {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PATTERN_MOD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for HexPattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEX_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RegexPattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == REGEX_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RegexMod {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == REGEX_MOD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for HexToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEX_TOKEN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for HexByte {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEX_BYTE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for HexAlternative {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEX_ALTERNATIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for HexTokenTail {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEX_TOKEN_TAIL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for HexJump {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEX_JUMP
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for HexPipe {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEX_PIPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Literal {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for BaseAlphabet {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == BASE_ALPHABET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for XorRange {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XOR_RANGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ExpressionStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == EXPRESSION_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Expression {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PrefixExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PREFIX_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl From<Expression> for Expr {
    fn from(node: Expression) -> Expr {
        Expr::Expression(node)
    }
}
impl From<PrefixExpr> for Expr {
    fn from(node: PrefixExpr) -> Expr {
        Expr::PrefixExpr(node)
    }
}
impl From<Literal> for Expr {
    fn from(node: Literal) -> Expr {
        Expr::Literal(node)
    }
}
impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, EXPRESSION | PREFIX_EXPR | LITERAL)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            EXPRESSION => Expr::Expression(Expression { syntax }),
            PREFIX_EXPR => Expr::PrefixExpr(PrefixExpr { syntax }),
            LITERAL => Expr::Literal(Literal { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::Expression(it) => &it.syntax,
            Expr::PrefixExpr(it) => &it.syntax,
            Expr::Literal(it) => &it.syntax,
        }
    }
}
impl AnyHasComments {
    #[inline]
    pub fn new<T: ast::HasComments>(node: T) -> AnyHasComments {
        AnyHasComments { syntax: node.syntax().clone() }
    }
}
impl AstNode for AnyHasComments {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SOURCE_FILE | RULE | BLOCK_EXPR | STRINGS | CONDITION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(AnyHasComments { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ImportStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for IncludeStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BlockExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Strings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MetaStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VariableStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PatternMod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HexPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RegexPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RegexMod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HexToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HexByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HexAlternative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HexTokenTail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HexJump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for HexPipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BaseAlphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XorRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExpressionStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PrefixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
