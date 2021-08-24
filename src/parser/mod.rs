mod declaration;
mod expression;
mod function;
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

pub type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq)]
pub struct Variable {
    name: String,
}

fn parse_variable(input: &str) -> Res<&str, Variable> {
    context(
        "Variable",
        // Keywords should not be parsed
        verify(
            recognize(pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
            // TODO: Check for all keywords
            |s: &str| s != "do",
        ),
    )(input)
    .map(|(next_input, res)| {
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
}
