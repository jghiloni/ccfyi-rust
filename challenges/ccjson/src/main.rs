use std::io::{stdin, BufReader, Read};
use pest::Parser;

mod json;

use json::{JSONParser, Rule};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = stdin().lock();

    let mut data = String::new();
    let reader = &mut BufReader::new(input);
    let _ = reader.get_mut().read_to_string(&mut data);

    match JSONParser::parse(Rule::document, data.as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
