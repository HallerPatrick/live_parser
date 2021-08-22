use crate::parser::Res;
use crate::parser::expression::Expression;

pub struct While {
    cond: Expression,
    stmts: Vec<Statement>
}

fn parse_while(input: &str) -> Res<&str, While> {

}
