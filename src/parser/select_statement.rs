use chumsky::prelude::*;

use crate::ast::{expression::Expression, join::*, select_statement::*};

use super::{common::table, identifier};

pub fn select_statement<E>(
    expr: E,
) -> impl Parser<char, SelectStatement, Error = Simple<char>> + Clone
where
    E: Parser<char, Expression, Error = Simple<char>> + Clone + 'static,
{
    let op = |c| text::keyword(c).padded();

    let num = text::digits(10).map(|s: String| s.parse::<i64>().unwrap());

    let order_expr = op("order").or(op("ORDER")).then(op("by").or(op("BY")));
    let limit_expr = op("limit").or(op("LIMIT")).ignore_then(num).or_not();
    let offset_expr = op("offset").or(op("OFFSET")).ignore_then(num).or_not();

    recursive(|stmt| {
        let sl_clause = select_clause(stmt, expr.clone());
        let order_by_expr = order_by(expr.clone()).separated_by(just(","));
        sl_clause
            .then(order_expr.ignore_then(order_by_expr))
            .then(limit_expr)
            .then(offset_expr)
            .map(|(((body, order_by), limit), offset)| SelectStatement {
                body,
                order_by,
                limit,
                offset,
            })
    })
    .boxed()
}

pub fn select_clause<S, E>(
    stmt: S,
    expr: E,
) -> impl Parser<char, SelectClause, Error = Simple<char>>
where
    S: Parser<char, SelectStatement, Error = Simple<char>> + Clone + 'static,
    E: Parser<char, Expression, Error = Simple<char>> + Clone + 'static,
{
    let op = |c| text::keyword(c).padded();

    let select_keyword = op("select").or(op("SELECT"));

    let distinct_or_not = op("distinct")
        .or(op("DISTINCT"))
        .or_not()
        .map(|dist| match dist {
            Some(_) => true,
            None => false,
        });

    let projection_clause = expr.clone().separated_by(just(","));

    let table_refrence = table_ref(stmt, expr.clone());

    let from_clause = op("from").or(op("FROM")).ignore_then(table_refrence);

    let where_expr = op("where")
        .or(op("WHERE"))
        .ignore_then(expr.clone())
        .or_not();

    let group_by = op("group")
        .or(op("GROUP"))
        .ignore_then(op("by").or(op("BY")))
        .ignore_then(expr.clone().separated_by(just(",")))
        .or_not();

    let having_expr = op("having")
        .or(op("HAVING"))
        .ignore_then(expr.clone())
        .or_not();

    select_keyword
        .ignore_then(distinct_or_not)
        .then(projection_clause)
        .then(from_clause)
        .then(where_expr)
        .then(group_by)
        .then(having_expr)
        .map(
            |(((((dist, projection), frm), where_exp), group_by), having)| {
                let group_by_expression = match group_by {
                    Some(grp) => grp,
                    None => vec![],
                };
                SelectClause {
                    distinct: dist,
                    projection,
                    from: frm,
                    where_exp,
                    group_by: group_by_expression,
                    having,
                }
            },
        )
}

pub fn table_ref<S, E>(
    stmt: S,
    expr: E,
) -> impl Parser<char, Option<TableReference>, Error = Simple<char>> + Clone
where
    S: Parser<char, SelectStatement, Error = Simple<char>> + Clone + 'static,
    E: Parser<char, Expression, Error = Simple<char>> + 'static,
{
    let table_reference = recursive(|table_ref| {
        let base_rel = table().map(TableReference::BaseRelation);

        let subquery = stmt
            .delimited_by(just("("), just(")"))
            .then_ignore(text::keyword("as").or(text::keyword("AS")).or_not())
            .padded()
            .then(identifier().or_not())
            .map(|(subquery, alias)| TableReference::SubQuery {
                subquery: Box::new(subquery),
                alias,
            });

        let join_ref = base_rel
            .clone()
            .or(subquery.clone())
            .or(table_ref.clone())
            .then(join_type())
            .then(base_rel.clone().or(subquery.clone()).or(table_ref.clone()))
            .then(join_condition(expr))
            .map(|(((left, op), right), cond)| TableReference::Join {
                left: Box::new(left),
                op,
                right: Box::new(right),
                cond,
            });

        subquery.clone().or(join_ref).or(base_rel)
    })
    .boxed();

    table_reference.or_not()
}

pub fn join_type() -> impl Parser<char, JoinType, Error = Simple<char>> {
    let op = |c| text::keyword(c).padded();

    let join_keyword = op("join").or(op("JOIN"));

    let inner_join = op("inner").or(op("INNER")).map(|_| JoinType::Inner);

    let left_join = op("left").or(op("LEFT")).map(|_| JoinType::LeftJoin);

    let right_join = op("right").or(op("RIGHT")).map(|_| JoinType::RightJoin);

    let full_join = op("full")
        .or(op("FULL"))
        .then(op("outer").or(op("OUTER")).or_not())
        .map(|_| JoinType::FullOuterJoin);

    let cross_join = op("cross").or(op("CROSS")).map(|_| JoinType::CrossJoin);

    inner_join
        .or(left_join)
        .or(right_join)
        .or(full_join)
        .or(cross_join)
        .then_ignore(join_keyword.clone())
        .or(join_keyword.clone().map(|_| JoinType::Inner))
}

pub fn join_condition<E>(expr: E) -> impl Parser<char, JoinCondition, Error = Simple<char>>
where
    E: Parser<char, Expression, Error = Simple<char>>,
{
    let op = |c| text::keyword(c).padded();

    let using_cond = op("using")
        .or(op("USING"))
        .ignore_then(identifier().separated_by(just(",")))
        .map(|idents| JoinCondition::Using(idents));

    let on_cond = op("on")
        .or(op("ON"))
        .ignore_then(expr.separated_by(just(",")))
        .map(|expr| JoinCondition::On(expr));

    using_cond.or(on_cond)
}

pub fn order_by<E>(expr: E) -> impl Parser<char, OrderBy, Error = Simple<char>>
where
    E: Parser<char, Expression, Error = Simple<char>>,
{
    expr.separated_by(just(","))
        .then(order_dir().or_not())
        .padded()
        .map(|(expr, direction)| OrderBy { expr, direction })
}

pub fn order_dir() -> impl Parser<char, Order, Error = Simple<char>> {
    let op = |c| text::keyword(c).padded();

    let asc_order = op("asc").or(op("ASC")).ignored().map(|_| Order::Asc);
    let desc_order = op("desc").or(op("DESC")).ignored().map(|_| Order::Desc);

    asc_order.or(desc_order)
}
