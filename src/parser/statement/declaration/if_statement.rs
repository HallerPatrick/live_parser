use crate::parser::expression::expression_lexer;
use crate::parser::expression::Expression;
use crate::parser::literals::sp;
use crate::parser::tokens::{end, ldo, lelse, lif};
use crate::parser::Res;

use crate::parser::statement::{parse_statements, Statement};

use nom::{
    combinator::opt,
    error::context,
    sequence::{delimited, preceded, terminated, tuple},
};

#[derive(Debug, PartialEq)]
pub struct If {
    cond: Expression,
    stmts: Vec<Statement>,
    else_statements: Option<Vec<Statement>>,
}

pub fn parse_if(input: &str) -> Res<&str, If> {
    tuple((
        if_condition,
        parse_statements,
        terminated(opt(else_statements), end),
    ))(input)
    .map(|(next_input, res)| {
        (
            next_input,
            If {
                cond: res.0,
                stmts: res.1,
                else_statements: res.2,
            },
        )
    })
}

fn else_statements(input: &str) -> Res<&str, Vec<Statement>> {
    context("ElseStmt", preceded(preceded(sp, lelse), parse_statements))(input)
}

fn if_condition(input: &str) -> Res<&str, Expression> {
    context(
        "IfCond",
        delimited(
            preceded(sp, lif),
            preceded(sp, expression_lexer),
            preceded(sp, ldo),
        ),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::parser::expression::ExpressionTerm;
    use crate::parser::literals::Literal;
    use crate::parser::statement::declaration::assignment::Assignment;
    use crate::parser::Variable;

    #[test]
    fn parse_if_condition() {
        let string = "if x < 3 do";
        let res = if_condition(string);

        assert_eq!(
            res,
            Ok((
                "",
                vec![
                    ExpressionTerm::Literal(Literal::Variable(Variable {
                        name: String::from("x")
                    })),
                    ExpressionTerm::Token(String::from("<")),
                    ExpressionTerm::Literal(Literal::Num(3.0))
                ]
            ))
        )
    }

    #[test]
    fn parse_if_statements() {
        let string = "if x < 3 do x + 3\n let y = 3 \nend";
        let res = parse_if(string);

        assert_eq!(
            res,
            Ok((
                "",
                If {
                    cond: vec![
                        ExpressionTerm::Literal(Literal::Variable(Variable {
                            name: String::from("x")
                        })),
                        ExpressionTerm::Token(String::from("<")),
                        ExpressionTerm::Literal(Literal::Num(3.0))
                    ],
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
                    ],
                    else_statements: None
                }
            ))
        )
    }

    // TODO: Test with else stmts
}
