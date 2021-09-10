use crate::parser::{
    literals::{parse_variable, parse_variable_raw, sp, Variable},
    statement::opt_line_ending,
    statement::parse_block,
    statement::Block,
    tokens::{end, fun, left_paren, right_paren},
    Res, Span,
};

use crate::literals::Literal;
use nom::{
    character::complete::char,
    error::context,
    multi::separated_list0,
    sequence::{delimited, preceded},
    sequence::{terminated, tuple},
};

/// Represents a function declaration.
#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    pub name: Literal<'a>,
    pub parameters: Vec<Literal<'a>>,
    pub statements: Block<'a>,
}

/// Parses a function declaration.
pub fn parse_function(input: Span) -> Res<Function> {
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
                name: res.0,
                parameters: res.1,
                statements: res.2,
            },
        )
    })
}

fn parse_function_name(input: Span) -> Res<Literal> {
    context(
        "FuncName",
        preceded(
            opt_line_ending,
            preceded(sp, preceded(fun, preceded(sp, parse_variable))),
        ),
    )(input)
}

fn parse_function_arguments<'a>(input: Span<'a>) -> Res<Vec<Literal<'a>>> {
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
    use crate::literals::Token;
    use crate::parser::expression::binary::BinaryOp;
    use crate::parser::expression::{ExprOrVarname, Expression, PrefixExpr};
    use crate::parser::literals::Literal;
    use crate::parser::statement::{If, LAssignment, ReturnStmt, Statement};
    use crate::parser::tokens::Operator;

    #[test]
    fn test_parse_function_name() {
        let string = "fun hello ()";
        let (_, res) = parse_function_name(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Literal::Variable(Token::new(Variable::new("hello"), Span::new("hello")))
        );
    }

    #[test]
    fn test_parse_function() {
        let string = "fun hello(x, y)\n\tlet some = \"1\"\nend";
        let (_, res) = parse_function(Span::new(string)).unwrap();

        assert_eq!(
            res,
            Function {
                name: Literal::Variable(Token::new(Variable::new("hello"), Span::new("hello"))),
                parameters: vec![
                    Literal::Variable(Token::new(Variable::new("x"), Span::new("x"))),
                    Literal::Variable(Token::new(Variable::new("y"), Span::new("y")))
                ],
                statements: Block {
                    statements: vec![Statement::LAssignment(LAssignment {
                        variable: Literal::Variable(Token::new(
                            Variable::new("some"),
                            Span::new("some")
                        )),
                        expression: Expression::Literal(Literal::Str(Token::new(
                            String::from("1"),
                            Span::new("1")
                        )))
                    })],
                    return_stmt: None
                }
            }
        );
    }

    #[test]
    fn test_fun_1() {
        let string = "fun fib(n)\n\nend";
        let (_, res) = parse_function(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Function {
                name: Literal::Variable(Token::new(Variable::new("fib"), Span::new("fib"))),
                parameters: vec![Literal::Variable(Token::new(
                    Variable::new("n"),
                    Span::new("n")
                ))],
                statements: Block {
                    statements: vec![],
                    return_stmt: None
                }
            }
        );
    }

    #[test]
    fn test_fun_return() {
        let string = "\n\n\tfun fib(n)\n\t\treturn 'Hello World'\n\tend";
        let (_, res) = parse_function(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Function {
                name: Literal::Variable(Token::new(Variable::new("fib"), Span::new("fib"))),
                parameters: vec![Literal::Variable(Token::new(
                    Variable::new("n"),
                    Span::new("n")
                ))],
                statements: Block {
                    statements: vec![],
                    return_stmt: Some(ReturnStmt {
                        values: vec![Expression::Literal(Literal::Str(Token::new(
                            String::from("Hello World"),
                            Span::new("Hello World")
                        )))],
                    })
                }
            }
        );
    }

    #[test]
    #[test]
    fn test_fun() {
        let string = "fun fib(n)\n    if n == 0  do\n        return 0\n    end\nend";
        let (_, res) = parse_function(Span::new(string)).unwrap();

        assert_eq!(
            res,
            Function {
                name: Literal::Variable(Token::new(Variable::new("fib"), Span::new("fib"))),
                parameters: vec![Literal::Variable(Token::new(
                    Variable::new("n"),
                    Span::new("n")
                ))],
                statements: Block {
                    statements: vec![Statement::If(If {
                        cond: Expression::BinaryOp(Box::new(BinaryOp {
                            left: Expression::PrefixExpr(Box::new(PrefixExpr {
                                prefix: ExprOrVarname::Varname(Literal::Variable(Token::new(
                                    Variable::new("n"),
                                    Span::new("n")
                                ))),
                                suffix_chain: vec![]
                            })),
                            op: Operator::EQ,
                            right: Expression::Literal(Literal::Num(Token::new(
                                0.0,
                                Span::new("0")
                            )))
                        })),
                        stmts: Block {
                            statements: vec![],
                            return_stmt: Some(ReturnStmt {
                                values: vec![Expression::Literal(Literal::Num(Token::new(
                                    0.0,
                                    Span::new("0")
                                )))]
                            })
                        },
                        else_statements: None
                    })],
                    return_stmt: None
                }
            }
        )
    }

    #[test]
    fn test_fun_2() {
        let string = "fun init(self, height, weight)\r\n       self.height = height\r\n       self.weight = weight \r\n    end\r\n\r\n     ";
        let (_, res) = parse_function(Span::new(string)).unwrap();
        // println!("{:?}", res);
    }
}
