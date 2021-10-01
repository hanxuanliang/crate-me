use std::convert::TryFrom;
use sqlparser::ast::{Expr, Select, SetExpr, Statement};
use anyhow::anyhow;

struct Sql<'a> {
    selection: Vec<Expr>,
    condition: Option<Expr>,
    source: &'a str,
    order_by: Vec<(String, bool)>,
    offset: Option<i64>,
    limit: Option<usize>,
}

// Statemene -> Sql (use From/Into)
impl<'a> TryFrom<&'a Statement> for Sql<'a> {
    type Error = anyhow::Error;

    fn try_from(sql: &'a Statement) -> Result<Self, Self::Error> {
        match sql {
            Statement::Query(query) => {
                let Select {
                    from: table_with_joins,
                    selection: where_clause,
                    projection,
                    group_by,
                    ..
                } = match &query.body {
                    SetExpr::Select(statement) => statement.as_ref(),
                    _ => return Err(anyhow!("cannot support ")),
                };

                Ok(Sql {})
            },
            _ => Err(anyhow!("can not support")),
        }
    }
}
