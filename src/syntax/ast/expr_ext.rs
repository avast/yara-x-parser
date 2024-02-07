use crate::{
    syntax::ast::{
        self,
        operators::{BinaryOp, LogicOp, UnaryOp},
        support, AstNode, AstToken,
    },
    SyntaxToken, T,
};

impl ast::PrefixExpr {
    pub fn op_kind(&self) -> Option<UnaryOp> {
        let res = match self.op_token()?.kind() {
            T![not] => UnaryOp::Not,
            _ => return None,
        };
        Some(res)
    }

    pub fn op_token(&self) -> Option<SyntaxToken> {
        self.syntax().first_child_or_token()?.into_token()
    }
}

impl ast::Expression {
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

    pub fn lhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).next()
    }

    pub fn rhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).nth(1)
    }

    pub fn sub_exprs(&self) -> (Option<ast::Expr>, Option<ast::Expr>) {
        let mut children = support::children(self.syntax());
        let first = children.next();
        let second = children.next();
        (first, second)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    String(ast::StringLit),
    Number(ast::Number),
    Variable(ast::Variable),
    Bool(bool),
}

impl ast::Literal {
    pub fn token(&self) -> SyntaxToken {
        self.syntax()
            .children_with_tokens()
            .find(|e| !e.kind().is_trivia())
            .and_then(|e| e.into_token())
            .unwrap()
    }

    pub fn kind(&self) -> LiteralKind {
        let token = self.token();

        if let Some(number) = ast::Number::cast(token.clone()) {
            return LiteralKind::Number(number);
        }

        if let Some(variable) = ast::Variable::cast(token.clone()) {
            return LiteralKind::Variable(variable);
        }

        if let Some(string) = ast::StringLit::cast(token.clone()) {
            return LiteralKind::String(string);
        }

        match token.kind() {
            T![true] => LiteralKind::Bool(true),
            T![false] => LiteralKind::Bool(false),
            _ => unreachable!(),
        }
    }
}
