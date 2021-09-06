use nom::{
    combinator::opt,
    error::context,
    multi::separated_list0,
    sequence::{delimited, preceded, terminated, tuple},
};

use super::{parse_expression, Expression};

use crate::parser::{
    literals::{parse_literal, sp, Literal},
    tokens::{comma, dot, left_paren, right_paren},
    Res,
};

#[derive(Clone, Debug, PartialEq)]
pub struct MemberCall {
    caller: Literal,
    call: Call,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub callee: Option<Literal>,
    pub args: Vec<Expression>,
}

pub fn args(input: &str) -> Res<&str, Vec<Expression>> {
    context(
        "Args",
        separated_list0(preceded(sp, comma), parse_expression),
    )(input)
}

pub fn parse_call(input: &str) -> Res<&str, Call> {
    context(
        "Call",
        tuple((
            opt(preceded(sp, parse_literal)),
            delimited(
                preceded(sp, left_paren),
                preceded(sp, args),
                preceded(sp, right_paren),
            ),
        )),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            Call {
                callee: res.0,
                args: res.1,
            },
        )
    })
}

pub fn parse_member_call(input: &str) -> Res<&str, MemberCall> {
    context(
        "MemberCall",
        tuple((
            terminated(preceded(sp, parse_literal), dot),
            preceded(sp, parse_call),
        )),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            MemberCall {
                caller: res.0,
                call: res.1,
            },
        )
    })
}

// #[cfg(test)]
// mod tests {

//     use crate::parser::Variable;

//     use super::*;

//     #[test]
//     fn test_call() {
//         let string = "call()";
//         let res = parse_call(string);
//         let e_res = Call {
//             callee: Literal::Variable(Variable {
//                 name: String::from("call"),
//             }),
//             args: vec![],
//         };
//         assert_eq!(res, Ok(("", e_res)));
//     }

//     #[test]
//     fn test_method_call() {
//         let string = "method.call()";
//         let res = parse_member_call(string);
//         let e_res = MemberCall {
//             caller: Literal::Variable(Variable {
//                 name: String::from("method"),
//             }),
//             call: Call {
//                 callee: Literal::Variable(Variable {
//                     name: String::from("call"),
//                 }),
//                 args: vec![],
//             },
//         };
//         assert_eq!(res, Ok(("", e_res)));
//     }
// }