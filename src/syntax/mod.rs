//! Syntax tree representation
//!
//! Properties:
//!    - errors handling
//!    - full-fidelity representation
//!    - easy to navigate
//!    - in future easy to extend with incremental re-parsing
//!
//! It is inspired by the Swift's libSyntax and the Rust's rowan.
//! [Swift]: <https://github.com/apple/swift/blob/13d593df6f359d0cb2fc81cfaac273297c539455/lib/Syntax/README.md>
//! [Rust-analyzer]: <https://github.com/rust-lang/rust-analyzer>
//!
//! It uses modified rowan crate for storing all the information in fast and convinient way.
//! [Rowan]: <https://github.com/rust-lang/rust-analyzer>
//! 
//! More detailed information can be also found in `rust-analyzer` syntax documentation
//! [Rust-analyzer]: <https://github.com/rust-lang/rust-analyzer/blob/4b7675fcc30d3e2c05eafc68a5724db66b58142c/docs/dev/syntax.md>

pub mod ast;
pub mod syntax_error;
pub mod syntax_node;
pub mod text_token_source;
pub mod text_tree_sink;
#[cfg(test)]
mod tests;

pub use rowan_test::GreenNode;
use std::{marker::PhantomData, sync::Arc};

use crate::{
    lexer::tokenize,
    parser::{self, SyntaxKind},
    syntax::{ast::AstNode, syntax_node::SyntaxNode},
    SyntaxError, TextTokenSource, TextTreeSink,
};

macro_rules! format_to {
    ($buf:expr) => ();
    ($buf:expr, $lit:literal $($arg:tt)*) => {
        { use ::std::fmt::Write as _; let _ = ::std::write!($buf, $lit $($arg)*); }
    };
}

/// A result of a successful parsing of a source file.
/// It provides AST and list of errors.
/// We always produce a syntax tree, even for invalid files.
pub struct Parse<T> {
    green: GreenNode,
    errors: Arc<Vec<SyntaxError>>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Parse<T> {
        Parse { green: self.green.clone(), errors: self.errors.clone(), _ty: PhantomData }
    }
}

impl<T> Parse<T> {
    fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
        Parse { green, errors: Arc::new(errors), _ty: PhantomData }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}

impl<T: AstNode> Parse<T> {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_syntax(self) -> Parse<SyntaxNode> {
        Parse { green: self.green, errors: self.errors, _ty: PhantomData }
    }

    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }

    pub fn ok(self) -> Result<T, Arc<Vec<SyntaxError>>> {
        if self.errors.is_empty() {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

impl Parse<SyntaxNode> {
    pub fn cast<N: AstNode>(self) -> Option<Parse<N>> {
        if N::cast(self.syntax_node()).is_some() {
            Some(Parse { green: self.green, errors: self.errors, _ty: PhantomData })
        } else {
            None
        }
    }
}

impl Parse<SourceFile> {
    pub fn debug_dump(&self) -> String {
        let mut buf = format!("{:#?}", self.tree().syntax());
        for err in self.errors.iter() {
            format_to!(buf, "error {:?}: {}\n", err.range(), err);
        }
        buf
    }
}

/// Source file represents single YARA file that can contain multiple rules
/// So far only subset of YARA is supported
/// YARA file is at this point represented as a string on input
pub use crate::syntax::ast::SourceFile;

impl SourceFile {
    pub fn parse(text: &str) -> Parse<SourceFile> {
        let (green, errors) = parse_text(text);
        let root = SyntaxNode::new_root(green.clone());

        assert_eq!(root.kind(), SyntaxKind::SOURCE_FILE);
        Parse { green, errors: Arc::new(errors), _ty: PhantomData }
    }
}

/// Parses the given string representation of file into a syntax tree.
fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
    let (tokens, lexer_errors) = tokenize(text);
    let mut token_source = TextTokenSource::new(text, &tokens);
    let mut tree_sink = TextTreeSink::new(text, &tokens);

    parser::parse(&mut token_source, &mut tree_sink);
    let (tree, mut parser_errors) = tree_sink.finish();
    parser_errors.extend(lexer_errors);

    (tree, parser_errors)
}
