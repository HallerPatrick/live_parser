use super::function::{parse_function, Function};

use crate::parser::{
    literals::parse_variable_raw,
    literals::sp,
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
        preceded(
            sp,
            tuple((
                parse_class_name,
                terminated(parse_methods, preceded(sp, end)),
            )),
        ),
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
            preceded(sp, preceded(sp, many0(parse_function))),
            opt_line_ending,
        ),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::parser::{
        expression::Expression,
        literals::{Literal, Variable},
        statement::declaration::assignment::LAssignment,
        statement::declaration::function::Function,
        statement::{Block, ReturnStmt, Statement},
    };

    #[test]
    fn test_prase_class_name() {
        let string = " class Hello ";
        let res = parse_class_name(string);
        assert_eq!(res, Ok((" ", "Hello")))
    }

    #[test]
    fn test_parse_class_with_method() {
        let string = "class Hello\n\tfun foo()\n\t\tlet some = 1\n\tend\nend";
        let res = parse_class(string);
        assert_eq!(
            res,
            Ok((
                "",
                Class {
                    name: String::from("Hello"),
                    methods: vec![Function {
                        name: String::from("foo"),
                        parameters: vec![],
                        statements: Block {
                            return_stmt: None,
                            statements: vec![Statement::LAssignment(LAssignment {
                                variable: Variable::new("some"),
                                expression: Expression::Literal(Literal::Num(1.0))
                            })]
                        }
                    }]
                }
            ))
        );
    }

    #[test]
    fn test_parse_class_with_method1() {
        let string = "\n\nclass Hello\n\n\tfun method1()\n\t\treturn 'Hello World'\n\tend\n\nend\n";
        let res = parse_class(string);
        assert_eq!(
            res,
            Ok((
                "\n",
                Class {
                    name: String::from("Hello"),
                    methods: vec![Function {
                        name: String::from("method1"),
                        parameters: vec![],
                        statements: Block {
                            return_stmt: Some(ReturnStmt {
                                values: vec![Expression::Literal(Literal::Str(String::from(
                                    "Hello World"
                                )))],
                            }),
                            statements: vec![]
                        }
                    }]
                }
            ))
        );
    }

    #[test]
    fn test_empty_class() {
        let string = "class Test\n\n   end\n";
        let res = parse_class(string);

        assert_eq!(
            res,
            Ok((
                "\n",
                Class {
                    name: String::from("Test"),
                    methods: vec![]
                }
            ))
        )
    }

    #[test]
    fn test_class_1() {
        let string = "class Person\r\n\r\n    fun init(self, height, weight)\r\n       self.height = height\r\n       self.weight = weight \r\n    end\r\n\r\n    fun get_bmi(self)\r\n       let w_res = self.height * self.height\r\n       return self.weight / w_res\r\n    end\r\nend\r\n\r\n";
        let res = parse_class(string);
        println!("{:?}", res);
    }
}
