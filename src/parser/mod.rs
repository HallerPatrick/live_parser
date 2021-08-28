#![allow(dead_code)]

mod declaration;
mod expression;
mod literals;
mod statement;
mod tokens;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{recognize, verify},
    error::{context, VerboseError},
    multi::many0,
    sequence::pair,
    IResult,
};

use crate::parser::{ tokens::KEYWORDS, statement::{ parse_statements, Statement } };

pub type Res<T, U> = IResult<T, U, VerboseError<T>>;


pub fn parse_source(input: &str) -> Res<&str, Vec<Statement>> {
    context(
        "Root",
        parse_statements
    )(input)
}


#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    name: String,
}

fn parse_variable_raw(input: &str) -> Res<&str, &str> {
    context(
        "Variable",
        // Keywords should not be parsed
        verify(
            recognize(pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
            |s: &str| !KEYWORDS.contains(&s),
        ),
    )(input)
}

fn parse_variable(input: &str) -> Res<&str, Variable> {
    parse_variable_raw(input).map(|(next_input, res)| {
        (
            next_input,
            Variable {
                name: String::from(res),
            },
        )
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_variable() {
        let string = "hello123";
        let res = parse_variable(string);
        assert_eq!(
            res,
            Ok((
                "",
                Variable {
                    name: String::from("hello123")
                }
            ))
        );
    }

    #[test]
    fn test_parse_variable_cap() {
        let string = "Hello123";
        let res = parse_variable(string);
        assert_eq!(
            res,
            Ok((
                "",
                Variable {
                    name: String::from("Hello123")
                }
            ))
        );
    }

    #[test]
    fn test_parse_variable_func() {
        let string = "hello()";
        let res = parse_variable(string);
        assert_eq!(
            res,
            Ok((
                "()",
                Variable {
                    name: String::from("hello")
                }
            ))
        );
    }
}
