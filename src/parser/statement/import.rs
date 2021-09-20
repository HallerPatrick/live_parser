//! Import other modules into the current namespace
//!
//! The import statement allowes to import modules from
//! the standard lib, local modules or thid-party modules
//!
//! ```py
//! import math
//! import lib.functools
//!
//! external import frosch.handler as fhandler
//! ```
use crate::parser::{
    literals::{parse_variable, sp},
    tokens::{dot, external, import, las},
    Res, Span,
};

use crate::literals::Literal;
use nom::{
    combinator::opt,
    error::context,
    multi::separated_list1,
    sequence::{preceded, tuple},
};

/// The import struct represents one import statement
#[derive(PartialEq, Debug)]
pub struct Import<'a> {
    /// If a module is a third-party module, therefore not
    /// in the local project or part of the standard lib
    /// it has to be marked as an external module.
    pub external: bool,

    /// The path defines where to look for the import.
    /// A path is represented as a list of Identifier names
    pub path: Vec<Literal<'a>>,

    /// For convenience can a import be aliased, which
    /// represented the imported module
    pub alias: Option<Literal<'a>>,
}

/// Parses the input inot a Import struct
pub fn parse_import(input: Span) -> Res<Import> {
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
    use crate::literals::{Token, Identifier};

    #[test]
    fn test_import() {
        let string = "import hello";
        let (_, res) = parse_import(Span::new(string)).unwrap();

        assert_eq!(
            res,
            Import {
                external: false,
                path: vec![Literal::Variable(Token::new(
                    Identifier::new("hello"),
                    Span::new("hello")
                ))],
                alias: None
            }
        )
    }

    #[test]
    fn test_import_multi() {
        let string = "import hello.world as tuna";
        let (_, res) = parse_import(Span::new(string)).unwrap();

        assert_eq!(
            res,
            Import {
                external: false,
                path: vec![
                    Literal::Variable(Token::new(Identifier::new("hello"), Span::new("hello"))),
                    Literal::Variable(Token::new(Identifier::new("world"), Span::new("world")))
                ],
                alias: Some(Literal::Variable(Token::new(
                    Identifier::new("tuna"),
                    Span::new("tuna")
                )))
            }
        )
    }

    #[test]
    fn test_import_external() {
        let string = "external import hello";
        let (_, res) = parse_import(Span::new(string)).unwrap();

        assert_eq!(
            res,
            Import {
                external: true,
                path: vec![Literal::Variable(Token::new(
                    Identifier::new("hello"),
                    Span::new("hello")
                ))],
                alias: None
            }
        )
    }

    #[test]
    fn test_import_alias() {
        let string = "import hello as h";
        let (_, res) = parse_import(Span::new(string)).unwrap();
        assert_eq!(
            res,
            Import {
                external: false,
                path: vec![Literal::Variable(Token::new(
                    Identifier::new("hello"),
                    Span::new("hello")
                ))],
                alias: Some(Literal::Variable(Token::new(
                    Identifier::new("h"),
                    Span::new("h")
                )))
            }
        )
    }
}
