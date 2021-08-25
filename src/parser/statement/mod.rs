mod assignment;
mod while_statement;

use nom::branch::alt;
use nom::combinator::map;

use assignment::{parse_assignment, Assignment};
use while_statement::{parse_while, While};

use super::expression::{expression_lexer, Expression};
use super::Res;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment(Assignment),
    While(While),
    Expression(Expression),
}

fn parse_expr_as_stmt(input: &str) -> Res<&str, Statement> {
    map(expression_lexer, Statement::Expression)(input)
}

pub fn parse_statement(input: &str) -> Res<&str, Statement> {
    alt((parse_expr_as_stmt, parse_assignment, parse_while))(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::expression::ExpressionTerm;
    use crate::parser::Variable;

    #[test]
    fn test_parse_statement_expr() {
        let string = "x";
        let res = parse_statement(string);
        assert_eq!(
            res,
            Ok((
                "",
                Statement::Expression(vec![ExpressionTerm::Variable(Variable {
                    name: String::from("x")
                })])
            ))
        );
    }
}
