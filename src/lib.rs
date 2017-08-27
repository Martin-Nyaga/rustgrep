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
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // Skip binary name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("You must provide a search query")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("You must provide a file name")
        };

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
    text.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    text.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
