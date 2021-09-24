use crate::parser::expression::{parse_expression, prefixexpr, Expression, PrefixExpr};
use crate::parser::tokens::llet;
use crate::parser::{
    literals::{parse_variable, sp},
    Res, Span,
};

use crate::literals::Literal;
use nom::{
    character::complete::char,
    error::context,
    sequence::{preceded, separated_pair},
};

/// A assignment is a statement, while a re-assignemnt is an epression
/// so we only parse a statement here, therefore is the let mandatory
#[derive(Debug, PartialEq)]
pub struct Assignment<'a> {
    pub variable: PrefixExpr<'a>,
    pub expression: Expression<'a>,
}

/// A assignment is a statement, while a re-assignemnt is an epression
/// so we only parse a statement here, therefore is the let mandatory
#[derive(Debug, PartialEq)]
pub struct LAssignment<'a> {
    pub variable: Literal<'a>,
    pub expression: Expression<'a>,
}

/// Assignment having following schema
/// <let-keyword> <variable> = <expression>
pub(crate) fn parse_assignment(input: Span) -> Res<Assignment> {
    context(
        "Assignment",
        preceded(
            sp,
            separated_pair(
                preceded(sp, prefixexpr),
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
pub(crate) fn parse_lassignment(input: Span) -> Res<LAssignment> {
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

    use crate::literals::Token;
    use crate::parser::expression::Expression;
    use crate::parser::literals::Literal;

    #[test]
    fn test_assignment() {
        let string = "let x = 3";
        let (_, res) = parse_lassignment(Span::new(string)).unwrap();
        assert_eq!(
            res,
            LAssignment {
                variable: Literal::Variable(Token::new("x", Span::new("x"))),
                expression: Expression::Literal(Literal::Num(Token::new(3.0, Span::new("3"))))
            }
        );
    }
}
