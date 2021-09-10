use crate::parser::{
    literals::{parse_variable, sp, Variable},
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

#[derive(PartialEq, Debug)]
pub struct Import<'a> {
    pub external: bool,
    pub path: Vec<Literal<'a>>,
    pub alias: Option<Literal<'a>>,
}

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
    use crate::literals::Token;

    #[test]
    fn test_import() {
        let string = "import hello";
        let (_, res) = parse_import(Span::new(string)).unwrap();

        assert_eq!(
            res,
            Import {
                external: false,
                path: vec![Literal::Variable(Token::new(
                    Variable::new("hello"),
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
                    Literal::Variable(Token::new(Variable::new("hello"), Span::new("hello"))),
                    Literal::Variable(Token::new(Variable::new("world"), Span::new("world")))
                ],
                alias: Some(Literal::Variable(Token::new(
                    Variable::new("tuna"),
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
                    Variable::new("hello"),
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
                    Variable::new("hello"),
                    Span::new("hello")
                ))],
                alias: Some(Literal::Variable(Token::new(
                    Variable::new("h"),
                    Span::new("h")
                )))
            }
        )
    }
}
