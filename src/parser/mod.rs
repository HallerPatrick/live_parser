#![allow(dead_code)]

mod comment;
pub mod expression;
pub mod literals;
pub mod statement;
pub mod tokens;

use nom::{
    error::{context, VerboseError},
    IResult,
};

use crate::parser::statement::{parse_block, Block};

pub type Res<T, U> = IResult<T, U, VerboseError<T>>;

/// Entry point to the parser, which parse the liva language
pub fn parse_source(input: &str) -> Res<&str, Block> {
    context("Root", parse_block)(input)
}
