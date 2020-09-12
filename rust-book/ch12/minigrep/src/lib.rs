use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = match args.next() {
            Some(s) => s,
            None => return Err("didn't get a query"),
        };

        let filename = match args.next() {
            Some(s) => s,
            None => return Err("didn't get filename"),
        };

        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    let search_fn = if config.case_insensitive {
        search_case_insensitive
    } else {
        search
    };

    for line in search_fn(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

// use the lifetime of "content" because we want the output is slice of
// sub-strings of "content"
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive
pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive
pick three.
Trust me";

        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, content)
        );
    }
}
