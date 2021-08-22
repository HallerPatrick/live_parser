use crate::parser::{literals::parse_literal, Res};

#[derive(Debug, PartialEq)]
pub struct Expression {}




pub fn parse_expression(input: &str) -> Res<&str, Expression> {
    Ok(("", Expression {}))
}

// https://github.com/Geal/nom/blob/master/tests/arithmetic.rs
