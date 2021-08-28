mod declaration;

use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::{fold_many0, many0, separated_list0};
use nom::sequence::{preceded, terminated};

use declaration::assignment::{parse_assignment, Assignment};
use declaration::for_statement::{parse_for, For};
use declaration::function::{parse_function, Function};
use declaration::if_statement::{parse_if, If};
use declaration::while_statement::{parse_while, While};
use declaration::class::{parse_class, Class};

use super::{ expression::{expression_lexer, Expression}, Res, literals::{ Literal, sp, parse_literal },  tokens::lreturn};


#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment(Assignment),
    While(While),
    For(For),
    If(If),
    Fun(Function),
    Class(Class),
    Return(ReturnStmt),
    Expression(Expression),
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
                        map(parse_return_stmt, Statement::Return),
                        map(expression_lexer, Statement::Expression),
                    )),
                ),
            ),
            opt_line_ending,
        ),
    )(input)
}

#[derive(Debug, PartialEq)]
pub struct ReturnStmt {
   pub values: Vec<Literal>
}

fn parse_return_stmt(input: &str) -> Res<&str, ReturnStmt>{
   context(
       "ReturnStmt",
       preceded(
           preceded(sp, lreturn),
           preceded(sp, parse_return_list)
           )
       )(input).map(|(next_input, res)| {
       (next_input, ReturnStmt { values: res })
   })
}

fn parse_return_list(input: &str)-> Res<&str, Vec<Literal>> {
    context(
        "ReturnList",
        preceded(sp,
                 separated_list0(
                     preceded(sp, char(',')),
                     preceded(sp, parse_literal),
                     )
                 )
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
    use crate::parser::expression::ExpressionTerm;
    use crate::parser::literals::Literal;
    use crate::parser::Variable;

    #[test]
    fn test_parse_statement_expr() {
        let string = "x";
        let res = parse_statement(string);
        assert_eq!(
            res,
            Ok((
                "",
                Statement::Expression(vec![ExpressionTerm::Literal(Literal::Variable(Variable {
                    name: String::from("x")
                }))])
            ))
        );
    }

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
                    expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
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
                    expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
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
                    expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
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
                " let y = 3",
                Statement::Assignment(Assignment {
                    variable: Variable {
                        name: String::from("x")
                    },
                    expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
                })
            ))
        )
    }

    #[test]
    fn test_parse_statements0() {
        let string = " x + 1\n let y = 3";
        let res = parse_statements(string);
        assert_eq!(
            res,
            Ok((
                "",
                vec![
                    Statement::Expression(vec![
                        ExpressionTerm::Literal(Literal::Variable(Variable {
                            name: String::from("x")
                        })),
                        ExpressionTerm::Token(String::from("+")),
                        ExpressionTerm::Literal(Literal::Num(1.0))
                    ]),
                    Statement::Assignment(Assignment {
                        variable: Variable {
                            name: String::from("y")
                        },
                        expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
                    })
                ]
            ))
        );
    }

    #[test]
    fn test_return_stmt()
    {
        let string = "return hello, 3";
        let res = parse_return_stmt(string);
        assert_eq!(res, Ok(("", ReturnStmt {
            values: vec![
                Literal::Variable(Variable {
                    name: String::from("hello")
                }),
                Literal::Num(3.0)
            ]
        })))
    }

    #[test]
    fn test() {
        let string = "method.call()";
        let res = parse_statement(string);
        println!("{:?}", res);
    }
   
}



