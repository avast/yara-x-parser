use crate::syntax::ast::{self, AstNode};
use crate::syntax::syntax_node::SyntaxElementChildren;

use super::AstToken;

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
