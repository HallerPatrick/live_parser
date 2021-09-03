mod declaration;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::{fold_many0, many0, separated_list0};
use nom::sequence::{preceded, terminated};

use declaration::assignment::{parse_assignment, Assignment};
use declaration::class::{parse_class, Class};
use declaration::for_statement::{parse_for, For};
use declaration::function::{parse_function, Function};
use declaration::if_statement::{parse_if, If};
use declaration::while_statement::{parse_while, While};

use super::{
    expression::{parse_expression, Expression},
    literals::{parse_literal, sp, Literal},
    tokens::lreturn,
    Res,
};

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

// fn parse_return_stmt(input: &str) -> Res<&str, ReturnStmt> {
//     context(
//         "ReturnStmt",
//         preceded(preceded(sp, lreturn), preceded(sp, parse_return_list)),
//     )(input)
//     .map(|(next_input, res)| (next_input, ReturnStmt { values: res }))
// }

// fn parse_return_list(input: &str) -> Res<&str, Vec<Expression>> {
//     context(
//         "ReturnList",
//         preceded(
//             sp,
//             separated_list0(preceded(sp, char(',')), preceded(sp, token)),
//         ),
//     )(input)
// }

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

// #[cfg(test)]
// #[ignore]
// mod tests {

//     use super::*;
//     use crate::parser::expression::call::Call;
//     use crate::parser::expression::ExpressionTerm;
//     use crate::parser::literals::Literal;
//     use crate::parser::Variable;

//     #[test]
//     fn test_parse_statement_expr() {
//         let string = "x";
//         let res = parse_statement(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 Statement::Expression(vec![ExpressionTerm::Literal(Literal::Variable(Variable {
//                     name: String::from("x")
//                 }))])
//             ))
//         );
//     }

//     #[test]
//     fn test_parse_assignemnt() {
//         let string = " let x = 3";
//         let res = parse_statement(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 Statement::Assignment(Assignment {
//                     variable: Variable {
//                         name: String::from("x")
//                     },
//                     expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
//                 })
//             ))
//         );
//     }

//     #[test]
//     fn test_parse_assignemnt_line_ending() {
//         let string = " let x = 3\n\n";
//         let res = parse_statement(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 Statement::Assignment(Assignment {
//                     variable: Variable {
//                         name: String::from("x")
//                     },
//                     expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
//                 })
//             ))
//         );
//     }

//     #[test]
//     fn test_parse_assignemnt_line_ending_indented_newline() {
//         let string = "\n\t let x = 3\n\n";
//         let res = parse_statement(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 Statement::Assignment(Assignment {
//                     variable: Variable {
//                         name: String::from("x")
//                     },
//                     expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
//                 })
//             ))
//         );
//     }

//     #[test]
//     fn test_parse_statements() {
//         let string = "let x = 3\n let y = 3";
//         let res = parse_statement(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "let y = 3",
//                 Statement::Assignment(Assignment {
//                     variable: Variable {
//                         name: String::from("x")
//                     },
//                     expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
//                 })
//             ))
//         )
//     }

//     #[test]
//     fn test_parse_statements0() {
//         let string = " x + 1\n let y = 3";
//         let res = parse_statements(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 vec![
//                     Statement::Expression(vec![
//                         ExpressionTerm::Literal(Literal::Variable(Variable {
//                             name: String::from("x")
//                         })),
//                         ExpressionTerm::Token(String::from("+")),
//                         ExpressionTerm::Literal(Literal::Num(1.0))
//                     ]),
//                     Statement::Assignment(Assignment {
//                         variable: Variable {
//                             name: String::from("y")
//                         },
//                         expression: vec![ExpressionTerm::Literal(Literal::Num(3.0))]
//                     })
//                 ]
//             ))
//         );
//     }

//     #[test]
//     fn test_return_stmt() {
//         let string = "return hello, 3";
//         let res = parse_return_stmt(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 ReturnStmt {
//                     values: vec![
//                         ExpressionTerm::Literal(Literal::Variable(Variable {
//                             name: String::from("hello")
//                         })),
//                         ExpressionTerm::Literal(Literal::Num(3.0))
//                     ]
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn test_return_stmt_2() {
//         let string = "return hello, func(n)";
//         let res = parse_return_stmt(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 ReturnStmt {
//                     values: vec![
//                         ExpressionTerm::Literal(Literal::Variable(Variable {
//                             name: String::from("hello")
//                         })),
//                         ExpressionTerm::Call(Call {
//                             callee: Literal::Variable(Variable {
//                                 name: String::from("func")
//                             }),
//                             args: vec![
//                                 ExpressionTerm::Literal(Literal::Variable(Variable {
//                                     name: String::from("n")
//                                 }))
//                             ]
//                         })
//                     ]
//                 }
//             ))
//         )
//     }

//     // TODO: Make this work
//     #[test]
//     fn test_return_stmt_3() {
//         let string = "return fib(n-1)";
//         let res = parse_return_stmt(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 ReturnStmt {
//                     values: vec![
//                         ExpressionTerm::Literal(Literal::Variable(Variable {
//                             name: String::from("hello")
//                         })),
//                         ExpressionTerm::Call(Call {
//                             callee: Literal::Variable(Variable {
//                                 name: String::from("func")
//                             }),
//                             args: vec![
//                                 ExpressionTerm::Literal(Literal::Variable(Variable {
//                                     name: String::from("n")
//                                 }))
//                             ]
//                         })
//                     ]
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn test_return_stmt_1() {
//         let string = "return hello, 3\nend";
//         let res = parse_return_stmt(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "end",
//                 ReturnStmt {
//                     values: vec![
//                         ExpressionTerm::Literal(Literal::Variable(Variable {
//                             name: String::from("hello")
//                         })),
//                         ExpressionTerm::Literal(Literal::Num(3.0))
//                     ]
//                 }
//             ))
//         )
//     }
// }
