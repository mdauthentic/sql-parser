use crate::ast::column::Column;
use crate::ast::expression::Expression;
use crate::ast::table::Table;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UpdateStatement {
    pub table: Table,
    pub fields: Vec<SetFields>,
    pub where_clause: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct SetFields {
    pub fields: HashMap<Column, Expression>,
}
