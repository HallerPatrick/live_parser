use nom::{
    error::context,
    sequence::{delimited, preceded, terminated, tuple},
};

use crate::parser::expression::{parse_expression, Expression};
use crate::parser::literals::sp;
use crate::parser::literals::{parse_variable, Variable};
use crate::parser::statement::{parse_block, Block};
use crate::parser::tokens::{end, ldo, lfor, lin};
use crate::parser::Res;

#[derive(Debug, PartialEq)]
pub struct For {
    iter_item: Variable,
    iterator: Expression,
    block: Block,
}

pub fn parse_for(input: &str) -> Res<&str, For> {
    tuple((
        parse_iter_item,
        parse_iterator,
        terminated(parse_block, end),
    ))(input)
    .map(|(next_input, res)| {
        (
            next_input,
            For {
                iter_item: res.0,
                iterator: res.1,
                block: res.2,
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

fn parse_iterator(input: &str) -> Res<&str, Expression> {
    context(
        "ForIterator",
        delimited(
            preceded(sp, lin),
            preceded(sp, parse_expression),
            preceded(sp, ldo),
        ),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::expression::{ExprOrVarname, Expression, PrefixExpr};
    use crate::parser::literals::Literal;
    use crate::parser::statement::{Block, LAssignment, Statement};

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
                Expression::PrefixExpr(Box::new(PrefixExpr {
                    prefix: ExprOrVarname::Varname(Variable::new("iterator")),
                    suffix_chain: vec![]
                }))
            ))
        );
    }

    #[test]
    fn test_parse_for_loop() {
        let string = "for x in y do \n let y = 3 \nend";
        let res = parse_for(string);

        assert_eq!(
            res,
            Ok((
                "",
                For {
                    iter_item: Variable {
                        name: String::from("x")
                    },
                    iterator: Expression::PrefixExpr(Box::new(PrefixExpr {
                        prefix: ExprOrVarname::Varname(Variable::new("y")),
                        suffix_chain: vec![]
                    })),
                    block: Block {
                        statements: vec![Statement::LAssignment(LAssignment {
                            variable: Variable {
                                name: String::from("y")
                            },
                            expression: Expression::Literal(Literal::Num(3.0))
                        })],
                        return_stmt: None
                    }
                }
            ))
        )
    }
}
