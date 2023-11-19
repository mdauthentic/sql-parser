use crate::ast::column::Column;
use crate::ast::expression::Literal;
use crate::ast::table::Table;

#[derive(Debug, Clone)]
pub struct InsertStatement {
    pub table: Table,
    pub fields: Option<Vec<Column>>,
    pub data: Vec<Vec<Literal>>,
}
