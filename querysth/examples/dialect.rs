use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {
    tracing_subscriber::fmt::init();

    let sql_1 = "select a, b from source";
    let ast = Parser::parse_sql(&GenericDialect::default(), sql_1);
    println!("{:#?}", ast);
}
