use crate::ast::Identifier;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: Identifier,
    pub database: Option<Identifier>,
}
