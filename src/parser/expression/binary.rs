use crate::parser::expression::{parse_expression, Expression};
use crate::parser::literals::sp;
use crate::parser::tokens::{parse_binary_operator, Operator, UnOperator};

use crate::parser::Res;

use nom::{
    error::context,
    sequence::{preceded, tuple},
};

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryOp {
    pub left: Expression,
    pub right: Expression,
    pub op: Operator,
}

pub(crate) fn parse_binary(input: &str) -> Res<&str, BinaryOp> {
    context(
        "Binary",
        tuple((
            preceded(sp, parse_expression),
            preceded(sp, parse_binary_operator),
            preceded(sp, parse_expression),
        )),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            BinaryOp {
                left: res.0,
                right: res.2,
                op: res.1,
            },
        )
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryOp {
    pub op: UnOperator,
    pub operand: Expression,
}
