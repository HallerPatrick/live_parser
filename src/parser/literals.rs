use std::collections::HashMap;

use crate::parser::{parse_variable, Res, Variable};
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag_no_case, take_while},
    character::complete::char,
    character::complete::{alphanumeric1 as alphanumeric, one_of, space1},
    combinator::{cut, map},
    error::context,
    multi::separated_list0,
    number::complete::double,
    sequence::{preceded, separated_pair, terminated},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Boolean(bool),
    Nil,
    Num(f64),
    Array(Vec<Literal>),
    Map(HashMap<String, Literal>),
    Variable(Variable),
}

pub fn sp(input: &str) -> Res<&str, &str> {
    let chars = " \t\r\n";
    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c))(input)
}

fn alphanumeric_ws(input: &str) -> Res<&str, &str> {
    alt((space1, alphanumeric))(input)
}

fn parse_string(input: &str) -> Res<&str, &str> {
    escaped(alphanumeric_ws, '\\', one_of("\"n\\"))(input)
}

fn parse_str_raw(input: &str) -> Res<&str, &str> {
    context(
        "String",
        preceded(char('\"'), cut(terminated(parse_string, char('\"')))),
    )(input)
}

fn parse_str(input: &str) -> Res<&str, Literal> {
    parse_str_raw(input).map(|(next_input, res)| (next_input, Literal::Str(String::from(res))))
}

fn parse_num(input: &str) -> Res<&str, Literal> {
    context("Num", double)(input).map(|(next_input, res)| (next_input, Literal::Num(res)))
}

fn parse_false(input: &str) -> Res<&str, Literal> {
    context("False", tag_no_case("False"))(input)
        .map(|(next_input, _)| (next_input, Literal::Boolean(false)))
}

fn parse_true(input: &str) -> Res<&str, Literal> {
    context("True", tag_no_case("True"))(input)
        .map(|(next_input, _)| (next_input, Literal::Boolean(true)))
}

fn parse_boolean(input: &str) -> Res<&str, Literal> {
    alt((parse_false, parse_true))(input)
}

fn parse_nil(input: &str) -> Res<&str, Literal> {
    context("Nil", tag_no_case("Nil"))(input).map(|(next_input, _)| (next_input, Literal::Nil))
}

fn parse_array(input: &str) -> Res<&str, Literal> {
    context(
        "Array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(sp, char(',')), parse_literal),
                preceded(sp, char(']')),
            )),
        ),
    )(input)
    .map(|(next_input, res)| (next_input, Literal::Array(Vec::from(res))))
}

fn parse_key_value(input: &str) -> Res<&str, (&str, Literal)> {
    separated_pair(
        preceded(sp, parse_str_raw),
        cut(preceded(sp, char(':'))),
        parse_literal,
    )(input)
}

fn parse_map(input: &str) -> Res<&str, Literal> {
    context(
        "Map",
        preceded(
            char('{'),
            cut(terminated(
                map(
                    separated_list0(preceded(sp, char(',')), parse_key_value),
                    |tuple_vec| {
                        tuple_vec
                            .into_iter()
                            .map(|(k, v)| (String::from(k), v))
                            .collect()
                    },
                ),
                preceded(sp, char('}')),
            )),
        ),
    )(input)
    .map(|(next_input, res)| (next_input, Literal::Map(res)))
}

pub fn parse_literal(input: &str) -> Res<&str, Literal> {
    context(
        "Literal",
        preceded(
            sp,
            alt((
                parse_nil,
                parse_boolean,
                parse_num,
                parse_str,
                parse_array,
                parse_map,
                map(parse_variable, Literal::Variable),
            )),
        ),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use nom::error::{ErrorKind, VerboseError, VerboseErrorKind};

    #[test]
    fn parse_nil_test() {
        let string = "Nil";
        let res = parse_nil(string);
        assert_eq!(res, Ok(("", Literal::Nil)));
    }

    #[test]
    fn parse_nil_test_fail() {
        let string = "False";
        let res = parse_nil(string);
        assert_eq!(
            res,
            Err(nom::Err::Error(VerboseError {
                errors: vec![
                    ("False", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("False", VerboseErrorKind::Context("Nil"))
                ]
            }))
        );
    }

    #[test]
    fn parse_boolean_test() {
        let string = "False";
        let res = parse_boolean(string);
        assert_eq!(res, Ok(("", Literal::Boolean(false))));

        let string = "True";
        let res = parse_boolean(string);
        assert_eq!(res, Ok(("", Literal::Boolean(true))));
    }

    #[test]
    fn parse_string_test() {
        let string = "\"HelloWorld\" ";
        let res = parse_str(string);
        assert_eq!(res, Ok((" ", Literal::Str(String::from("HelloWorld")))));

        let string = "\"Hello World\" ";
        let res = parse_str(string);
        assert_eq!(res, Ok((" ", Literal::Str(String::from("Hello World")))));

        let string = "\"Hello    World\" ";
        let res = parse_str(string);
        assert_eq!(res, Ok((" ", Literal::Str(String::from("Hello    World")))));
    }

    #[test]
    fn parse_num_test() {
        let string = "1.1";
        let res = parse_num(string);
        assert_eq!(res, Ok(("", Literal::Num(1.1))));

        let string = "123.0";
        let res = parse_num(string);
        assert_eq!(res, Ok(("", Literal::Num(123.0))));

        let string = "1";
        let res = parse_num(string);
        assert_eq!(res, Ok(("", Literal::Num(1.0))));
    }

    #[test]
    fn parse_array_test() {
        let string = "[1, 1, 1]";
        let res = parse_array(string);
        assert_eq!(
            res,
            Ok((
                "",
                Literal::Array(vec![
                    Literal::Num(1.0),
                    Literal::Num(1.0),
                    Literal::Num(1.0)
                ])
            ))
        )
    }

    #[test]
    fn parse_array_test1() {
        let string = "[1, \"String\", 1]";
        let res = parse_array(string);
        assert_eq!(
            res,
            Ok((
                "",
                Literal::Array(vec![
                    Literal::Num(1.0),
                    Literal::Str(String::from("String")),
                    Literal::Num(1.0)
                ])
            ))
        )
    }

    #[test]
    fn parse_array_map() {
        let string = "{}";
        let res = parse_map(string);
        assert_eq!(res, Ok(("", Literal::Map(HashMap::new()))))
    }

    #[test]
    fn parse_array_map1() {
        let string = "{\"Helllo\": 2, \"World\": 3}";
        let res = parse_map(string);
        let mut hm = HashMap::new();
        hm.insert(String::from("Helllo"), Literal::Num(2.0));
        hm.insert(String::from("World"), Literal::Num(3.0));
        assert_eq!(res, Ok(("", Literal::Map(hm))))
    }

    #[test]
    fn parse_literal_test() {
        let string = "Nil";
        let res = parse_literal(string);
        assert_eq!(res, Ok(("", Literal::Nil)))
    }
}
