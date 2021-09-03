use crate::parser::expression::{parse_expression, Expression};
use crate::parser::literals::sp;
use crate::parser::tokens::{
    land, lor, parse_binary_operator, parse_unary_operator, BoolOperator, Operator, UnOperator,
};

use crate::parser::Res;

use nom::{
    branch::alt,
    error::context,
    multi::many1,
    sequence::{preceded, terminated, tuple},
};

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryOp {
    pub left: Expression,
    pub right: Expression,
    pub op: Operator,
}

pub fn parse_binary(input: &str) -> Res<&str, BinaryOp> {
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

/// Unary can only be applied prefixed
pub fn parse_unary(input: &str) -> Res<&str, UnaryOp> {
    context(
        "Unary",
        tuple((
            preceded(sp, parse_unary_operator),
            preceded(sp, parse_expression),
        )),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            UnaryOp {
                op: res.0,
                operand: res.1,
            },
        )
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoolOp {
    op: BoolOperator,
    values: Vec<Expression>,
}

/// Bool operations are infix but can be chained
/// Example: a and b and c -> BoolOp { op: And, values: [a, b, c] }
pub fn parse_bool_op(input: &str) -> Res<&str, BoolOp> {
    context("BoolOp", alt((parse_and_op, parse_and_op)))(input)
}

pub fn parse_or_op(input: &str) -> Res<&str, BoolOp> {
    context(
        "OrOperator",
        tuple((
            many1(terminated(
                preceded(sp, parse_expression),
                preceded(sp, lor),
            )),
            preceded(sp, parse_expression),
        )),
    )(input)
    .map(|(next_input, res)| {
        let mut values = res.0;
        values.push(res.1);
        (
            next_input,
            BoolOp {
                op: BoolOperator::Or,
                values,
            },
        )
    })
}

pub fn parse_and_op(input: &str) -> Res<&str, BoolOp> {
    context(
        "OrOperator",
        tuple((many1(terminated(parse_expression, land)), parse_expression)),
    )(input)
    .map(|(next_input, res)| {
        let mut values = res.0;
        values.push(res.1);
        (
            next_input,
            BoolOp {
                op: BoolOperator::And,
                values,
            },
        )
    })
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     use crate::parser::expression::call::Call;
//     use crate::parser::literals::Literal;
//     use crate::parser::tokens::Operator;
//     use crate::parser::Variable;

//     #[test]
//     fn test_parse_binary() {
//         let string = "3 + 3 + 3";
//         let res = parse_binary(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 BinaryOp {
//                     op: Operator::Add,
//                     left: Expression::Literal(Literal::Num(3.0)),
//                     right: Expression::Literal(Literal::Num(3.0))
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn test_comp() {
//         let string = "3 < 3";
//         let res = parse_comp(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 CompOp {
//                     op: CompOperator::LessThan,
//                     left: Expression::Literal(Literal::Num(3.0)),
//                     right: Expression::Literal(Literal::Num(3.0))
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn test_parse_or_op() {
//         let string = "call() or call1()";
//         let res = parse_or_op(string);

//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 BoolOp {
//                     op: BoolOperator::Or,
//                     values: vec![
//                         Expression::Call(Call {
//                             callee: Literal::Variable(Variable {
//                                 name: String::from("call")
//                             }),
//                             args: vec![]
//                         }),
//                         Expression::Call(Call {
//                             callee: Literal::Variable(Variable {
//                                 name: String::from("call1")
//                             }),
//                             args: vec![]
//                         })
//                     ]
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn test_parse_or_op_multi_or() {
//         let string = "call() or call1() or lit()";
//         let res = parse_or_op(string);

//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 BoolOp {
//                     op: BoolOperator::Or,
//                     values: vec![
//                         Expression::Call(Call {
//                             callee: Literal::Variable(Variable {
//                                 name: String::from("call")
//                             }),
//                             args: vec![]
//                         }),
//                         Expression::Call(Call {
//                             callee: Literal::Variable(Variable {
//                                 name: String::from("call1")
//                             }),
//                             args: vec![]
//                         }),
//                         Expression::Call(Call {
//                             callee: Literal::Variable(Variable {
//                                 name: String::from("lit")
//                             }),
//                             args: vec![]
//                         }),
//                     ]
//                 }
//             ))
//         )
//     }
// }
