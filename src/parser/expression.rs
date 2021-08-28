use crate::parser::{
    literals::{parse_literal, Literal, sp},
    parse_variable,
    tokens::parse_tokens,
    Res, Variable
};

use nom::{
    branch::alt, character::complete::space0 as space, combinator::map, multi::fold_many1,
    sequence::delimited,
};

// #[derive(Debug, PartialEq)]
// pub struct Expression {}

// pub fn parse_expression(input: &str) -> Res<&str, Expression> {
//     Ok(("", Expression {}))
// }

// An expression can be everything which yiealds a "result"
// E.g:
// 1 + 1 -> Arithemtic
// 1 * 2 + 1
// foo.bar() -> Method calls
// baz()     -> Function Calls
// Class::foo() -> Function calls of classes (static methods)
//
// Parenthese show order of evaluation
// 1 * (1 + 2)
//
// For now expressions are only lexed
// where only literals are being parsed but not evaluated

// ParseFn::None => self.error("Expected expression"),
// ParseFn::Binary => self.binary(),
// ParseFn::Grouping => self.grouping(),
// ParseFn::Unary => self.unary(),
// ParseFn::Number => self.number(),
// ParseFn::Literal => self.literal(),
// ParseFn::String => self.string(),
// ParseFn::Variable => self.variable(can_assign),
// ParseFn::And => self.and_operator(),
// ParseFn::Or => self.or_operator(),
// ParseFn::Call => self.call(),
// ParseFn::Dot => self.dot(can_assign),
// ParseFn::This => self.this(),
// ParseFn::Super => self.super_(),
//
//
// For now a expression is only a list of tokens
pub type Expression = Vec<ExpressionTerm>;

// Represent a token or literal in a expression
#[derive(Debug, PartialEq)]
pub enum ExpressionTerm {
    Token(String),
    Literal(Literal),
    // Variable(Variable),
    Keyword(String),
}

pub fn expression_lexer(input: &str) -> Res<&str, Expression> {
    fold_many1(token, Vec::new, |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(input)
}

fn token(input: &str) -> Res<&str, ExpressionTerm> {
    delimited(
        sp,
        alt((
            // map(parse_keywords, expr_keyword),
            map(parse_tokens, expr_token),
            map(parse_literal, expr_literal),
            // map(parse_variable, expr_variable),
        )),
        space,
    )(input)
}

fn expr_keyword(input: &str) -> ExpressionTerm {
    ExpressionTerm::Keyword(String::from(input))
}

fn expr_token(input: &str) -> ExpressionTerm {
    ExpressionTerm::Token(String::from(input))
}

fn expr_literal(literal: Literal) -> ExpressionTerm {
    ExpressionTerm::Literal(literal)
}

// fn expr_variable(variable: Variable) -> ExpressionTerm {
//     ExpressionTerm::Variable(variable)
// }

// https://github.com/Geal/nom/blob/master/tests/arithmetic.rs
//
//
#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn test_expr_lexer5() {
    //     let string = "";
    //     let res = expression_lexer(string);
    //     assert_eq!(
    //         res,
    //         Ok((
    //             "",
    //             vec![
    //             ]
    //         ))
    //     )
    // }

    #[test]
    fn test_expr_lexer4() {
        let string = "()";
        let res = expression_lexer(string);
        assert_eq!(
            res,
            Ok((
                "",
                vec![
                    ExpressionTerm::Token(String::from("(")),
                    ExpressionTerm::Token(String::from(")"))
                ]
            ))
        )
    }

    #[test]
    fn test_expr_lexer3() {
        let string = "3 < x";
        let res = expression_lexer(string);
        assert_eq!(
            res,
            Ok((
                "",
                vec![
                    ExpressionTerm::Literal(Literal::Num(3.0)),
                    ExpressionTerm::Token(String::from("<")),
                    ExpressionTerm::Literal(Literal::Variable(Variable {
                        name: String::from("x")
                    })),
                ]
            ))
        )
    }

    #[test]
    fn test_expr_lexer2() {
        let string = "(3 + 4)";
        let res = expression_lexer(string);
        assert_eq!(
            res,
            Ok((
                "",
                vec![
                    ExpressionTerm::Token(String::from("(")),
                    ExpressionTerm::Literal(Literal::Num(3.0)),
                    ExpressionTerm::Token(String::from("+")),
                    ExpressionTerm::Literal(Literal::Num(4.0)),
                    ExpressionTerm::Token(String::from(")"))
                ]
            ))
        )
    }

    #[test]
    fn test_expr_lexer1() {
        let string = "(3 + x)";
        let res = expression_lexer(string);
        assert_eq!(
            res,
            Ok((
                "",
                vec![
                    ExpressionTerm::Token(String::from("(")),
                    ExpressionTerm::Literal(Literal::Num(3.0)),
                    ExpressionTerm::Token(String::from("+")),
                    ExpressionTerm::Literal(Literal::Variable(Variable {
                        name: String::from("x")
                    })),
                    ExpressionTerm::Token(String::from(")"))
                ]
            ))
        )
    }

        
    // TODO: FIx this 
    #[test]
    fn test_expr_lexer0() {
        let string = "method.call()";
        let res = parse_statement(string);
        println!("{:?}", res);
    }
}
