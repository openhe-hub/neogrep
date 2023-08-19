use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub struct SearchResult {
    pub line_str: String,
    pub line_pos: i32,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args.");
        }
        let query = (&args[1]).to_string();
        let file_path = (&args[2]).to_string();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn read_file_content(file_path: &String) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    // println!("=================================");
    // println!("{}", content);
    // println!("=================================");

    Ok(content)
}

pub fn run(config: &Config, content: &String) {
    let res = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    println!("Result:");
    if res.is_empty() {
        println!("{} not found.", config.query);
    } else {
        for (idx, search_result) in res.iter().enumerate() {
            println!(
                "[{idx}] row = {}: {}",
                search_result.line_pos, search_result.line_str
            );
        }
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<SearchResult> {
    let mut res: Vec<SearchResult> = vec![];
    let mut row_idx = 0;

    for line in content.lines() {
        if line.contains(query) {
            res.push(SearchResult {
                line_str: line.to_string(),
                line_pos: row_idx,
            });
        }
        row_idx += 1;
    }

    res
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<SearchResult> {
    let mut res: Vec<SearchResult> = vec![];
    let insensitive_query = query.to_lowercase();
    let mut row_idx = 0;

    for line in content.lines() {
        if line.to_lowercase().contains(&insensitive_query) {
            res.push(SearchResult {
                line_str: line.to_string(),
                line_pos: row_idx,
            });
        }
        row_idx += 1;
    }

    res
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "duct a\nduct b\n dsdsd";
        // assert_eq!(vec!["duct a", "duct b"], search(query, content));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "duct a\nDuct b\n dsdsd";
        // assert_eq!(vec!["duct a"], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let content = "duct a\nDuct b\n dsdsd";
        // assert_eq!(
        //     vec!["duct a", "Duct b"],
        //     search_case_insensitive(query, content)
        // );
    }
}
