use std::env;
use std::error::Error;
use std::fs;

pub struct Config
{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config
{
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
        where T: Iterator<Item = String>
    {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file name"),
        };

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        case_sensitive = match args.next() {
            Some(arg) => {
                if arg == "--case_sensitive" {
                    true
                } else if arg == "--case_insensitive" {
                    false
                } else {
                    case_sensitive
                }
            }
            None => case_sensitive,
        };

        Ok(Config { query,
                    filename,
                    case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for matched_line in result {
        println!("{}", matched_line);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_create_config_success()
    {
        let args = vec![String::from("minigrep"),
                        String::from("query"),
                        String::from("poem.txt")];
        assert!(Config::new(args.into_iter()).is_ok());
    }

    #[test]
    fn test_create_config_not_enough_arguments()
    {
        let args = vec![String::from("minigrep")];
        assert!(Config::new(args.into_iter()).is_err());
    }

    #[test]
    fn test_run_success()
    {
        let args = vec![String::from("minigrep"),
                        String::from("query"),
                        String::from("poem.txt")];
        let config = Config::new(args.into_iter()).unwrap_or_else(|err| {
                         panic!("Failed to prepare config for testing run: {}", err);
                     });

        assert!(run(config).is_ok());
    }

    #[test]
    fn test_run_invalid_file_name()
    {
        let args = vec![String::from("minigrep"),
                        String::from("query"),
                        String::from("invalid")];
        let config = Config::new(args.into_iter()).unwrap_or_else(|err| {
                         panic!("Failed to prepare config for testing run: {}", err);
                     });

        assert!(run(config).is_err());
    }

    #[test]
    fn test_search_case_sensitive()
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."],
                   search_case_sensitive(query, contents));
    }

    #[test]
    fn test_search_case_insensitive()
    {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me!";

        assert_eq!(vec!["Rust:", "Trust me!"],
                   search_case_insensitive(query, contents));
    }
}
