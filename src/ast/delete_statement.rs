use crate::ast::expression::Expression;
use crate::ast::table::Table;

#[derive(Clone, Debug)]
pub struct DeleteStatement {
    pub table: Table,
    pub where_clause: Option<Expression>,
}
