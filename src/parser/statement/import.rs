use crate::parser::{
    literals::{parse_variable, sp, Variable},
    tokens::{dot, external, import, las},
    Res,
};

use nom::{
    combinator::opt,
    error::context,
    multi::separated_list1,
    sequence::{preceded, tuple},
};

#[derive(PartialEq, Debug)]
pub struct Import {
    pub external: bool,
    pub path: Vec<Variable>,
    pub alias: Option<Variable>,
}

pub fn parse_import(input: &str) -> Res<&str, Import> {
    context(
        "Import",
        preceded(
            sp,
            tuple((
                opt(preceded(sp, external)),
                preceded(
                    preceded(sp, import),
                    separated_list1(preceded(sp, dot), preceded(sp, parse_variable)),
                ),
                opt(preceded(preceded(sp, las), preceded(sp, parse_variable))),
            )),
        ),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            Import {
                external: res.0.is_some(),
                path: res.1,
                alias: res.2,
            },
        )
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_import() {
        let string = "import hello";
        let res = parse_import(string);

        assert_eq!(
            res,
            Ok((
                "",
                Import {
                    external: false,
                    path: vec![Variable {
                        name: String::from("hello")
                    }],
                    alias: None
                }
            ))
        )
    }

    #[test]
    fn test_import_multi() {
        let string = "import hello.world as tuna";
        let res = parse_import(string);

        assert_eq!(
            res,
            Ok((
                "",
                Import {
                    external: false,
                    path: vec![
                        Variable {
                            name: String::from("hello")
                        },
                        Variable {
                            name: String::from("world")
                        }
                    ],
                    alias: Some(Variable {
                        name: String::from("tuna")
                    })
                }
            ))
        )
    }

    #[test]
    fn test_import_external() {
        let string = "external import hello";
        let res = parse_import(string);

        assert_eq!(
            res,
            Ok((
                "",
                Import {
                    external: true,
                    path: vec![Variable {
                        name: String::from("hello")
                    }],
                    alias: None
                }
            ))
        )
    }

    #[test]
    fn test_import_alias() {
        let string = "import hello as h";
        let res = parse_import(string);
        assert_eq!(
            res,
            Ok((
                "",
                Import {
                    external: false,
                    path: vec![Variable {
                        name: String::from("hello")
                    }],
                    alias: Some(Variable {
                        name: String::from("h")
                    })
                }
            ))
        )
    }
}
