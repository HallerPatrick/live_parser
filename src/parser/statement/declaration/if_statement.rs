use crate::parser::expression::parse_expression;
use crate::parser::expression::Expression;
use crate::parser::literals::sp;
use crate::parser::tokens::{end, ldo, lelse, lif};
use crate::parser::Res;

use crate::parser::statement::{parse_block, Block};

use nom::{
    combinator::opt,
    error::context,
    sequence::{delimited, preceded, terminated, tuple},
};

#[derive(Debug, PartialEq)]
pub struct If {
    pub cond: Expression,
    pub stmts: Block,
    pub else_statements: Option<Block>,
}

pub fn parse_if(input: &str) -> Res<&str, If> {
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

fn else_statements(input: &str) -> Res<&str, Block> {
    context("ElseStmt", preceded(preceded(sp, lelse), parse_block))(input)
}

fn if_condition(input: &str) -> Res<&str, Expression> {
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

    use crate::parser::expression::Expression;
    use crate::parser::expression::{binary::BinaryOp, ExprOrVarname, PrefixExpr};
    use crate::parser::literals::{Literal, Variable};
    use crate::parser::statement::declaration::assignment::LAssignment;
    use crate::parser::statement::{ReturnStmt, Statement};
    use crate::parser::tokens::Operator;

    #[test]
    fn parse_if_condition() {
        let string = "if x < 3 do";
        let res = if_condition(string);

        assert_eq!(
            res,
            Ok((
                "",
                Expression::BinaryOp(Box::new(BinaryOp {
                    left: Expression::PrefixExpr(Box::new(PrefixExpr {
                        prefix: ExprOrVarname::Varname(Variable::new("x")),
                        suffix_chain: vec![],
                    })),
                    op: Operator::Lt,
                    right: Expression::Literal(Literal::Num(3.0)),
                }))
            ))
        )
    }

    #[test]
    fn parse_if_statements() {
        let string = "if x < 3 do let z = x + 3\n let y = 3 \nend";
        let res = parse_if(string);

        assert_eq!(
            res,
            Ok((
                "",
                If {
                    cond: Expression::BinaryOp(Box::new(BinaryOp {
                        left: Expression::PrefixExpr(Box::new(PrefixExpr {
                            prefix: ExprOrVarname::Varname(Variable::new("x")),
                            suffix_chain: vec![],
                        })),
                        op: Operator::Lt,
                        right: Expression::Literal(Literal::Num(3.0)),
                    })),
                    stmts: Block {
                        statements: vec![
                            Statement::LAssignment(LAssignment {
                                variable: Variable::new("z"),
                                expression: Expression::BinaryOp(Box::new(BinaryOp {
                                    left: Expression::PrefixExpr(Box::new(PrefixExpr {
                                        prefix: ExprOrVarname::Varname(Variable::new("x")),
                                        suffix_chain: vec![],
                                    })),
                                    op: Operator::Add,
                                    right: Expression::Literal(Literal::Num(3.0)),
                                })),
                            }),
                            Statement::LAssignment(LAssignment {
                                variable: Variable::new("y"),
                                expression: Expression::Literal(Literal::Num(3.0)),
                            }),
                        ],
                        return_stmt: None,
                    },
                    else_statements: None
                }
            ))
        )
    }

    #[test]
    fn parse_if_statements_with_return() {
        let string = "if x < 3 do return 0 \nend";
        let res = parse_if(string);

        assert_eq!(
            res,
            Ok((
                "",
                If {
                    cond: Expression::BinaryOp(Box::new(BinaryOp {
                        left: Expression::PrefixExpr(Box::new(PrefixExpr {
                            prefix: ExprOrVarname::Varname(Variable::new("x")),
                            suffix_chain: vec![],
                        })),
                        op: Operator::Lt,
                        right: Expression::Literal(Literal::Num(3.0)),
                    })),
                    stmts: Block {
                        statements: vec![],
                        return_stmt: Some(ReturnStmt {
                            values: vec![Expression::Literal(Literal::Num(0.0))],
                        }),
                    },
                    else_statements: None
                }
            ))
        )
    }
}
