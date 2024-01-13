use crate::ast::column::Column;
use crate::ast::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A named reference to a qualified filed in a schema.
    ColumnReference(ColumnReference),
    /// An expression with a specific name.
    Alias {
        expr: Box<Expression>,
        alias: Identifier,
    },
    /// A constant
    Literal(Literal),
    /// A unary expression such as "-id"
    UnaryExpr { op: UnaryOp, expr: Box<Expression> },
    /// A binary expression e.g. "col = value"
    BinaryExpr {
        left: Box<Expression>,
        op: BinOp,
        right: Box<Expression>,
    },
    //SubQuery(Box<SelectStatement>),
    FunctionExpression {
        func: AggregationFunction,
        args: Vec<Expression>,
    },
    In {
        /// Left hand side expression
        left: Box<Expression>,
        /// Right hand side expression
        right: Vec<Expression>,
        /// this represents "IN" or "NOT IN" depending on the predicate
        not_in: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnReference {
    /// e.g. `*`
    Wildcard,
    /// e.g. `tbl.*`
    QualifiedWildcard(Identifier),
    /// table column
    Column(Column),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Null,
    String(String),
    UnsignedInteger(i64),
    UnsignedFloat(f64),
    Date(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Addition,
    Multiplication,
    Gt,
    Lt,
    Eq,
    /* Subtraction,
    Division,
    Mod,
    GtEq,
    LtEq,
    NotEq,
    And,
    Or, */
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Minus,
    LogicalNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AggregationFunction {
    Count,
    Sum,
    Avg,
    Max,
    Min,
}
