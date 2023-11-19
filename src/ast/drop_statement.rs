use crate::ast::table::Table;

#[derive(Debug, Clone)]
pub struct DropStatement {
    pub table: Table,
    pub if_exists: bool,
}
