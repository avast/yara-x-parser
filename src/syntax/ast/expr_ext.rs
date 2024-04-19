//! Extension traits for the numerous AST nodes in typed layer
//! These traits provide additional functionality to the AST nodes
//! that is not provided by the generated code.
//! This is the only place where we should add functionality
//! and methods that we want to use in the typed layer.
//!
//! It provides operators methods and methods for obtaining left-hand side and right-hand side
//! of expressions

use crate::{
    syntax::ast::{
        self,
        operators::{BinaryOp, BoolTermExprOp, ExprOp, LogicOp},
        support, AstNode,
    },
    SyntaxToken, T,
};

impl ast::ExprBody {
    pub fn op_details(&self) -> Option<(SyntaxToken, BinaryOp)> {
        self.syntax().children_with_tokens().filter_map(|it| it.into_token()).find_map(|c| {
            let bin_op = match c.kind() {
                T![+] => BinaryOp::ExprOp(ExprOp::Add),
                T![-] => BinaryOp::ExprOp(ExprOp::Sub),
                T![*] => BinaryOp::ExprOp(ExprOp::Mul),
                T![backslash] => BinaryOp::ExprOp(ExprOp::Div),
                T![%] => BinaryOp::ExprOp(ExprOp::Mod),
                T![&] => BinaryOp::ExprOp(ExprOp::BitAnd),
                T![|] => BinaryOp::ExprOp(ExprOp::BitOr),
                T![^] => BinaryOp::ExprOp(ExprOp::BitXor),
                T![<<] => BinaryOp::ExprOp(ExprOp::Shl),
                T![>>] => BinaryOp::ExprOp(ExprOp::Shr),
                T![.] => BinaryOp::ExprOp(ExprOp::Dot),
                _ => return None,
            };
            Some((c, bin_op))
        })
    }

    pub fn op_kind(&self) -> Option<BinaryOp> {
        self.op_details().map(|t| t.1)
    }

    pub fn op_token(&self) -> Option<SyntaxToken> {
        self.op_details().map(|t| t.0)
    }

    pub fn lhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).next()
    }

    pub fn rhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).nth(1)
    }
}

impl ast::XorRange {
    pub fn lhs(&self) -> SyntaxToken {
        self.syntax()
            .children_with_tokens()
            .filter(|e| !e.kind().is_trivia())
            .nth(1)
            .and_then(|e| e.into_token())
            .unwrap()
    }

    pub fn rhs(&self) -> SyntaxToken {
        self.syntax()
            .children_with_tokens()
            .filter(|e| !e.kind().is_trivia())
            .nth(3)
            .and_then(|e| e.into_token())
            .unwrap()
    }
}

impl ast::Range {
    pub fn lhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).next()
    }

    pub fn rhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).nth(1)
    }
}

impl ast::HexJump {
    pub fn lhs(&self) -> SyntaxToken {
        self.syntax()
            .children_with_tokens()
            .filter(|e| !e.kind().is_trivia())
            .nth(1)
            .and_then(|e| e.into_token())
            .unwrap()
    }

    pub fn rhs(&self) -> SyntaxToken {
        self.syntax()
            .children_with_tokens()
            .filter(|e| !e.kind().is_trivia())
            .nth(3)
            .and_then(|e| e.into_token())
            .unwrap()
    }
}

impl ast::BooleanExpr {
    pub fn op_details(&self) -> Option<(SyntaxToken, BinaryOp)> {
        self.syntax().children_with_tokens().filter_map(|it| it.into_token()).find_map(|c| {
            let bin_op = match c.kind() {
                T![and] => BinaryOp::LogicOp(LogicOp::And),
                T![or] => BinaryOp::LogicOp(LogicOp::Or),
                _ => return None,
            };
            Some((c, bin_op))
        })
    }

    pub fn op_kind(&self) -> Option<BinaryOp> {
        self.op_details().map(|t| t.1)
    }

    pub fn op_token(&self) -> Option<SyntaxToken> {
        self.op_details().map(|t| t.0)
    }

    pub fn lhs(&self) -> Option<ast::Expression> {
        support::children(self.syntax()).next()
    }

    pub fn rhs(&self) -> Option<ast::Expression> {
        support::children(self.syntax()).nth(1)
    }
}

impl ast::BooleanTermExpr {
    pub fn op_details(&self) -> Option<(SyntaxToken, BinaryOp)> {
        self.syntax().children_with_tokens().filter_map(|it| it.into_token()).find_map(|c| {
            let bin_op = match c.kind() {
                T![==] => BinaryOp::BoolTermExprOp(BoolTermExprOp::Eq),
                T![!=] => BinaryOp::BoolTermExprOp(BoolTermExprOp::Ne),
                T![<] => BinaryOp::BoolTermExprOp(BoolTermExprOp::Lt),
                T![<=] => BinaryOp::BoolTermExprOp(BoolTermExprOp::Le),
                T![>] => BinaryOp::BoolTermExprOp(BoolTermExprOp::Gt),
                T![>=] => BinaryOp::BoolTermExprOp(BoolTermExprOp::Ge),
                T![contains] => BinaryOp::BoolTermExprOp(BoolTermExprOp::Contains),
                T![icontains] => BinaryOp::BoolTermExprOp(BoolTermExprOp::IContains),
                T![startswith] => BinaryOp::BoolTermExprOp(BoolTermExprOp::StartsWith),
                T![istartswith] => BinaryOp::BoolTermExprOp(BoolTermExprOp::IStartsWith),
                T![endswith] => BinaryOp::BoolTermExprOp(BoolTermExprOp::EndsWith),
                T![iendswith] => BinaryOp::BoolTermExprOp(BoolTermExprOp::IEndsWith),
                T![iequals] => BinaryOp::BoolTermExprOp(BoolTermExprOp::IEquals),
                T![matches] => BinaryOp::BoolTermExprOp(BoolTermExprOp::Matches),
                _ => return None,
            };
            Some((c, bin_op))
        })
    }

    pub fn op_kind(&self) -> Option<BinaryOp> {
        self.op_details().map(|t| t.1)
    }

    pub fn op_token(&self) -> Option<SyntaxToken> {
        self.op_details().map(|t| t.0)
    }

    pub fn lhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).next()
    }

    pub fn rhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).nth(1)
    }
}

impl ast::VariableWildcard {
    pub fn matches(&self, ident: &str) -> bool {
        if self.star_token().is_some() {
            ident.starts_with(self.variable_token().unwrap().text())
        } else {
            ident == self.variable_token().unwrap().text()
        }
    }
}
