//! Implementations of operators for the AST.

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    LogicOp(LogicOp),
    ExprOp(ExprOp),
    BoolTermExprOp(BoolTermExprOp),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LogicOp {
    And,
    Or,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Dot,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BoolTermExprOp {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    Contains,
    IContains,
    StartsWith,
    IStartsWith,
    EndsWith,
    IEndsWith,
    IEquals,
    Matches,
}

impl fmt::Display for LogicOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            LogicOp::And => "and",
            LogicOp::Or => "or",
        };
        f.write_str(res)
    }
}

impl fmt::Display for ExprOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            ExprOp::Add => "+",
            ExprOp::Sub => "-",
            ExprOp::Mul => "*",
            ExprOp::Div => "\\",
            ExprOp::Mod => "%",
            ExprOp::BitAnd => "&",
            ExprOp::BitOr => "|",
            ExprOp::BitXor => "^",
            ExprOp::Shl => "<<",
            ExprOp::Shr => ">>",
            ExprOp::Dot => ".",
        };
        f.write_str(res)
    }
}

impl fmt::Display for BoolTermExprOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            BoolTermExprOp::Eq => "==",
            BoolTermExprOp::Ne => "!=",
            BoolTermExprOp::Gt => ">",
            BoolTermExprOp::Lt => "<",
            BoolTermExprOp::Ge => ">=",
            BoolTermExprOp::Le => "<=",
            BoolTermExprOp::Contains => "contains",
            BoolTermExprOp::IContains => "icontains",
            BoolTermExprOp::StartsWith => "startswith",
            BoolTermExprOp::IStartsWith => "istartswith",
            BoolTermExprOp::EndsWith => "endswith",
            BoolTermExprOp::IEndsWith => "iendswith",
            BoolTermExprOp::IEquals => "iequals",
            BoolTermExprOp::Matches => "matches",
        };
        f.write_str(res)
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::LogicOp(op) => fmt::Display::fmt(op, f),
            BinaryOp::ExprOp(op) => fmt::Display::fmt(op, f),
            BinaryOp::BoolTermExprOp(op) => fmt::Display::fmt(op, f),
        }
    }
}
