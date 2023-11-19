pub mod column;
mod delete_statement;
pub mod drop_statement;
pub mod expression;
pub mod insert_statement;
pub mod join;
pub mod select_statement;
pub mod table;
pub mod update_statement;

use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
