mod argparser;
mod stocks;
extern crate reqwest;
// use reqwest::Error;

use argparser::*;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::String;
use stocks::Stock;

/**
 * UsageList :
 *
 * Single stock w/o command:
 *      bloomy AAPL
 *      bloomy apple
 *      bloomy tesla
 *
 * Single stock command w/ options:
 *  With chart option:
 *      bloomy apple -c
 *  With chart and long datalog:
 *      bloomy apple -cd
 *
 * Portfolio listing:
 *      bloomy portfolio
 *
 * Portfolio listing w/ options:
 *  With long option:
 *      bloomy portfolio -l
 *  With valuation options:
 *      bloomy portfolio -v
 */

// Main entry function
// #[cfg(not(target_arch = "wasm32"))]
// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     // run()?;
//     display_stock();
//     Ok(())
// }

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}

// Testing Reqwest get
async fn display_stock() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://jsonplaceholder.typicode.com/todos/1").await?;
    println!("Status:{}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);
    Ok(())
}

fn run() -> Result<(), Error> {
    let _client = reqwest::Client::new();
    let args: Vec<String> = env::args().skip(1).collect();
    // load_config(&String::from("config.txt"))?;

    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut argparser: ArgParser;

    loop {
        let mut buffer = String::new();
        write!(stdout, "Enter your command> ")?;
        stdout.flush()?;
        stdin.read_line(&mut buffer)?;
        write!(stdout, "You typed {}", buffer);
        if buffer.trim() == "q" {
            break;
        }
        argparser = parsearg(&mut buffer)?;

        match argparser.command {
            Some(Command::Stock) => println!("Stock now!"),
            Some(Command::Portfolio) => println!("Portfolio now!"),
            Some(Command::Market) => println!("Market now!"),
            Some(Command::Help) => println!("Help now!"),
            _ => println!("Nothing"),
        }
        stdout.flush()?;
    }
    Ok(())
}

fn parsearg(input: &mut String) -> Result<ArgParser, Error> {
    let split = input.split(" ");
    let arguments: Vec<&str> = split.collect();
    println!("{:?}", &input);
    match arguments[0].to_lowercase().as_str().trim() {
        "stock" => {
            return Ok(ArgParser {
                command: Some(Command::Stock),
            });
        }
        "portfolio" => {
            return Ok(ArgParser {
                command: Some(Command::Portfolio),
            });
        }
        "market" => {
            return Ok(ArgParser {
                command: Some(Command::Market),
            });
        }
        "help" => {
            return Ok(ArgParser {
                command: Some(Command::Help),
            });
        }
        _ => {
            println!("Well, it's an error!");
            return Ok(ArgParser { command: None });
        }
    }
}

/**
 * Config file loader, config.txt must be in the following format:
 *
 * Line 0: URL
 * Line 1: KEY
 *
 * Example:
 * https://www.stockapisite.com/api/v2/
 * OAdawjiofWA20489ajiofwajoi
 *
 */
fn load_config(configfile: &String) -> Result<Config, std::io::Error> {
    let path = Path::new(configfile);

    let config = match fs::File::open(&path) {
        Err(err) => panic!("Could not open config file, {}", err),
        Ok(config) => config,
    };

    let reader = BufReader::new(config);
    let mut url = String::from("");
    let mut key = String::from("");

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                if index == 0 {
                    url = String::from(&line);
                } else if index == 1 {
                    key = String::from(&line);
                }
                println!("{} {}", index, line)
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(Config { url, key })
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),

    #[error(transparent)]
    IoError(#[from] io::Error),
}
