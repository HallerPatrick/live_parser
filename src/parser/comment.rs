use crate::parser::{
    literals::sp,
    tokens::{comment, newline},
    Res,
};

use nom::{
    bytes::complete::take_while,
    character::complete::line_ending,
    error::context,
    sequence::{preceded, terminated},
};

pub(crate) fn parse_comment(input: &str) -> Res<&str, &str> {
    context(
        "Comment",
        preceded(take_while(move |s| s != '\n'), line_ending),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_comment() {
        let string = "// This is a comment\n";
        let res = parse_comment(string);
        assert_eq!(res, Ok(("", "\n")));
    }
}
