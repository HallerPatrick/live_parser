use crate::parser::expression::expression_lexer;
use crate::parser::expression::Expression;
use crate::parser::literals::sp;
use crate::parser::tokens::{ldo, left_paren, lwhile, right_paren};
use crate::parser::Res;

use super::Statement;

use nom::{
    bytes::complete::tag,
    combinator::cut,
    sequence::{delimited, preceded, terminated},
};

pub struct While {
    cond: Expression,
    stmts: Vec<Statement>,
}

// fn parse_while(input: &str) -> Res<&str, While> {

// }

fn while_condition(input: &str) -> Res<&str, Expression> {
    delimited(
        preceded(sp, lwhile),
        preceded(sp, expression_lexer),
        preceded(sp, tag("do")),
    )(input)
}

// fn parser(input: &str) -> Res<&str, &str> {

// }

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
}
