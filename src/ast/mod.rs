use std::fmt::{Display, Formatter};

pub mod column;
pub mod delete_statement;
pub mod drop_statement;
pub mod expression;
pub mod insert_statement;
pub mod join;
pub mod select_statement;
pub mod table;
pub mod update_statement;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
