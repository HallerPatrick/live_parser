#![allow(dead_code)]

mod comment;
pub mod expression;
pub mod literals;
pub mod statement;
pub mod tokens;

use nom::{
    error::{context, VerboseError},
    IResult, InputIter, Needed,
};

use nom_locate::LocatedSpan;

use crate::parser::statement::{parse_block, Block};

pub type Span<'a> = LocatedSpan<&'a str>;

pub type Res<'a, O> = IResult<Span<'a>, O, VerboseError<Span<'a>>>;

//
// #[derive(Debug, Clone, UnspecializedInput)]
// pub struct Span<'a, X = ()> {
//     sp: LocatedSpan<&'a str, X>
// }
//
// impl<'a> Span<'a> {
//
//     pub fn new(inp: &'a str) -> Span<'a> {
//         Span {
//             sp: LocatedSpan::new(inp)
//         }
//     }
//
//     pub fn fragment(&self) -> &str {
//         self.sp.fragment()
//     }
// }
//
// impl <'a>PartialEq for Span<'a> {
//     fn eq(&self, other: &Self) -> bool {
//         self.fragment() == other.fragment()
//     }
// }
//
// impl <'a> nom::InputLength for Span<'a> {
//     fn input_len(&self) -> usize {
//         self.sp.input_len()
//     }
// }
//
// impl <'a> nom::InputIter for Span<'a> {
//     type Item = <&'a str as InputIter>::Item;
//     type Iter = <&'a str as InputIter>::Iter;
//     type IterElem = <&'a str as InputIter>::IterElem;
//
//     fn iter_indices(& self) -> Self::Iter {
//         self.fragment().iter_indices()
//     }
//
//     fn iter_elements(& self) -> Self::IterElem {
//         self.fragment().iter_elements()
//     }
//
//     fn position<P>(& self, predicate: P) -> Option<usize> where P: Fn(Self::Item) -> bool {
//         self.fragment().position(predicate)
//     }
//
//     fn slice_index(& self, count: usize) -> Result<usize, Needed> {
//         self.fragment().slice_index(count)
//     }
// }
//
//
/// Entry point to the parser, which parse the liva language
pub fn parse_source(input: Span) -> Res<Block> {
    context("Root", parse_block)(input)
}
