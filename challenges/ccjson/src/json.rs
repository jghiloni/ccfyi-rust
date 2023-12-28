use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/json.pest"]
pub(crate) struct JSONParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    use std::path::PathBuf;
    use glob::glob;

    #[test]
    fn run_tests_with_valid() -> Result<(), Box<dyn std::error::Error>> {
        let mut results = Vec::<Result<PathBuf, Box<dyn std::error::Error>>>::new();
        for entry in glob("testdata/**/valid*.json").unwrap() {
            if let Ok(pathbuf) = entry {
                println!("Testing {}", pathbuf.clone().display());
                let data = std::fs::read_to_string(pathbuf.clone())?;
                results.push(match JSONParser::parse(Rule::document, &data) {
                    Ok(_) => Ok(pathbuf.clone()),
                    Err(e) => Err(Box::new(e)),
                });
            };
        }

        let num_failures = results.iter().fold(0, |acc, r| if r.is_err() { acc + 1 } else { acc } );
        assert_eq!(0, num_failures);

        Ok(())
    }

    #[test]
    fn run_tests_with_invalid() -> Result<(), Box<dyn std::error::Error>> {
        let mut results = Vec::<Result<PathBuf, Box<dyn std::error::Error>>>::new();
        for entry in glob("testdata/**/invalid*.json").unwrap() {
            if let Ok(pathbuf) = entry {
                println!("Testing {}", pathbuf.clone().display());
                let data = std::fs::read_to_string(pathbuf.clone())?;
                results.push(match JSONParser::parse(Rule::document, &data) {
                    Ok(_) => Ok(pathbuf.clone()),
                    Err(e) => Err(Box::new(e)),
                });
            };
        }

        let num_success = results.iter().fold(0, |acc, r| if r.is_ok() { acc + 1 } else { acc } );
        assert_eq!(0, num_success);

        Ok(())
    }
}
