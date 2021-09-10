use crate::parser::expression::parse_expression;
use crate::parser::expression::Expression;
use crate::parser::literals::sp;
use crate::parser::tokens::{end, ldo, lelse, lif};
use crate::parser::{Res, Span};

use crate::parser::statement::{parse_block, Block};

use nom::{
    combinator::opt,
    error::context,
    sequence::{delimited, preceded, terminated, tuple},
};

#[derive(Debug, PartialEq)]
pub struct If<'a> {
    pub cond: Expression<'a>,
    pub stmts: Block<'a>,
    pub else_statements: Option<Block<'a>>,
}

pub fn parse_if(input: Span) -> Res<If> {
    context(
        "If",
        tuple((
            if_condition,
            parse_block,
            terminated(opt(else_statements), preceded(sp, end)),
        )),
    )(input)
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

fn else_statements(input: Span) -> Res<Block> {
    context("ElseStmt", preceded(preceded(sp, lelse), parse_block))(input)
}

fn if_condition(input: Span) -> Res<Expression> {
    context(
        "IfCond",
        delimited(
            preceded(sp, lif),
            preceded(sp, parse_expression),
            preceded(sp, ldo),
        ),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::literals::Token;
    use crate::parser::expression::Expression;
    use crate::parser::expression::{binary::BinaryOp, ExprOrVarname, PrefixExpr};
    use crate::parser::literals::{Literal, Variable};
    use crate::parser::statement::declaration::assignment::LAssignment;
    use crate::parser::statement::{ReturnStmt, Statement};
    use crate::parser::tokens::Operator;

    #[test]
    fn parse_if_condition() {
        let string = "if x < 3 do";
        let (_, res) = if_condition(Span::new(string)).unwrap();

        assert_eq!(
            res,
            Expression::BinaryOp(Box::new(BinaryOp {
                left: Expression::PrefixExpr(Box::new(PrefixExpr {
                    prefix: ExprOrVarname::Varname(Literal::Variable(Token::new(
                        Variable::new("x"),
                        Span::new("x")
                    ))),
                    suffix_chain: vec![],
                })),
                op: Operator::Lt,
                right: Expression::Literal(Literal::Num(Token::new(3.0, Span::new("3")))),
            }))
        )
    }

    #[test]
    fn parse_if_statements() {
        let string = "if x < 3 do let z = x + 3\n let y = 3 \nend";
        let (_, res) = parse_if(Span::new(string)).unwrap();

        assert_eq!(
            res,
            If {
                cond: Expression::BinaryOp(Box::new(BinaryOp {
                    left: Expression::PrefixExpr(Box::new(PrefixExpr {
                        prefix: ExprOrVarname::Varname(Literal::Variable(Token::new(
                            Variable::new("x"),
                            Span::new("x")
                        ))),
                        suffix_chain: vec![],
                    })),
                    op: Operator::Lt,
                    right: Expression::Literal(Literal::Num(Token::new(3.0, Span::new("3")))),
                })),
                stmts: Block {
                    statements: vec![
                        Statement::LAssignment(LAssignment {
                            variable: Literal::Variable(Token::new(
                                Variable::new("z"),
                                Span::new("z")
                            )),
                            expression: Expression::BinaryOp(Box::new(BinaryOp {
                                left: Expression::PrefixExpr(Box::new(PrefixExpr {
                                    prefix: ExprOrVarname::Varname(Literal::Variable(Token::new(
                                        Variable::new("x"),
                                        Span::new("x")
                                    ))),
                                    suffix_chain: vec![],
                                })),
                                op: Operator::Add,
                                right: Expression::Literal(Literal::Num(Token::new(
                                    3.0,
                                    Span::new("3")
                                ))),
                            })),
                        }),
                        Statement::LAssignment(LAssignment {
                            variable: Literal::Variable(Token::new(
                                Variable::new("y"),
                                Span::new("y")
                            )),
                            expression: Expression::Literal(Literal::Num(Token::new(
                                3.0,
                                Span::new("3")
                            ))),
                        }),
                    ],
                    return_stmt: None,
                },
                else_statements: None
            }
        )
    }

    #[test]
    fn parse_if_statements_with_return() {
        let string = "if x < 3 do return 0 \nend";
        let (_, res) = parse_if(Span::new(string)).unwrap();

        assert_eq!(
            res,
            If {
                cond: Expression::BinaryOp(Box::new(BinaryOp {
                    left: Expression::PrefixExpr(Box::new(PrefixExpr {
                        prefix: ExprOrVarname::Varname(Literal::Variable(Token::new(
                            Variable::new("x"),
                            Span::new("x")
                        ))),
                        suffix_chain: vec![],
                    })),
                    op: Operator::Lt,
                    right: Expression::Literal(Literal::Num(Token::new(3.0, Span::new("3")))),
                })),
                stmts: Block {
                    statements: vec![],
                    return_stmt: Some(ReturnStmt {
                        values: vec![Expression::Literal(Literal::Num(Token::new(
                            0.0,
                            Span::new("0")
                        )))],
                    }),
                },
                else_statements: None
            }
        )
    }
}
