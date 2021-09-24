//! Collection of all possible literals

use crate::Span;
use std::borrow::Borrow;
use std::collections::HashMap;

use crate::parser::{comment::parse_comment, tokens::KEYWORDS, Res};
use nom::number::complete::recognize_float;
use nom::{
    self,
    branch::alt,
    bytes::complete::{escaped, tag, tag_no_case, take_while},
    character::complete::char,
    character::complete::{alpha1, alphanumeric1 as alphanumeric, one_of, space1},
    combinator::{cut, map, opt, recognize},
    error::context,
    error_position,
    multi::{many0, separated_list0},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Clone, Debug)]
pub struct Token<'a, T> {
    pub value: T,
    pub pos: Span<'a>,
}

impl<'a, T> Token<'a, T> {
    pub fn new(value: T, pos: Span) -> Token<T> {
        Token { value, pos }
    }
}

// Collection of all possible literals
#[derive(Clone, Debug, PartialEq)]
pub enum Literal<'a> {
    Str(Token<'a, String>),
    Boolean(Token<'a, bool>),
    Nil(Span<'a>),
    Num(Token<'a, f64>),
    Array(Vec<Literal<'a>>),
    Map(HashMap<String, Literal<'a>>),
    Variable(Token<'a, Identifier<'a>>),
}

impl<'a, T> PartialEq for Token<'a, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}

// TODO: Change to identifier
// Contains the name of a identifier, which has to start with letter, but can contain
// numbers and underscores
// #[derive(Debug, PartialEq, Clone)]
// pub struct Identifier<'a> {
//     pub name: &'a str,
// }

// impl<'a> ToString for Identifier<'a> {
//     fn to_string(&self) -> String {
//         String::from(self.name)
//     }
// }

// impl<'a> Identifier<'a> {
//     pub fn new(name: &'a str) -> Self {
//         Identifier { name }
//     }
// }

pub type Identifier<'a> = &'a str;

pub(crate) fn parse_variable_raw(input: Span) -> Res<Span> {
    let res = context(
        "Variable",
        // Keywords should not be parsed
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric, tag("_")))),
        )),
    )(input);

    if let IResult::Ok((_, name)) = res {
        if KEYWORDS.contains(name.borrow()) {
            return Err(nom::Err::Error(error_position!(
                input,
                nom::error::ErrorKind::Tag
            )));
        }
    }
    res
}

pub(crate) fn parse_variable(input: Span) -> Res<Literal> {
    parse_variable_raw(input).map(|(next_input, res)| {
        (
            next_input,
            Literal::Variable(Token::new(*res.fragment(), res)),
        )
    })
}

pub(crate) fn sp(input: Span) -> Res<&str> {
    let chars = " \t\r\n";
    tuple((take_while(move |c| chars.contains(c)), opt(parse_comment)))(input)
        .map(|(next_input, _)| (next_input, ""))
}

fn alphanumeric_ws(input: Span) -> Res<Span> {
    alt((space1, alphanumeric))(input)
}

fn parse_string(input: Span) -> Res<Span> {
    escaped(alphanumeric_ws, '\\', one_of("\"n\\"))(input)
}

fn parse_str_raw(input: Span) -> Res<Span> {
    alt((parse_str_raw_single, parse_str_raw_double))(input)
}
fn parse_str_raw_single(input: Span) -> Res<Span> {
    context(
        "String",
        preceded(char('\"'), cut(terminated(parse_string, char('\"')))),
    )(input)
}

fn parse_str_raw_double(input: Span) -> Res<Span> {
    context(
        "String",
        preceded(char('\''), cut(terminated(parse_string, char('\'')))),
    )(input)
}

fn parse_str(input: Span) -> Res<Literal> {
    parse_str_raw(input).map(|(next_input, res)| {
        (
            next_input,
            Literal::Str(Token::new(String::from(*res.fragment()), res)),
        )
    })
}

fn parse_num(input: Span) -> Res<Literal> {
    context("Num", recognize_float)(input).map(|(next_input, res)| {
        (
            next_input,
            Literal::Num(Token::new(res.fragment().parse::<f64>().unwrap(), res)),
        )
    })
}

fn parse_false(input: Span) -> Res<Literal> {
    context("False", tag_no_case("False"))(input)
        .map(|(next_input, res)| (next_input, Literal::Boolean(Token::new(false, res))))
}

fn parse_true(input: Span) -> Res<Literal> {
    context("True", tag_no_case("True"))(input)
        .map(|(next_input, res)| (next_input, Literal::Boolean(Token::new(true, res))))
}

fn parse_boolean(input: Span) -> Res<Literal> {
    alt((parse_false, parse_true))(input)
}

fn parse_nil(input: Span) -> Res<Literal> {
    context("Nil", tag_no_case("Nil"))(input)
        .map(|(next_input, res)| (next_input, Literal::Nil(res)))
}

// TODO: Not only literals, but also expressions are allowed in list
fn parse_array(input: Span) -> Res<Literal> {
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
    .map(|(next_input, res)| (next_input, Literal::Array(res)))
}

fn parse_key_value(input: Span) -> Res<(Span, Literal)> {
    separated_pair(
        preceded(sp, parse_str_raw),
        cut(preceded(sp, char(':'))),
        parse_literal,
    )(input)
}

// TODO: Not only literals, but also expressions are allowed in maps
fn parse_map(input: Span) -> Res<Literal> {
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
                            .map(|(k, v)| (String::from(*k.fragment()), v))
                            .collect()
                    },
                ),
                preceded(sp, char('}')),
            )),
        ),
    )(input)
    .map(|(next_input, res)| (next_input, Literal::Map(res)))
}

