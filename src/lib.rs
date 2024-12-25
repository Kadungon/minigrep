use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new<T: Iterator<Item = String>>(args: &mut T) -> Result<Config, &'static str> {
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        args.next();

        let query = match args.next() {
            Some(val) => val,
            None => return Err("Didn't get a query string"),
        };

        let file_name = match args.next() {
            Some(val) => val,
            None => return Err("Didn't get a file path"),
        };

        Ok(Self {
            query,
            file_name,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    let results = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };

    if results.len() > 0 {
        println!("-------------------------lines found-------------------------");

        for line in results {
            println!("{line}");
        }
    } else {
        println!("No matching content found for query: {}", &config.query);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
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
    fn case_sensitive() {
        let content = "\
hello my name is rust.
I am compiled language,
developed for high performance and memory consumption 
    ";

        let query = "ang";

        assert_eq!(vec!["I am compiled language,"], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let content = "\
hello my name is rust.
I am compiled language,
developed for hiGh peRformance and memory consumption";

        let query = "eRf";

        assert_eq!(
            vec!["developed for hiGh peRformance and memory consumption"],
            search(query, content)
        );
    }
}
