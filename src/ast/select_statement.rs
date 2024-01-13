use crate::ast::expression::Expression;
use crate::ast::join::{JoinCondition, JoinType};
use crate::ast::table::Table;
use crate::ast::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct SelectClause {
    pub distinct: bool,
    pub projection: Vec<Expression>,
    pub from: Option<TableReference>,
    pub where_exp: Option<Expression>,
    pub group_by: Vec<Expression>,
    pub having: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableReference {
    BaseRelation(Table),
    SubQuery {
        subquery: Box<SelectStatement>,
        alias: Option<Identifier>,
    },
    Join {
        left: Box<TableReference>,
        op: JoinType,
        right: Box<TableReference>,
        cond: JoinCondition,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectStatement {
    pub body: SelectClause,
    /// e.g. ORDER BY FirstName ASC, LastName DESC
    pub order_by: Vec<OrderBy>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderBy {
    pub expr: Vec<Expression>,
    pub direction: Option<Order>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Order {
    Asc,
    Desc,
}