pub(crate) fn parse_literal<'a>(input: Span<'a>) -> Res<Literal<'a>> {
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
                parse_variable,
            )),
        ),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_token() {
        let token1 = Token::new("hello", Span::new("Thisone"));
        let token2 = Token::new("hello", Span::new("other"));
        assert_eq!(token1, token2);
    }
    #[test]
    fn test_sp() {
        let string = " \n\n";
        let (_, res) = sp(Span::new(string)).unwrap();
        assert_eq!(res, "");
    }

    #[test]
    fn test_sp2() {
        let string = " \n\n// \n";
        let (_, res) = sp(Span::new(string)).unwrap();
        assert_eq!(res, "");
    }

    #[test]
    fn parse_nil_test() {
        let string = "Nil";
        let (_, res) = parse_nil(Span::new(string)).unwrap();
        assert_eq!(res, Literal::Nil(Span::new("Nil")));
    }

    // #[ignore]
    // #[test]
    // fn parse_nil_test_fail() {
    //     let string = "False";
    //     let (_, res) = parse_nil(Span::new(string));
    //     assert_eq!(
    //         res,
    //         Err(nom::Err::Error(VerboseError {
    //             errors: vec![
    //                 ("False", VerboseErrorKind::Nom(ErrorKind::Tag)),
    //                 ("False", VerboseErrorKind::Context("Nil"))
    //             ]
    //         }))
    //     );
    // }

    #[test]
    fn parse_boolean_test() {
        let string = "False";
        let (_, res) = parse_boolean(Span::new(string)).unwrap();
        assert_eq!(res, Literal::Boolean(Token::new(false, Span::new("False"))));

        let string = "True";
        let (_, res) = parse_boolean(Span::new(string)).unwrap();
        assert_eq!(res, Literal::Boolean(Token::new(true, Span::new("True"))));
    }

    #[test]
    fn parse_string_test() {
        let string = "\"HelloWorld\" ";
        let (_, res) = parse_str(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Literal::Str(Token::new(
                String::from("HelloWorld"),
                Span::new("HelloWorld")
            ))
        );

        let string = "\"Hello World\" ";
        let (_, res) = parse_str(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Literal::Str(Token::new(
                String::from("Hello World"),
                Span::new("Hello World")
            ))
        );

        let string = "\"Hello    World\" ";
        let (_, res) = parse_str(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Literal::Str(Token::new(
                String::from("Hello    World"),
                Span::new("Hello    World")
            ))
        );
    }

    #[test]
    fn parse_num_test() {
        let string = "1.1";
        let (_, res) = parse_num(Span::new(string)).unwrap();
        assert_eq!(res, Literal::Num(Token::new(1.1, Span::new("1.1"))));

        let string = "123.0";
        let (_, res) = parse_num(Span::new(string)).unwrap();
        assert_eq!(res, Literal::Num(Token::new(123.0, Span::new("123.0"))));

        let string = "1";
        let (_, res) = parse_num(Span::new(string)).unwrap();
        assert_eq!(res, Literal::Num(Token::new(1.0, Span::new("1"))));
    }

    #[test]
    fn parse_array_test() {
        let string = "[1, 1, 1]";
        let (_, res) = parse_array(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Literal::Array(vec![
                Literal::Num(Token::new(1.0, Span::new("1"))),
                Literal::Num(Token::new(1.0, Span::new("1"))),
                Literal::Num(Token::new(1.0, Span::new("1")))
            ])
        )
    }

    #[test]
    fn parse_array_test1() {
        let string = "[1, \"String\", 1]";
        let (_, res) = parse_array(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Literal::Array(vec![
                Literal::Num(Token::new(1.0, Span::new("1"))),
                Literal::Str(Token::new(String::from("String"), Span::new("String"))),
                Literal::Num(Token::new(1.0, Span::new("1"))),
            ])
        )
    }

    #[test]
    fn parse_array_map() {
        let string = "{}";
        let (_, res) = parse_map(Span::new(string)).unwrap();
        assert_eq!(res, Literal::Map(HashMap::new()));
    }

    #[test]
    fn parse_array_map1() {
        let string = "{\"Helllo\": 2, \"World\": 3}";
        let (_, res) = parse_map(Span::new(string)).unwrap();
        let mut hm = HashMap::new();
        hm.insert(
            String::from("Helllo"),
            Literal::Num(Token::new(2.0, Span::new("2"))),
        );
        hm.insert(
            String::from("World"),
            Literal::Num(Token::new(3.0, Span::new("3"))),
        );
        assert_eq!(res, Literal::Map(hm));
    }

    #[test]
    fn parse_literal_test() {
        let string = "Nil";
        let (_, res) = parse_literal(Span::new(string)).unwrap();
        assert_eq!(res, Literal::Nil(Span::new("Nil")))
    }

    #[test]
    fn test_parse_variable() {
        let string = "hello123";
        let (_, res) = parse_variable(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Literal::Variable(Token::new("hello123", Span::new("hello123")))
        );
    }

    #[test]
    fn test_parse_variable_cap() {
        let string = "Hello123";
        let (_, res) = parse_variable(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Literal::Variable(Token::new("Hello123", Span::new("Hello123")))
        );
    }

    #[test]
    fn test_parse_variable_func() {
        let string = "hello()";
        let (_, res) = parse_variable(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Literal::Variable(Token::new("hello", Span::new("hello")))
        );
    }
}
