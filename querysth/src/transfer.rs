use std::convert::TryFrom;
use sqlparser::ast::{
    Select, SetExpr, Statement, 
    Expr, 
    Offset as SqlOffset,
    Value as SqlValue,
};
use anyhow::anyhow;

pub struct Offset<'a>(&'a SqlOffset);

struct Sql<'a> {
    selection: Vec<Expr>,
    condition: Option<Expr>,
    source: &'a str,
    order_by: Vec<(String, bool)>,
    offset: Option<i64>,
    limit: Option<usize>,
}

/// sqlparser 解析出来的Statemene -> Sql (use From/Into)
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

/// sqlparser 解析出来的 offset expr -> i64
impl<'a> From<Offset<'a>> for i64 {
    fn from(offset: Offset) -> Self {
        match offset.0 {
            SqlOffset {
                value: Expr::Value(SqlValue::Number(v, _b)),
                ..
            } => v.parse().unwrap_or(0),
            _ => 0,
        }
    }
}
