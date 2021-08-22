/// Colletion of some keywords and tokens which are common

use crate::parser::Res;

use nom::bytes::complete::tag;


// Operators


pub fn add(input: &str) ->  Res<&str, &str> {
   tag("+") (input)
}

pub fn sub(input: &str) ->  Res<&str, &str> {
   tag("-") (input)
}

pub fn mul(input: &str) ->  Res<&str, &str> {
   tag("*") (input)
}

pub fn div(input: &str) ->  Res<&str, &str> {
   tag("/") (input)
}

pub fn equal(input: &str) ->  Res<&str, &str> {
   tag("==") (input)
}

pub fn unequal(input: &str) ->  Res<&str, &str> {
   tag("!=") (input)
}



// Keywords

pub fn lreturn(input: &str) ->  Res<&str, &str> {
   tag("return") (input)
}

pub fn class(input: &str) ->  Res<&str, &str> {
   tag("class") (input)
}

pub fn end(input: &str) ->  Res<&str, &str> {
   tag("end") (input)
}

pub fn fun(input: &str) ->  Res<&str, &str> {
   tag("fun") (input)
}



