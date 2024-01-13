use chumsky::prelude::*;

use crate::ast::{column::Column, table::Table};

use super::identifier;

pub fn column() -> impl Parser<char, Column, Error = Simple<char>> + Clone {
    let simple_col = identifier().map(|column| Column {
        column,
        table: None,
    });

    let qualified_col =
        identifier()
            .then_ignore(just("."))
            .then(identifier())
            .map(|(tbl, column)| Column {
                column,
                table: Some(tbl),
            });

    choice((qualified_col, simple_col))
}

pub fn table() -> impl Parser<char, Table, Error = Simple<char>> + Clone {
    let simple_tbl = identifier().map(|name| Table {
        name,
        database: None,
    });

    let qualified_tbl = identifier()
        .then_ignore(just("."))
        .then(identifier())
        .map(|(db, name)| Table {
            name,
            database: Some(db),
        });

    choice((qualified_tbl, simple_tbl))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::Identifier;

    #[test]
    fn test_parse_column() {
        let expected = Column {
            column: Identifier("colA".to_string()),
            table: None,
        };
        let actual = column().parse("colA").unwrap();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_qualified_parse_column() {
        let expected = Column {
            column: Identifier("colA".to_string()),
            table: Some(Identifier("tbl".to_string())),
        };
        let actual = column().parse("tbl.colA").unwrap();
        assert_eq!(actual, expected)
    }
}
