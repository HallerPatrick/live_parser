use nom::{
    combinator::opt,
    error::context,
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
};

use super::{parse_expression, Expression};

use crate::parser::{
    literals::{parse_literal, sp, Literal},
    tokens::{comma, left_paren, right_paren},
    Res, Span,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Call<'a> {
    pub callee: Option<Literal<'a>>,
    pub args: Vec<Expression<'a>>,
}

pub(crate) fn args(input: Span) -> Res<Vec<Expression>> {
    context(
        "Args",
        separated_list0(preceded(sp, comma), parse_expression),
    )(input)
}

pub(crate) fn parse_call(input: Span) -> Res<Call> {
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

#[cfg(test)]
mod tests {

    use crate::parser::literals::Identifier;

    use super::*;
    use crate::literals::Token;

    #[test]
    fn test_call() {
        let string = "call()";
        let (_, res) = parse_call(Span::new(string)).unwrap();
        let e_res = Call {
            callee: Some(Literal::Variable(Token::new(
                Identifier { name: "call" },
                Span::new("call"),
            ))),
            args: vec![],
        };
        assert_eq!(res, e_res);
    }
}
