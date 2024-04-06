//! Similary as `operators.rs` it contains various extensions
//! and methods for `ast::Expr` and `ast::Literal` nodes.
//! `LiteralKind` type will probably also be changed during integration
//! It is for now just to showcase its abilities

use crate::{
    syntax::ast::{
        self,
        operators::{BinaryOp, BoolTermExprOp, ExprOp, LogicOp},
        support, AstNode, AstToken,
    },
    SyntaxToken, T,
};

//impl ast::PrefixExpr {
//    pub fn op_kind(&self) -> Option<UnaryOp> {
//        let res = match self.op_token()?.kind() {
//            T![not] => UnaryOp::Not,
//            _ => return None,
//        };
//        Some(res)
//    }
//
//    pub fn op_token(&self) -> Option<SyntaxToken> {
//        self.syntax().first_child_or_token()?.into_token()
//    }
//}
//
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
            .find(|e| !e.kind().is_trivia())
            .and_then(|e| e.into_token())
            .unwrap()
    }

    pub fn rhs(&self) -> SyntaxToken {
        self.syntax()
            .children_with_tokens()
            .filter(|e| !e.kind().is_trivia())
            .last()
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
//
//#[derive(Clone, Debug, PartialEq, Eq, Hash)]
//pub enum LiteralKind {
//    String(ast::StringLit),
//    Int(ast::IntLit),
//    Float(ast::FloatLit),
//    Variable(ast::Variable),
//    Bool(ast::BoolLit),
//}

//impl ast::Literal {
//    pub fn token(&self) -> SyntaxToken {
//        self.syntax()
//            .children_with_tokens()
//            .find(|e| !e.kind().is_trivia())
//            .and_then(|e| e.into_token())
//            .unwrap()
//    }
//
//    pub fn kind(&self) -> LiteralKind {
//        let token = self.token();
//
//        if let Some(number) = ast::IntLit::cast(token.clone()) {
//            LiteralKind::Int(number)
//        } else if let Some(number) = ast::FloatLit::cast(token.clone()) {
//            LiteralKind::Float(number)
//        } else if let Some(variable) = ast::Variable::cast(token.clone()) {
//            LiteralKind::Variable(variable)
//        } else if let Some(string) = ast::StringLit::cast(token.clone()) {
//            LiteralKind::String(string)
//        } else if let Some(boolean) = ast::BoolLit::cast(token.clone()) {
//            LiteralKind::Bool(boolean)
//        } else {
//            unreachable!("Unknown literal kind")
//        }
//    }
//}
//
