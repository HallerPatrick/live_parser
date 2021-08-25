use crate::parser::expression::expression_lexer;
use crate::parser::expression::Expression;
use crate::parser::literals::sp;
use crate::parser::tokens::{ldo, left_paren, lwhile, right_paren};
use crate::parser::Res;

use super::{parse_statement, Statement};

use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
};

#[derive(Debug, PartialEq)]
pub struct While {
    cond: Expression,
    stmts: Vec<Statement>,
}

// TODO: Fix this
/// Statements are delimited by newlines
fn parse_statements(input: &str) -> Res<&str, Vec<Statement>> {
    separated_list0(preceded(sp, line_ending), parse_statement)(input)
}

pub fn parse_while(input: &str) -> Res<&str, Statement> {
    tuple((while_condition, parse_statements))(input).map(|(next_input, res)| {
        (
            next_input,
            Statement::While(While {
                cond: res.0,
                stmts: res.1,
            }),
        )
    })
}

fn while_condition(input: &str) -> Res<&str, Expression> {
    delimited(
        preceded(sp, lwhile),
        preceded(sp, expression_lexer),
        preceded(sp, tag("do")),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::parser::expression::ExpressionTerm;
    use crate::parser::literals::Literal;
    use crate::parser::Variable;

    #[test]
    fn parse_while_condition() {
        let string = "while x < 3 do";
        let res = while_condition(string);

        assert_eq!(
            res,
            Ok((
                "",
                vec![
                    ExpressionTerm::Variable(Variable {
                        name: String::from("x")
                    }),
                    ExpressionTerm::Token(String::from("<")),
                    ExpressionTerm::Literal(Literal::Num(3.0))
                ]
            ))
        )
    }

    #[test]
    fn test_parse_statement() {
        let string = "let x = 3\nlet x = 4";
        let res = parse_statements(string);
        assert_eq!(res, Ok(("", vec![])))
    }

    // #[test]
    // fn parse_while_condition1() {
    //     let string = "while x do x + 3\n let y = 3";
    //     let res = parse_while(string);

    //     assert_eq!(
    //         res,
    //         Ok((
    //             "",
    //             Statement::While(
    //                 While {
    //                 cond:
    //             vec![
    //                 ExpressionTerm::Variable(Variable {
    //                     name: String::from("x")
    //                 }),
    //                 ExpressionTerm::Token(String::from("<")),
    //                 ExpressionTerm::Literal(Literal::Num(3.0))
    //             ], stmts: vec![]})
    //         ))
    //     )
    // }
}
