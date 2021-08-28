/// Colletion of some keywords and tokens which are common
use crate::parser::Res;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::error::context;

pub const KEYWORDS: [&'static str; 11] = [
    "return", "class", "end", "fun", "do", "while", "for", "if", "let", "in", "else",
];

macro_rules! define_token {
    ( $( { $fn_name:ident, $name:literal, $token:literal } ), *) => {
        $(
            pub fn $fn_name(input: &str) -> Res<&str, &str> {
                context(
                    $name,
                    tag($token)
                )(input)
            }
        )*
    }
}

define_token! {
    {add, "Add", "+"},
    {sub, "Sub", "-"},
    {mul, "Mul", "*"},
    {div, "Div", "/"},
    {equal, "Equal", "="},
    {unequal, "Unequal", "=="},
    {dot, "Dot", "."},
    {left_paren, "LeftParen", "("},
    {right_paren, "RightParen", ")"},
    {less_than, "LessThan", "<"},
    {greater_than, "GreaterThan", ">"},
    {less_eq_than, "LessEqThan", "<="},
    {greater_eq_than, "GreaterEqThan", ">="}
}

define_token! {
    {lreturn, "Return", "return"},
    {class, "Class", "class"},
    {end, "End", "end"},
    {fun, "Fun", "fun"},
    {ldo, "Do", "do"},
    {lwhile, "While", "while"},
    {lfor, "For", "for"},
    {lif, "If", "if"},
    {lelse, "Else", "else"},
    {llet, "Let", "let"},
    {lin, "In", "in"}
}

pub fn parse_tokens(input: &str) -> Res<&str, &str> {
    alt((
        add,
        sub,
        mul,
        div,
        equal,
        unequal,
        left_paren,
        right_paren,
        less_than,
        greater_than,
        less_eq_than,
        greater_eq_than,
    ))(input)
}

pub fn parse_keywords(input: &str) -> Res<&str, &str> {
    alt((lreturn, class, end, fun, ldo, lwhile, lfor, lif, llet))(input)
}
