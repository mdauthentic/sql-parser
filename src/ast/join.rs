use crate::ast::expression::Expression;
use crate::ast::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub enum JoinType {
    Inner,
    LeftJoin,
    RightJoin,
    FullOuterJoin,
    CrossJoin,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JoinCondition {
    Using(Vec<Identifier>),
    On(Vec<Expression>),
}
