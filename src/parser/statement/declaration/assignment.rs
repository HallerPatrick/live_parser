use crate::parser::expression::{parse_expression, Expression};
use crate::parser::tokens::llet;
use crate::parser::{
    literals::{parse_variable, sp, Variable},
    Res,
};

use nom::{
    character::complete::char,
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

/// A assignment is a statement, while a re-assignemnt is an epression
/// so we only parse a statement here, therefore is the let mandatory
#[derive(Debug, PartialEq)]
pub struct LAssignment {
    pub variable: Variable,
    pub expression: Expression,
}

/// Assignment having following schema
/// <let-keyword> <variable> = <expression>
pub(crate) fn parse_assignment(input: &str) -> Res<&str, Assignment> {
    context(
        "Assignment",
        preceded(
            sp,
            separated_pair(
                preceded(sp, parse_variable),
                preceded(sp, char('=')),
                parse_expression,
            ),
        ),
    )(input)
    .map(|(next_input, (variable, expression))| {
        (
            next_input,
            Assignment {
                variable,
                expression,
            },
        )
    })
}

/// Assignment having following schema
/// <let-keyword> <variable> = <expression>
pub(crate) fn parse_lassignment(input: &str) -> Res<&str, LAssignment> {
    context(
        "LAssignment",
        preceded(
            preceded(sp, llet),
            preceded(
                sp,
                separated_pair(
                    preceded(sp, parse_variable),
                    preceded(sp, char('=')),
                    parse_expression,
                ),
            ),
        ),
    )(input)
    .map(|(next_input, (variable, expression))| {
        (
            next_input,
            LAssignment {
                variable,
                expression,
            },
        )
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::parser::expression::Expression;
    use crate::parser::literals::Literal;

    #[test]
    fn test_assignment() {
        let string = "let x = 3";
        let res = parse_lassignment(string);
        assert_eq!(
            res,
            Ok((
                "",
                LAssignment {
                    variable: Variable {
                        name: String::from("x")
                    },
                    expression: Expression::Literal(Literal::Num(3.0))
                }
            ))
        );
    }
}
