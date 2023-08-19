use std::env;
use std::process;

use neo_grep::*;

fn main() {
    // parse config
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem occurs when parsing args: {err}");
        process::exit(1);
    });
    println!("config = {:?}", config);

    // read file content
    let content = read_file_content(&config.file_path).unwrap_or_else(|err| {
        eprintln!("Application Err: {err}");
        process::exit(1);
    });

    // search
    run(&config, &content);
}
