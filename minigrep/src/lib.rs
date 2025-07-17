use std::{fs, env};
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Searches a string for the provided query string. Splits the searched string by \n
/// and returns a vector with each line that matched the query
/// 
/// # Examples
/// 
/// ```
/// let query = "ell";
/// let contents = "hello\nworld";
/// 
/// let result = minigrep::search(query, contents);
/// assert_eq!(vec!["hello"], result);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub struct EnvVars {
    pub ignore_case: bool
}

impl EnvVars {
    pub fn new() -> EnvVars {
        EnvVars { 
            ignore_case: env::var("IGNORE_CASE").is_ok() 
        }
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>, env_vars: EnvVars) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string and file path arguments")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file path argument")
        };
        let ignore_case: bool= match args.next() {
            Some(arg) => arg.parse().unwrap_or(env_vars.ignore_case),
            None => env_vars.ignore_case
        };

        Ok(Config { query, file_path, ignore_case })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }

    #[test]
    fn ignore_case_prioritizes_command_line_argument_over_env_var() {
        let args: Vec<String> = vec![String::from("progarm path"), String::from("query"), String::from("file path"), String::from("true")];
        let env_vars = EnvVars { ignore_case: false };
        let config = Config::build(args.into_iter(), env_vars).expect("Unexpected problem building config");
        assert!(config.ignore_case);
    }
}