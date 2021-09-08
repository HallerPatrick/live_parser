use crate::parser::{
    literals::{parse_variable, parse_variable_raw, sp, Variable},
    statement::opt_line_ending,
    statement::parse_block,
    statement::Block,
    tokens::{end, fun, left_paren, right_paren},
    Res,
};

use nom::{
    character::complete::char,
    error::context,
    multi::separated_list0,
    sequence::{delimited, preceded},
    sequence::{terminated, tuple},
};

/// Represents a function declaration.
#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Variable>,
    pub statements: Block,
}

/// Parses a function declaration.
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
    context(
        "ParameterList",
        preceded(
            sp,
            delimited(
                left_paren,
                separated_list0(preceded(sp, char(',')), preceded(sp, parse_variable)),
                preceded(sp, right_paren),
            ),
        ),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::expression::binary::BinaryOp;
    use crate::parser::expression::{ExprOrVarname, Expression, PrefixExpr};
    use crate::parser::literals::Literal;
    use crate::parser::statement::{If, LAssignment, ReturnStmt, Statement};
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
                        statements: vec![Statement::LAssignment(LAssignment {
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
    fn test_fun_return() {
        let string = "\n\n\tfun fib(n)\n\t\treturn 'Hello World'\n\tend";
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
                        return_stmt: Some(ReturnStmt {
                            values: vec![Expression::Literal(Literal::Str(String::from(
                                "Hello World"
                            )))],
                        })
                    }
                }
            ))
        );
    }

    #[test]
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

    #[test]
    fn test_fun_2() {
        let string = "fun init(self, height, weight)\r\n       self.height = height\r\n       self.weight = weight \r\n    end\r\n\r\n     ";
        let res = parse_function(string);
        // println!("{:?}", res);
    }
}
