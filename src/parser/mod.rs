use crate::ast::Identifier;
use chumsky::prelude::*;

pub mod common;
pub mod expression;
pub mod select_statement;

pub fn identifier() -> impl Parser<char, Identifier, Error = Simple<char>> + Clone {
    let ident = text::ident().padded();

    let quoted_ident = just("`")
        .ignore_then(ident)
        .then_ignore(just("`"))
        .map(Identifier);

    ident.map(Identifier).or(quoted_ident)
}

/// Contains a list of reserved words to be matched against. These words cannot be used as an identifier.
pub fn is_reserved_word(name: &str) -> bool {
    match name {
        "create" | "select" | "drop" | "rename" | "case" | "else" | "if" | "instanceof"
        | "where" | "truncate" | "merge" | "new" | "escape" | "raw" | "fetch" | "insert"
        | "while" | "const" | "alter" | "exists" | "with" | "for" | "switch" | "yield"
        | "throw" | "delete" | "index" | "true" | "false" | "let" => true,
        _ => false,
    }
}

/// Comments are meant to be ignored
pub fn comments() -> impl Parser<char, (), Error = Simple<char>> {
    let multi_line = just("/*").padded().then(take_until(just("*/"))).ignored();
    let single_line = just("--")
        .padded()
        .then(take_until(text::newline()))
        .ignored();
    single_line.or(multi_line)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identifier() {
        let expected = Identifier("col".to_string());

        let actual = identifier().parse("col").unwrap();
        assert_eq!(actual, expected);

        let expected2 = Identifier("col_1".to_string());
        let actual2 = identifier().parse("col_1").unwrap();
        assert_eq!(actual2, expected2);
    }

    #[test]
    fn test_escaped_identifier() {
        let expected = Identifier("col".to_string());
        let actual = identifier().parse("`col`").unwrap();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_single_line_comment() {
        let actual = comments().parse("-- this is a comment\n").unwrap();
        assert_eq!(actual, ())
    }

    #[test]
    fn test_multi_line_comment() {
        let actual = comments().parse("/*this is a comment*/").unwrap();
        assert_eq!(actual, ())
    }
}
