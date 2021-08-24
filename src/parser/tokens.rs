/// Colletion of some keywords and tokens which are common
use crate::parser::Res;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::error::context;

pub fn tokens(input: &str) -> Res<&str, &str> {
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

pub fn keywords(input: &str) -> Res<&str, &str> {
    alt((lreturn, class, end, fun, ldo, lwhile))(input)
}

// TODO: Add context everywhere

// Operators

pub fn add(input: &str) -> Res<&str, &str> {
    context("Add", tag("+"))(input)
}

pub fn sub(input: &str) -> Res<&str, &str> {
    context("Sub", tag("-"))(input)
}

pub fn mul(input: &str) -> Res<&str, &str> {
    context("mul", tag("*"))(input)
}

pub fn div(input: &str) -> Res<&str, &str> {
    tag("/")(input)
}

pub fn equal(input: &str) -> Res<&str, &str> {
    tag("==")(input)
}

pub fn unequal(input: &str) -> Res<&str, &str> {
    tag("!=")(input)
}

pub fn dot(input: &str) -> Res<&str, &str> {
    tag(".")(input)
}

pub fn left_paren(input: &str) -> Res<&str, &str> {
    tag("(")(input)
}

pub fn right_paren(input: &str) -> Res<&str, &str> {
    tag(")")(input)
}

pub fn less_than(input: &str) -> Res<&str, &str> {
    tag("<")(input)
}

pub fn greater_than(input: &str) -> Res<&str, &str> {
    tag(">")(input)
}

pub fn less_eq_than(input: &str) -> Res<&str, &str> {
    tag("<=")(input)
}

pub fn greater_eq_than(input: &str) -> Res<&str, &str> {
    tag(">=")(input)
}

// Keywords

pub fn lreturn(input: &str) -> Res<&str, &str> {
    tag("return")(input)
}

pub fn class(input: &str) -> Res<&str, &str> {
    tag("class")(input)
}

pub fn end(input: &str) -> Res<&str, &str> {
    tag("end")(input)
}

pub fn fun(input: &str) -> Res<&str, &str> {
    tag("fun")(input)
}

pub fn ldo(input: &str) -> Res<&str, &str> {
    context("do", tag("#do"))(input)
}

pub fn lwhile(input: &str) -> Res<&str, &str> {
    context("while", tag("while"))(input)
}
