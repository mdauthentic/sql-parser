use crate::ast::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub column: Identifier,
    pub table: Option<Identifier>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnSpecification {
    pub column: Column,
    pub sql_type: String, // change to proper type
    pub constraint: Vec<ColumnConstraint>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnConstraint {
    Unique,
    PrimaryKey,
    AutoIncrement,
    NotNull,
    Nullable,
}
