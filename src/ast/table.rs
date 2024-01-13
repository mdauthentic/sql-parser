use crate::ast::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub name: Identifier,
    pub database: Option<Identifier>,
}
