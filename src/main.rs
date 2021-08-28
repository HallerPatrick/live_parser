
pub mod parser;

use std::fs;
use std::error::Error;

use parser::parse_source;


fn main() -> Result<(), Box<dyn Error>> {
    let source: String = fs::read_to_string("../examples/func.lv")?;
    println!("{:?}", parse_source(source.as_str()));
    Ok(())
}
