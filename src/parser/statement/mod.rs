mod declaration;
mod import;

use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::{fold_many0, many0, separated_list0};
use nom::sequence::{preceded, terminated, tuple};
use nom::Err;

use crate::parser::expression::{parse_expression, prefixexpr, ExprSuffix, PrefixExpr};
use crate::parser::literals::sp;
use crate::parser::tokens::{comma, lreturn};
use crate::parser::statement::import::{ parse_import, Import };
use declaration::assignment::{parse_assignment, Assignment};
use declaration::class::{parse_class, Class};
use declaration::for_statement::{parse_for, For};
use declaration::function::{parse_function, Function};
use declaration::if_statement::{parse_if, If};
use declaration::while_statement::{parse_while, While};

use super::{expression::Expression, Res};

#[derive(Debug, PartialEq)]
pub struct Block {
    statements: Vec<Statement>,
    return_stmt: Option<ReturnStmt>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment(Assignment),
    FuncCall(PrefixExpr),
    While(While),
    For(For),
    If(If),
    Fun(Function),
    Class(Class),
    Return(ReturnStmt),
    Expression(Expression),
    Import(Import),
}

pub fn parse_block(input: &str) -> Res<&str, Block> {
    context(
        "Block",
        tuple((many0(parse_statement), opt(parse_return_stmt))),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            Block {
                statements: res.0,
                return_stmt: res.1,
            },
        )
    })
}

pub fn parse_statement(input: &str) -> Res<&str, Statement> {
    context(
        "Stmt",
        terminated(
            preceded(
                opt_line_ending,
                preceded(
                    space0,
                    alt((
                        map(parse_assignment, Statement::Assignment),
                        map(parse_if, Statement::If),
                        map(parse_for, Statement::For),
                        map(parse_while, Statement::While),
                        map(parse_function, Statement::Fun),
                        map(parse_class, Statement::Class),
                        map(parse_function_call, Statement::FuncCall),
                        map(parse_import, Statement::Import),
                        // map(parse_return_stmt, Statement::Return),
                        // map(parse_expression, Statement::Expression),
                    )),
                ),
            ),
            opt_line_ending,
        ),
    )(input)
}

#[derive(Debug, PartialEq)]
pub struct ReturnStmt {
    pub values: Vec<Expression>,
}

fn parse_function_call(input: &str) -> Res<&str, PrefixExpr> {
    let func_call_expr = prefixexpr(input);

    let is_func_call = match func_call_expr {
        Ok((_, ref o)) => match o.suffix_chain.last().unwrap() {
            ExprSuffix::FuncCall(_) => true,
            _ => false,
        },
        _ => false,
    };

    if is_func_call {
        func_call_expr
    } else {
        Err(Err::Error(nom::error::VerboseError { errors: vec![] }))
    }
}

fn parse_return_stmt(input: &str) -> Res<&str, ReturnStmt> {
    context(
        "ReturnStmt",
        preceded(preceded(sp, lreturn), preceded(sp, parse_return_list)),
    )(input)
    .map(|(next_input, res)| (next_input, ReturnStmt { values: res }))
}

fn parse_return_list(input: &str) -> Res<&str, Vec<Expression>> {
    context(
        "ReturnList",
        preceded(
            sp,
            separated_list0(preceded(sp, comma), preceded(sp, parse_expression)),
        ),
    )(input)
}

pub fn opt_line_ending(input: &str) -> Res<&str, &str> {
    opt(many0(line_ending))(input).map(|(next_input, _)| (next_input, ""))
}

