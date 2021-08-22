use crate::parser::expression::Expression;
use crate::parser::Res;

use super::Statement;

pub struct While {
    cond: Expression,
    stmts: Vec<Statement>,
}

// fn parse_while(input: &str) -> Res<&str, While> {

// }
