use crate::ast::expression::Expression;
use crate::ast::Identifier;

#[derive(Debug, Clone)]
pub enum JoinType {
    Inner,
    LeftJoin,
    RightJoin,
    FullOuterJoin,
    CrossJoin,
}

#[derive(Debug, Clone)]
pub enum JoinCondition {
    Using(Vec<Identifier>),
    On(Box<Expression>),
}
