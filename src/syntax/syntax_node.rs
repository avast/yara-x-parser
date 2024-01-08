//! This module defines Concrete Syntax Tree (CST), used by rust-analyzer.
//!
//! The CST includes comments and whitespace, provides a single node type,
//! `SyntaxNode`, and a basic traversal API (parent, children, siblings).
//!
//! The *real* implementation is in the (language-agnostic) `rowan` crate, this
//! module just wraps its API.

use rowan_test::{GreenNodeBuilder, Language};
use text_size::TextSize;

use crate::parser::{self, syntaxkind::SyntaxKind};
use crate::SyntaxError;

pub(crate) use rowan_test::GreenNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum YARALanguage {}
impl Language for YARALanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan_test::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan_test::SyntaxKind {
        rowan_test::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan_test::SyntaxNode<YARALanguage>;
pub type SyntaxToken = rowan_test::SyntaxToken<YARALanguage>;
pub type SyntaxElement = rowan_test::SyntaxElement<YARALanguage>;
pub type SyntaxNodeChildren = rowan_test::SyntaxNodeChildren<YARALanguage>;
pub type SyntaxElementChildren = rowan_test::SyntaxElementChildren<YARALanguage>;
pub type PreorderWithTokens = rowan_test::api::PreorderWithTokens<YARALanguage>;

#[derive(Default)]
pub struct SyntaxTreeBuilder {
    errors: Vec<SyntaxError>,
    inner: GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    pub(crate) fn finish_raw(self) -> (GreenNode, Vec<SyntaxError>) {
        let green = self.inner.finish();
        (green, self.errors)
    }

    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let kind = YARALanguage::kind_to_raw(kind);
        self.inner.token(kind, text)
    }

    pub fn start_node(&mut self, kind: SyntaxKind) {
        let kind = YARALanguage::kind_to_raw(kind);
        self.inner.start_node(kind)
    }

    pub fn finish_node(&mut self) {
        self.inner.finish_node()
    }

    pub fn error(&mut self, error: parser::ParseError, text_pos: TextSize) {
        self.errors
            .push(SyntaxError::new_at_offset(*error.0, text_pos))
    }
}