/// Statements are delimited by newlines
pub fn parse_statements(input: &str) -> Res<&str, Vec<Statement>> {
    context(
        "Stmts",
        fold_many0(parse_statement, Vec::new, |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        }),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::expression::binary::BinaryOp;
    use crate::parser::expression::call::Call;
    use crate::parser::expression::ExprOrVarname;
    use crate::parser::literals::Literal;
    use crate::parser::tokens::Operator;
    use crate::parser::Variable;

    #[test]
    fn test_parse_assignemnt() {
        let string = " let x = 3";
        let res = parse_statement(string);
        assert_eq!(
            res,
            Ok((
                "",
                Statement::Assignment(Assignment {
                    variable: Variable {
                        name: String::from("x")
                    },
                    expression: Expression::Literal(Literal::Num(3.0))
                })
            ))
        );
    }

    #[test]
    fn test_parse_assignemnt_line_ending() {
        let string = " let x = 3\n\n";
        let res = parse_statement(string);
        assert_eq!(
            res,
            Ok((
                "",
                Statement::Assignment(Assignment {
                    variable: Variable {
                        name: String::from("x")
                    },
                    expression: Expression::Literal(Literal::Num(3.0))
                })
            ))
        );
    }

    #[test]
    fn test_parse_assignemnt_line_ending_indented_newline() {
        let string = "\n\t let x = 3\n\n";
        let res = parse_statement(string);
        assert_eq!(
            res,
            Ok((
                "",
                Statement::Assignment(Assignment {
                    variable: Variable {
                        name: String::from("x")
                    },
                    expression: Expression::Literal(Literal::Num(3.0))
                })
            ))
        );
    }

    #[test]
    fn test_parse_statements() {
        let string = "let x = 3\n let y = 3";
        let res = parse_statement(string);
        assert_eq!(
            res,
            Ok((
                "let y = 3",
                Statement::Assignment(Assignment {
                    variable: Variable {
                        name: String::from("x")
                    },
                    expression: Expression::Literal(Literal::Num(3.0))
                })
            ))
        )
    }

    #[test]
    fn test_return_stmt() {
        let string = "return hello, 3";
        let res = parse_return_stmt(string);
        assert_eq!(
            res,
            Ok((
                "",
                ReturnStmt {
                    values: vec![
                        Expression::PrefixExpr(Box::new(PrefixExpr {
                            prefix: ExprOrVarname::Varname(Variable::new("hello")),
                            suffix_chain: vec![]
                        })),
                        Expression::Literal(Literal::Num(3.0))
                    ]
                }
            ))
        )
    }

    #[test]
    fn test_return_stmt_2() {
        let string = "return hello, func(n)";
        let res = parse_return_stmt(string);
        assert_eq!(
            res,
            Ok((
                "",
                ReturnStmt {
                    values: vec![
                        Expression::PrefixExpr(Box::new(PrefixExpr {
                            prefix: ExprOrVarname::Varname(Variable::new("hello")),
                            suffix_chain: vec![]
                        })),
                        Expression::PrefixExpr(Box::new(PrefixExpr {
                            prefix: ExprOrVarname::Varname(Variable::new("func")),
                            suffix_chain: vec![ExprSuffix::FuncCall(Call {
                                callee: None,
                                args: vec![Expression::PrefixExpr(Box::new(PrefixExpr {
                                    prefix: ExprOrVarname::Varname(Variable::new("n")),
                                    suffix_chain: vec![]
                                }))]
                            })]
                        }))
                    ]
                }
            ))
        )
    }

    #[test]
    fn test_return_stmt_3() {
        let string = "return fib(n-1)";
        let res = parse_return_stmt(string);
        assert_eq!(
            res,
            Ok((
                "",
                ReturnStmt {
                    values: vec![Expression::PrefixExpr(Box::new(PrefixExpr {
                        prefix: ExprOrVarname::Varname(Variable::new("fib")),
                        suffix_chain: vec![ExprSuffix::FuncCall(Call {
                            callee: None,
                            args: vec![Expression::BinaryOp(Box::new(BinaryOp {
                                op: Operator::Sub,
                                left: Expression::PrefixExpr(Box::new(PrefixExpr {
                                    prefix: ExprOrVarname::Varname(Variable::new("n")),
                                    suffix_chain: vec![]
                                })),
                                right: Expression::Literal(Literal::Num(1.0))
                            }))]
                        })]
                    }))]
                }
            ))
        )
    }
}
