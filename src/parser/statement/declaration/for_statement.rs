use nom::{
    error::context,
    sequence::{delimited, preceded, terminated, tuple},
};

use crate::parser::literals::sp;
use crate::parser::statement::{parse_statements, Statement};
use crate::parser::tokens::{end, ldo, lfor, lin};
use crate::parser::{parse_variable, Res, Variable};

#[derive(Debug, PartialEq)]
pub struct For {
    iterator: Variable,
    iter_item: Variable,
    stmts: Vec<Statement>,
}

pub fn parse_for(input: &str) -> Res<&str, For> {
    tuple((
        parse_iter_item,
        parse_iterator,
        terminated(parse_statements, end),
    ))(input)
    .map(|(next_input, res)| {
        (
            next_input,
            For {
                iter_item: res.0,
                iterator: res.1,
                stmts: res.2,
            },
        )
    })
}

fn parse_iter_item(input: &str) -> Res<&str, Variable> {
    context(
        "ForIterItem",
        preceded(preceded(sp, lfor), preceded(sp, parse_variable)),
    )(input)
}

fn parse_iterator(input: &str) -> Res<&str, Variable> {
    context(
        "ForIterator",
        delimited(
            preceded(sp, lin),
            preceded(sp, parse_variable),
            preceded(sp, ldo),
        ),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::expression::ExpressionTerm;
    use crate::parser::literals::Literal;
    use crate::parser::statement::Assignment;

    #[test]
    fn test_parse_iter_item() {
        let string = "for item in iterator do";
        let res = parse_iter_item(string);
        assert_eq!(
            res,
            Ok((
                " in iterator do",
                Variable {
                    name: String::from("item")
                }
            ))
        );
    }

    #[test]
    fn test_parse_iterator() {
        let string = " in iterator do";
        let res = parse_iterator(string);
        assert_eq!(
            res,
            Ok((
                "",
                Variable {
                    name: String::from("iterator")
                }
            ))
        );
    }

    #[test]
    fn test_parse_for_loop() {
        let string = "for x in y do x + 3\n let y = 3 \nend";
        let res = parse_for(string);

        assert_eq!(
            res,
            Ok((
                "",
                For {
                    iter_item: Variable {
                        name: String::from("x")
                    },
                    iterator: Variable {
                        name: String::from("y")
                    },
                    stmts: vec![
                        Statement::Expression(vec![
                            ExpressionTerm::Literal(Literal::Variable(Variable {
                                name: String::from("x")
                            })),
                            ExpressionTerm::Token(String::from("+")),
                            ExpressionTerm::Literal(Literal::Num(3.0))
                        ]),
                        Statement::Assignment(Assignment {
                            variable: Variable {
                                name: String::from("y")
                            },
                            expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
                        })
                    ]
                }
            ))
        )
    }
}
