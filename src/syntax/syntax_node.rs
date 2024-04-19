//! This module represents CST for YARA language
//!
//! THe CST includes trivia such as comments or whitespaces
//! `SyntaxNode` provides basic API that allows to travers the tree
//! to find parent, children or siblings
//! This is just a wrapper around `rowan` crate API

use rowan_test::{GreenNodeBuilder, Language};
use text_size::TextSize;

use crate::parser::{self, syntax_kind::SyntaxKind};
use crate::SyntaxError;

pub(crate) use rowan_test::GreenNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum YARALanguage {}
impl Language for YARALanguage {
    type Kind = SyntaxKind;

    /// Convert from raw kind to SyntaxKind
    fn kind_from_raw(raw: rowan_test::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from(raw.0)
    }

    /// Convert from SyntaxKind to raw kind
    fn kind_to_raw(kind: SyntaxKind) -> rowan_test::SyntaxKind {
        rowan_test::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan_test::SyntaxNode<YARALanguage>;
pub type SyntaxToken = rowan_test::SyntaxToken<YARALanguage>;
#[allow(dead_code)]
pub type SyntaxElement = rowan_test::SyntaxElement<YARALanguage>;
pub type SyntaxNodeChildren = rowan_test::SyntaxNodeChildren<YARALanguage>;
pub type SyntaxElementChildren = rowan_test::SyntaxElementChildren<YARALanguage>;
#[allow(dead_code)]
pub type PreorderWithTokens = rowan_test::api::PreorderWithTokens<YARALanguage>;

#[derive(Default)]
pub struct SyntaxTreeBuilder {
    errors: Vec<SyntaxError>,
    inner: GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    /// Finish building the tree and return the result as a pair of GreenNode and list of errors
    pub(crate) fn finish_raw(self) -> (GreenNode, Vec<SyntaxError>) {
        let green = self.inner.finish();
        (green, self.errors)
    }

    /// Create a token
    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let kind = YARALanguage::kind_to_raw(kind);
        self.inner.token(kind, text)
    }

    /// Start a new node
    pub fn start_node(&mut self, kind: SyntaxKind) {
        let kind = YARALanguage::kind_to_raw(kind);
        self.inner.start_node(kind)
    }

    /// Finish the current node
    pub fn finish_node(&mut self) {
        self.inner.finish_node()
    }

    /// Add a new syntax error to the list of errors
    pub fn error(&mut self, error: parser::ParseError, text_pos: TextSize) {
        self.errors.push(SyntaxError::new_at_offset(error.0, text_pos))
    }
}
