use crate::parser::{
    literals::sp,
    parse_variable_raw,
    statement::opt_line_ending,
    statement::parse_block,
    statement::{declaration::parse_parameter_list, Block, Statement},
    tokens::{end, fun},
    Res, Variable,
};

use nom::{
    error::context,
    sequence::preceded,
    sequence::{terminated, tuple},
};

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Variable>,
    pub statements: Block,
}

pub fn parse_function(input: &str) -> Res<&str, Function> {
    context(
        "Func",
        terminated(
            tuple((parse_function_name, parse_function_arguments, parse_block)),
            preceded(sp, end),
        ),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            Function {
                name: String::from(res.0),
                parameters: res.1,
                statements: res.2,
            },
        )
    })
}

fn parse_function_name(input: &str) -> Res<&str, &str> {
    context(
        "FuncName",
        preceded(
            opt_line_ending,
            preceded(sp, preceded(fun, preceded(sp, parse_variable_raw))),
        ),
    )(input)
}

fn parse_function_arguments(input: &str) -> Res<&str, Vec<Variable>> {
    context("FuncArguments", parse_parameter_list)(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::expression::binary::BinaryOp;
    use crate::parser::expression::{ExprOrVarname, Expression, PrefixExpr};
    use crate::parser::literals::Literal;
    use crate::parser::statement::{Assignment, If, ReturnStmt};
    use crate::parser::tokens::Operator;

    #[test]
    fn test_parse_function_name() {
        let string = "fun hello ()";
        let res = parse_function_name(string);
        assert_eq!(res, Ok((" ()", "hello")));
    }

    #[test]
    fn test_parse_function() {
        let string = "fun hello(x, y)\n\tlet some = \"1\"\nend";
        let res = parse_function(string);

        assert_eq!(
            res,
            Ok((
                "",
                Function {
                    name: String::from("hello"),
                    parameters: vec![
                        Variable {
                            name: String::from("x")
                        },
                        Variable {
                            name: String::from("y")
                        }
                    ],
                    statements: Block {
                        statements: vec![Statement::Assignment(Assignment {
                            variable: Variable {
                                name: String::from("some")
                            },
                            expression: Expression::Literal(Literal::Str(String::from("1")))
                        })],
                        return_stmt: None
                    }
                }
            ))
        );
    }

    #[test]
    fn test_fun_1() {
        let string = "fun fib(n)\n\nend";
        let res = parse_function(string);
        assert_eq!(
            res,
            Ok((
                "",
                Function {
                    name: String::from("fib"),
                    parameters: vec![Variable {
                        name: String::from("n")
                    }],
                    statements: Block {
                        statements: vec![],
                        return_stmt: None
                    }
                }
            ))
        );
    }

    #[test]
    fn test_fun() {
        let string = "fun fib(n)\n    if n == 0  do\n        return 0\n    end\nend";
        let res = parse_function(string);

        assert_eq!(
            res,
            Ok((
                "",
                Function {
                    name: String::from("fib"),
                    parameters: vec![Variable {
                        name: String::from("n")
                    }],
                    statements: Block {
                        statements: vec![Statement::If(If {
                            cond: Expression::BinaryOp(Box::new(BinaryOp {
                                left: Expression::PrefixExpr(Box::new(PrefixExpr {
                                    prefix: ExprOrVarname::Varname(Variable {
                                        name: String::from("n")
                                    }),
                                    suffix_chain: vec![]
                                })),
                                op: Operator::EQ,
                                right: Expression::Literal(Literal::Num(0.0))
                            })),
                            stmts: Block {
                                statements: vec![],
                                return_stmt: Some(ReturnStmt {
                                    values: vec![Expression::Literal(Literal::Num(0.0))]
                                })
                            },
                            else_statements: None
                        })],
                        return_stmt: None
                    }
                }
            ))
        )
    }
}
