//! Implementation of these traits is in `generated/` folder
//! So far the only implemented trait is `HasComments` which is used
//! to iterate over comments in the syntax tree
//! This can be easily extended to support other traits

use crate::syntax::ast::{self, support, AstNode};
use crate::syntax::syntax_node::SyntaxElementChildren;

use super::AstToken;

pub trait HasModifier: AstNode {
    fn modifier(&self) -> Vec<String> {
        support::children::<ast::Modifier>(self.syntax())
            .map(|m| m.syntax().text().to_string())
            .collect::<Vec<_>>()
    }
}
pub trait HasComments: AstNode {
    fn comments(&self) -> CommentIter {
        CommentIter { iter: self.syntax().children_with_tokens() }
    }
}

impl CommentIter {
    pub fn from_syntax_node(syntax_node: &ast::SyntaxNode) -> CommentIter {
        CommentIter { iter: syntax_node.children_with_tokens() }
    }
}

pub struct CommentIter {
    iter: SyntaxElementChildren,
}

impl Iterator for CommentIter {
    type Item = ast::Comment;
    fn next(&mut self) -> Option<ast::Comment> {
        self.iter.by_ref().find_map(|el| el.into_token().and_then(ast::Comment::cast))
    }
}
