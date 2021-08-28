use crate::parser::expression::expression_lexer;
use crate::parser::expression::Expression;
use crate::parser::literals::sp;
use crate::parser::tokens::{end, ldo, lwhile};
use crate::parser::Res;

use crate::parser::statement::{parse_statements, Statement};

use nom::{
    error::context,
    sequence::{delimited, preceded, terminated, tuple},
};

#[derive(Debug, PartialEq)]
pub struct While {
    cond: Expression,
    stmts: Vec<Statement>,
}

pub fn parse_while(input: &str) -> Res<&str, While> {
    tuple((while_condition, terminated(parse_statements, end)))(input).map(|(next_input, res)| {
        (
            next_input,
            While {
                cond: res.0,
                stmts: res.1,
            },
        )
    })
}

fn while_condition(input: &str) -> Res<&str, Expression> {
    context(
        "WhileCond",
        delimited(
            preceded(sp, lwhile),
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
    fn parse_while_condition() {
        let string = "while x < 3 do";
        let res = while_condition(string);

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
    fn parse_while_statement() {
        let string = "while x < 3 do x + 3\n let y = 3 \nend";
        let res = parse_while(string);

        assert_eq!(
            res,
            Ok((
                "",
                While {
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
                    ]
                }
            ))
        )
    }
}
