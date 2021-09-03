pub mod assignment;
pub mod class;
pub mod for_statement;
pub mod function;
pub mod if_statement;
pub mod while_statement;

use crate::parser::literals::sp;
use crate::parser::tokens::{left_paren, right_paren};
use crate::parser::{parse_variable, Res, Variable};

use nom::{
    character::complete::char,
    error::context,
    multi::separated_list0,
    sequence::{delimited, preceded},
};

// use super::{parse_for, parse_while, For, While};

pub fn parse_parameter_list(input: &str) -> Res<&str, Vec<Variable>> {
    context(
        "ParameterList",
        preceded(
            sp,
            delimited(
                left_paren,
                separated_list0(preceded(sp, char(',')), preceded(sp, parse_variable)),
                preceded(sp, right_paren),
            ),
        ),
    )(input)
}

// #[cfg(test)]
// #[ignore]
// mod tests {

//     use super::*;

//     #[test]
//     fn tst_parse_parameter_list_empty() {
//         let string = "()";
//         let res = parse_parameter_list(string);
//         assert_eq!(res, Ok(("", vec![])));
//     }

//     #[test]
//     fn tst_parse_parameter_list_one_arg() {
//         let string = "(x)";
//         let res = parse_parameter_list(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 vec![Variable {
//                     name: String::from("x")
//                 },]
//             ))
//         );
//     }

//     #[test]
//     fn tst_parse_parameter_list() {
//         let string = "(x, som1 )";
//         let res = parse_parameter_list(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 vec![
//                     Variable {
//                         name: String::from("x")
//                     },
//                     Variable {
//                         name: String::from("som1")
//                     }
//                 ]
//             ))
//         );
//     }
// }
