#![allow(dead_code)]

use std::fs;

mod comment;
pub mod expression;
pub mod literals;
pub mod statement;
pub mod tokens;

use nom::{error::VerboseError, IResult};

use nom_locate::LocatedSpan;

use crate::parser::statement::{parse_block, Block};

pub type Span<'a> = LocatedSpan<&'a str>;

pub type Res<'a, O> = IResult<Span<'a>, O, VerboseError<Span<'a>>>;

/// Entry point to the parser, which parse the liva language
// pub fn parse_source(input: Span) -> Block {
//     let (_, root) = parse_block(input).unwrap();
//     root
// }

/// Entry point to the parser, which parse the liva language
pub fn parse_source(input: Span) -> Block {
    let (_, root) = parse_block(input).unwrap();
    root
}

// fn read_source(filename: &str) -> Result<&Span, Box<dyn std::error::Error>> {
//     let f = filename.clone();
//     let source: &str = &fs::read_to_string(f)?;
//     Ok(&Span::new(source))
// }

// pub fn parse_source_from_file(filename: &str) -> Result<Block, Box<dyn std::error::Error>> {
//     Ok(parse_source(read_source(filename)))
// }
