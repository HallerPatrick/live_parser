use crate::parser::expression::{expression_lexer, Expression};
use crate::parser::{literals::sp, parse_variable, Res, Variable};

use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::cut,
    error::context,
    sequence::{preceded, separated_pair},
};

/// A assignment is a statement, while a re-assignemnt is an epression
/// so we only parse a statement here, therefore is the let mandatory
#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub variable: Variable,
    pub expression: Expression,
}

/// Assignment having following schema
/// <let-keyword> <variable> = <expression>
pub fn parse_assignment(input: &str) -> Res<&str, Assignment> {
    context(
        "Assignment",
        preceded(
            tag("let"),
            preceded(
                sp,
                separated_pair(
                    preceded(sp, parse_variable),
                    cut(preceded(sp, char('='))),
                    expression_lexer,
                ),
            ),
        ),
    )(input)
    .map(|(next_input, (variable, expression))| {
        (
            next_input,
            Assignment {
                variable: variable,
                expression: expression,
            },
        )
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::parser::expression::ExpressionTerm;
    use crate::parser::literals::Literal;

    #[test]
    fn test_assignment() {
        let string = "let x = 3";
        let res = parse_assignment(string);
        assert_eq!(
            res,
            Ok((
                "",
                Assignment {
                    variable: Variable {
                        name: String::from("x")
                    },
                    expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
                }
            ))
        );
    }
}
