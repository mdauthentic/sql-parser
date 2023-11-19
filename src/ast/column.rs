use crate::ast::Identifier;

#[derive(Debug, Clone)]
pub struct Column {
    pub column: Identifier,
    pub table: Option<Identifier>,
    pub database: Option<Identifier>,
}

#[derive(Clone, Debug)]
pub struct ColumnSpecification {
    pub column: Column,
    pub sql_type: String, // change to proper type
    pub constraint: Vec<ColumnConstraint>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug)]
pub enum ColumnConstraint {
    Unique,
    PrimaryKey,
    AutoIncrement,
    NotNull,
    Nullable,
}
