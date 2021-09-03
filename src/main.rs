#[macro_use]
extern crate lazy_static;

// #![feature(const_for)]
// #![feature(const_mut_refs)]

pub mod parser;

use std::error::Error;
use std::fs;

use parser::parse_source;

fn main() -> Result<(), Box<dyn Error>> {
    let source: String = fs::read_to_string("examples/fib.lv")?;
    println!("{:?}", parse_source(source.as_str()));
    Ok(())
}
