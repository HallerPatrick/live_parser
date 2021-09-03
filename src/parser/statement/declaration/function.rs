use crate::parser::{
    literals::sp,
    parse_variable_raw,
    statement::opt_line_ending,
    statement::parse_statements,
    statement::{declaration::parse_parameter_list, Statement},
    tokens::{end, fun},
    Res, Variable,
};

use nom::{
    error::context,
    sequence::preceded,
    sequence::{terminated, tuple},
};

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Variable>,
    pub statements: Vec<Statement>,
}

pub fn parse_function(input: &str) -> Res<&str, Function> {
    context(
        "Func",
        terminated(
            tuple((
                parse_function_name,
                parse_function_arguments,
                parse_statements,
            )),
            preceded(sp, end),
        ),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            Function {
                name: String::from(res.0),
                parameters: res.1,
                statements: res.2,
            },
        )
    })
}

fn parse_function_name(input: &str) -> Res<&str, &str> {
    context(
        "FuncName",
        preceded(
            opt_line_ending,
            preceded(sp, preceded(fun, preceded(sp, parse_variable_raw))),
        ),
    )(input)
}

fn parse_function_arguments(input: &str) -> Res<&str, Vec<Variable>> {
    context("FuncArguments", parse_parameter_list)(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::expression::Expression;
    use crate::parser::literals::Literal;
    use crate::parser::statement::Assignment;

    #[test]
    fn test_parse_function_name() {
        let string = "fun hello ()";
        let res = parse_function_name(string);
        assert_eq!(res, Ok((" ()", "hello")));
    }

    // #[test]
    // fn test_parse_function() {
    //     let string = "fun hello(x, y)\n\tlet some = \"1\"\nend";
    //     let res = parse_function(string);

    //     assert_eq!(
    //         res,
    //         Ok((
    //             "",
    //             Function {
    //                 name: String::from("hello"),
    //                 parameters: vec![
    //                     Variable {
    //                         name: String::from("x")
    //                     },
    //                     Variable {
    //                         name: String::from("y")
    //                     }
    //                 ],
    //                 statements: vec![Statement::Assignment(Assignment {
    //                     variable: Variable {
    //                         name: String::from("some")
    //                     },
    //                     expression: Expression
    //                 })]
    //             }
    //         ))
    //     );
    // }

    // #[test]
    // fn test_fun() {
    //     let string = "fun fib(n)\n    if n == 0  do\n        return 0\n    end\n\n    if n == 1 do\n        return 1\n    end\n\n    if n == 2 do\n        return 1\n    end\n\n    return fib(n-1) + fib(n-2)\nend";
    //     let res = parse_function(string);
    //     println!("{:?}", res);
    // }

    // #[test]
    // fn test_fun_1() {
    //     let string = "fun fib(n)\n\nend";
    //     let res = parse_function(string);
    //     println!("{:?}", res);
    // }
}
