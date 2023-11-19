use crate::ast::expression::Expression;
use crate::ast::join::{JoinCondition, JoinType};
use crate::ast::table::Table;
use crate::ast::Identifier;

#[derive(Debug, Clone)]
pub struct SelectClause {
    distinct: bool,
    projection: Vec<Projection>,
    from: Option<TableReference>,
    where_exp: Option<Expression>,
    group_by: Vec<Expression>,
    having: Option<Expression>,
}

#[derive(Debug, Clone)]
pub enum Projection {
    WildCard,
    QualifiedWildcard(Identifier),
    UnnamedExpr(Expression),
    AliasedExpr { expr: Expression, alias: Identifier },
}

#[derive(Debug, Clone)]
pub enum TableReference {
    BaseRelation(Table),
    SubQuery {
        subquery: Box<SelectStatement>,
        alias: Option<Identifier>,
    },
    Join {
        left: Box<TableReference>,
        right: Box<TableReference>,
        op: JoinType,
        cond: JoinCondition,
    },
}

#[derive(Debug, Clone)]
pub struct SelectStatement {
    body: SelectClause,
    /// e.g. ORDER BY FirstName ASC, LastName DESC
    order_by: Vec<OrderBy>,
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct OrderBy {
    expr: Expression,
    direction: Option<Order>,
}

#[derive(Debug, Clone)]
pub enum Order {
    Asc,
    Desc,
}
