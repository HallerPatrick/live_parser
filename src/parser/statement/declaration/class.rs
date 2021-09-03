use super::function::{parse_function, Function};

use crate::parser::{
    literals::sp,
    parse_variable_raw,
    statement::opt_line_ending,
    tokens::{class, end},
    Res,
};

use nom::{
    error::context,
    multi::many0,
    sequence::{preceded, terminated, tuple},
};

#[derive(Debug, PartialEq)]
pub struct Class {
    pub name: String,
    pub methods: Vec<Function>,
}

pub fn parse_class(input: &str) -> Res<&str, Class> {
    context(
        "Class",
        tuple((
            parse_class_name,
            terminated(parse_methods, preceded(sp, end)),
        )),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            Class {
                name: String::from(res.0),
                methods: res.1,
            },
        )
    })
}

fn parse_class_name(input: &str) -> Res<&str, &str> {
    context(
        "ClassName",
        preceded(preceded(sp, class), preceded(sp, parse_variable_raw)),
    )(input)
}

fn parse_methods(input: &str) -> Res<&str, Vec<Function>> {
    context(
        "Methods",
        terminated(
            preceded(opt_line_ending, preceded(sp, many0(parse_function))),
            opt_line_ending,
        ),
    )(input)
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     use crate::parser::{
//         expression::ExpressionTerm, literals::Literal,
//         statement::declaration::assignment::Assignment, statement::Statement, Variable,
//     };

//     #[test]
//     fn test_prase_class_name() {
//         let string = " class Hello ";
//         let res = parse_class_name(string);
//         assert_eq!(res, Ok((" ", "Hello")))
//     }

//     #[test]
//     fn test_parse_class() {
//         let string = "class Hello\n\tfun foo()\n\t\tlet some = 1\n\tend\nend";
//         let res = parse_class(string);
//         assert_eq!(
//             res,
//             Ok((
//                 "",
//                 Class {
//                     name: String::from("Hello"),
//                     methods: vec![Function {
//                         name: String::from("foo"),
//                         parameters: vec![],
//                         statements: vec![Statement::Assignment(Assignment {
//                             variable: Variable {
//                                 name: String::from("some")
//                             },
//                             expression: vec![ExpressionTerm::Literal(Literal::Num(1.0))]
//                         })]
//                     }]
//                 }
//             ))
//         );
//     }

//     #[test]
//     fn test() {
//         let string = "class Test\n\n   end\n";
//         let res = parse_class(string);

//         assert_eq!(
//             res,
//             Ok((
//                 "\n",
//                 Class {
//                     name: String::from("Test"),
//                     methods: vec![]
//                 }
//             ))
//         )
//     }
// }
