use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("You must provide a query and a filename!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut contents = String::new();
    File::open(config.filename)?
        .read_to_string(&mut contents)?;
    
    let results = match config.case_sensitive {
        true => search(&config.query, &contents),
        false => search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); 

    for line in text.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); 
    let query = query.to_lowercase();

    for line in text.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_finds_one_result() {
        let query = "stupid";
        let text = "\
Two things are infinite:
the universe and human stupidity;
and I'm not sure about the universe.";

        assert_eq!(
            vec!["the universe and human stupidity;"],
            search(query, text)
        );
    }

    #[test]
    fn case_insensitive_search() {
        let query = "STUpiD";
        let text = "\
Two things are infinite:
the universe and human stupidity;
and I'm not sure about the universe.";

        assert_eq!(
            vec!["the universe and human stupidity;"],
            search_case_insensitive(query, text)
        );
    }
}
