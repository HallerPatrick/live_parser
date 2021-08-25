use crate::parser::expression::{expression_lexer, Expression};
use crate::parser::{literals::sp, parse_variable, Res, Variable};

use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, opt},
    error::context,
    sequence::{preceded, separated_pair},
};

use super::Statement;

#[derive(Debug, PartialEq)]
pub struct Assignment {
    // Wether a new variable is initalized or just reassigned
    init: bool,
    variable: Variable,
    expression: Expression,
}

fn parse_assign_keyword(input: &str) -> Res<&str, bool> {
    opt(tag("let"))(input).map(|(next_input, res)| (next_input, res.is_some()))
}

/// Assignment having following schema
/// <let-keyword> <variable> = <expression>
/// the let keyword is optional
pub fn parse_assignment(input: &str) -> Res<&str, Statement> {
    let (out, init) = parse_assign_keyword(input)?;

    context(
        "Assignment",
        preceded(
            sp,
            separated_pair(
                preceded(sp, parse_variable),
                cut(preceded(sp, char('='))),
                expression_lexer,
            ),
        ),
    )(out)
    .map(|(next_input, (variable, expression))| {
        (
            next_input,
            Statement::Assignment(Assignment {
                init: init,
                variable: variable,
                expression: expression,
            }),
        )
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::parser::expression::ExpressionTerm;
    use crate::parser::literals::Literal;

    #[test]
    fn test_let_keyword() {
        let string = "let";
        let res = parse_assign_keyword(string);
        assert_eq!(res, Ok(("", true)));

        let string = "nlet";
        let res = parse_assign_keyword(string);
        assert_eq!(res, Ok(("nlet", false)));
    }
    #[test]
    fn test_assignment() {
        let string = "let x = 3";
        let res = parse_assignment(string);
        assert_eq!(
            res,
            Ok((
                "",
                Statement::Assignment(Assignment {
                    init: true,
                    variable: Variable {
                        name: String::from("x")
                    },
                    expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
                })
            ))
        );
    }

    #[test]
    fn test_assignment_no_init() {
        let string = " x = 3";
        let res = parse_assignment(string);
        assert_eq!(
            res,
            Ok((
                "",
                Statement::Assignment(Assignment {
                    init: false,
                    variable: Variable {
                        name: String::from("x")
                    },
                    expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
                })
            ))
        );
    }
}
