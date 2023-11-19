use crate::ast::column::Column;
use crate::ast::select_statement::SelectStatement;
use crate::ast::Identifier;

#[derive(Debug, Clone)]
pub enum Expression {
    /// An expression with a specific name.
    Alias(Box<Expression>, String),
    /// A named reference to a qualified filed in a schema.
    ColumnReference(Column),
    /// A constant
    Literal(Literal),
    /// type cast
    Cast,
    /// A unary expression such as "-id"
    UnaryExpr {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    /// A binary expression e.g. "col = value"
    BinaryExpr {
        left: Box<Expression>,
        op: BinOp,
        right: Box<Expression>,
    },
    SubQuery(Box<SelectStatement>),
    FunctionExpression {
        name: Identifier,
        args: Vec<FunctionArgs>,
        distinct: bool,
    },
    Exists(Box<SelectStatement>),
    InSubQuery(Box<SelectStatement>),
    In {
        expr: Box<Expression>,
        args: Vec<Expression>,
    },
    NotIn {
        expr: Box<Expression>,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Null,
    String(String),
    UnsignedInteger(usize),
    UnsignedFloat(f64),
    DateTime,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Mod,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Eq,
    NotEq,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum FunctionArgs {
    Wildcard,
    Expr(Expression),
    AggregationFunction {
        func: AggregationFunction,
        expr: Expression,
    },
}

#[derive(Debug, Clone)]
pub enum AggregationFunction {
    Count,
    CountStar,
    CountDistinct,
    Sum,
    Min,
    Max,
    Avg,
}
