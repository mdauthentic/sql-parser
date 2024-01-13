use super::{common::column, identifier};
use crate::ast::expression::{
    AggregationFunction, BinOp, ColumnReference,
    Expression::{self, *},
    Literal, UnaryOp,
};
use chumsky::prelude::*;

pub fn expression() -> impl Parser<char, Expression, Error = Simple<char>> + Clone {
    let op = |c| just(c).padded();

    let unary_op = choice((
        op("!").map(|_| UnaryOp::LogicalNot),
        op("-").map(|_| UnaryOp::Minus),
    ));

    let bin_op = choice((
        op("+").map(|_| BinOp::Addition),
        op("*").map(|_| BinOp::Multiplication),
        op(">").map(|_| BinOp::Gt),
        op("<").map(|_| BinOp::Lt),
        op("=").map(|_| BinOp::Eq),
    ));

    let expr = recursive(|expr| {
        // ColumnReference and Literal
        let atom = literal().map(Literal).or(column_ref());

        let aliased_expr = atom
            .clone()
            .or(expr.clone())
            .then_ignore(just("AS").padded())
            .then(identifier())
            .map(|(exp, alias)| Alias {
                expr: Box::new(exp),
                alias,
            });

        let unary_expr = unary_op.then(expr.clone()).map(|(op, exp)| UnaryExpr {
            op,
            expr: Box::new(exp),
        });

        let bin_expr = atom
            .clone()
            .or(unary_expr.clone())
            .or(aliased_expr.clone())
            .then(bin_op)
            .then(atom.clone().or(unary_expr.clone()).or(aliased_expr.clone()))
            .map(|((lhs, op), rhs)| BinaryExpr {
                left: Box::new(lhs),
                op,
                right: Box::new(rhs),
            });

        // IN / NOT IN expression
        let in_notin_expr = atom
            .clone()
            .or(unary_expr.clone())
            .or(aliased_expr.clone())
            .or(bin_expr.clone())
            .then(op("or").or(op("NOT")).or_not())
            .then_ignore(op("in").or(op("IN")))
            .then(
                atom.clone()
                    .or(unary_expr.clone())
                    .or(aliased_expr.clone())
                    .or(bin_expr.clone())
                    .separated_by(op(","))
                    .delimited_by(op("("), op(")")),
            )
            .map(|((exp, nt_kw), right)| {
                let not_in = nt_kw.is_some();
                In {
                    left: Box::new(exp),
                    right,
                    not_in,
                }
            });

        // function expression
        let fn_expr = function_op()
            .then_ignore(just("("))
            .then(
                atom.clone()
                    .or(unary_expr.clone())
                    .or(aliased_expr.clone())
                    .or(bin_expr.clone())
                    .or(in_notin_expr.clone())
                    .separated_by(op(",")),
            )
            .then_ignore(op(")"))
            .map(|(func, args)| FunctionExpression { func, args });

        fn_expr
            .or(in_notin_expr.clone())
            .or(bin_expr.clone())
            .or(aliased_expr)
            .or(unary_expr.clone())
            .or(atom)
            .padded()
    });

    expr
}

pub fn column_ref() -> impl Parser<char, Expression, Error = Simple<char>> + Clone {
    let star = just("*").map(|_| ColumnReference(ColumnReference::Wildcard));
    let qualified_star = identifier()
        .then_ignore(just("."))
        .then_ignore(just("*"))
        .map(|id| ColumnReference(ColumnReference::QualifiedWildcard(id)));

    qualified_star
        .or(star)
        .or(column().map(|col| ColumnReference(ColumnReference::Column(col))))
}

pub fn literal() -> impl Parser<char, Literal, Error = Simple<char>> + Clone {
    let exp = one_of("eE").chain(one_of("+-").or_not().chain::<char, _, _>(text::digits(10)));

    let integer = filter(|c: &char| c.is_ascii_digit() && *c != '0')
        .chain::<_, Vec<char>, _>(filter(|c: &char| c.is_ascii_digit() || *c == '_').repeated())
        .or(just('0').map(|c| vec![c]));

    let frac = just('.')
        .chain::<char, _, _>(filter(|c: &char| c.is_ascii_digit()))
        .chain::<char, _, _>(filter(|c: &char| c.is_ascii_digit() || *c == '_').repeated());

    let number = integer
        .chain::<char, _, _>(frac.or_not().flatten())
        .chain::<char, _, _>(exp.or_not().flatten())
        .try_map(|chars, span| {
            let str = chars.into_iter().filter(|c| *c != '_').collect::<String>();

            if let Ok(i) = str.parse::<i64>() {
                Ok(Literal::UnsignedInteger(i))
            } else if let Ok(f) = str.parse::<f64>() {
                Ok(Literal::UnsignedFloat(f))
            } else {
                Err(Simple::expected_input_found(span, None, None))
            }
        });

    let date_literal = filter(|c: &char| c.is_ascii_digit())
        .repeated()
        .exactly(4)
        .collect::<String>()
        .then_ignore(just::<_, _, Simple<char>>("-"))
        .then(
            filter(|c: &char| c.is_ascii_digit())
                .repeated()
                .exactly(2)
                .collect::<String>(),
        )
        .then_ignore(just("-"))
        .then(
            filter(|c: &char| c.is_ascii_digit())
                .repeated()
                .exactly(2)
                .collect::<String>(),
        )
        .map(|((year, month), day)| Literal::Date(format!("{}-{}-{}", year, month, day)));

    let null_literal = text::keyword("null")
        .or(text::keyword("NULL"))
        .ignored()
        .map(|_| Literal::Null);
    let string_literal = just('"')
        .ignore_then(text::ident())
        .then_ignore(just('"'))
        .map(Literal::String);

    choice((null_literal, date_literal, number, string_literal))
}

pub fn function_op() -> impl Parser<char, AggregationFunction, Error = Simple<char>> + Clone {
    let count_kw = text::keyword("count")
        .or(text::keyword("COUNT"))
        .to(AggregationFunction::Count);
    let sum_kw = text::keyword("sum")
        .or(text::keyword("SUM"))
        .to(AggregationFunction::Sum);
    let avg_kw = text::keyword("avg")
        .or(text::keyword("AVG"))
        .to(AggregationFunction::Avg);
    let max_kw = text::keyword("max")
        .or(text::keyword("MAX"))
        .to(AggregationFunction::Max);
    let min_kw = text::keyword("min")
        .or(text::keyword("MIN"))
        .to(AggregationFunction::Min);

    choice((count_kw, sum_kw, avg_kw, max_kw, min_kw))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::expression::*;

    #[test]
    fn test_literal() {
        let actual = literal().parse(r#""col""#).unwrap();
        assert_eq!(actual, Literal::String("col".to_string()));

        let actual_ufloat = literal().parse("20.24").unwrap();
        assert_eq!(actual_ufloat, Literal::UnsignedFloat(20.24));

        let actual_date = literal().parse("2024-01-13").unwrap();
        assert_eq!(actual_date, Literal::Date("2024-01-13".to_string()))
    }
}
