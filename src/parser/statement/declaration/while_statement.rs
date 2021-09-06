use crate::parser::expression::parse_expression;
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
            preceded(sp, parse_expression),
            preceded(sp, ldo),
        ),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::parser::expression::binary::BinaryOp;
    use crate::parser::expression::{ExprOrVarname, Expression, PrefixExpr};
    use crate::parser::literals::Literal;
    use crate::parser::statement::declaration::assignment::Assignment;
    use crate::parser::tokens::Operator;
    use crate::parser::Variable;

    #[test]
    fn parse_while_condition() {
        let string = "while x < 3 do";
        let res = while_condition(string);

        assert_eq!(
            res,
            Ok((
                "",
                Expression::BinaryOp(Box::new(BinaryOp {
                    op: Operator::Lt,
                    left: Expression::PrefixExpr(Box::new(PrefixExpr {
                        prefix: ExprOrVarname::Varname(Variable::new("x")),
                        suffix_chain: vec![],
                    })),
                    right: Expression::Literal(Literal::Num(3.0)),
                }))
            ))
        )
    }

    #[test]
    fn parse_while_statement() {
        let string = "while x < 3 do let z = x + 3\n let y = 3 \nend";
        let res = parse_while(string);

        assert_eq!(
            res,
            Ok((
                "",
                While {
                    cond: Expression::BinaryOp(Box::new(BinaryOp {
                        op: Operator::Lt,
                        left: Expression::PrefixExpr(Box::new(PrefixExpr {
                            prefix: ExprOrVarname::Varname(Variable::new("x")),
                            suffix_chain: vec![],
                        })),
                        right: Expression::Literal(Literal::Num(3.0)),
                    })),
                    stmts: vec![
                        Statement::Assignment(Assignment {
                            variable: Variable::new("z"),
                            expression: Expression::BinaryOp(Box::new(BinaryOp {
                                op: Operator::Add,
                                left: Expression::PrefixExpr(Box::new(PrefixExpr {
                                    prefix: ExprOrVarname::Varname(Variable::new("x")),
                                    suffix_chain: vec![],
                                })),
                                right: Expression::Literal(Literal::Num(3.0)),
                            })),
                        }),
                        Statement::Assignment(Assignment {
                            variable: Variable::new("y"),
                            expression: Expression::Literal(Literal::Num(3.0)),
                        }),
                    ]
                }
            ))
        )
    }
}
