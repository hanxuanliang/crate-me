use sqlparser::dialect::Dialect;

#[derive(Debug, Default)]
pub struct TryDialect;

impl Dialect for TryDialect {
    fn is_identifier_start(&self, c: char) -> bool {
        ('a'..='z').contains(&c) 
            || ('A'..='Z').contains(&c) 
            || c == '_'
    }
    // identifier contains ':', '/', '?', '&', '='
    fn is_identifier_part(&self, c: char) -> bool {
        ('a'..='z').contains(&c)
            || ('A'..='Z').contains(&c)
            || ('0'..='9').contains(&c)
            || [':', '/', '?', '&', '=', '+', '#', '.', '-', '_', '%'].contains(&c)
    }
}

#[allow(dead_code)]
pub fn example_sql() -> String {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let sql = format!(
        "SELECT * FROM {}",
        url
    );
    sql
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::parser::Parser;
    
    #[test]
    fn it_works() {
        assert!(Parser::parse_sql(&TryDialect::default(), &example_sql()).is_ok());
    }
}
