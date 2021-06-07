mod argparser;
mod config;
mod equity;
mod fetcher;
mod portfolio;

use argparser::*;
use colored::*;
use config::read_user_from_file;
use equity::Equity;
use fetcher::Fetcher;
use portfolio::Portfolio;
use serde_json::Value;
use std::env;
use std::error;
use std::fs;
use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::String;
use tokio::task::*;

// Equity Commands:
// equity
// - Lists all equity commands
// equity [TICKER]
// - Search an equity TICKER
// equity [TICKER] overview
// - Fetch and render company overview of [TICKER]
// equity [TICKER] price
// - Fetch and render current price of [TICKER]
// equity [TICKER] price chart
// equity [TICKER] price historic [WEEK/MONTH/YEAR]
// equity [TICKER] an dcf
// equity [TICKER] an comp
//
// Portfolio Commands:
// port
// port list
// port switch [PORTFOLIO]
// port add [TICKER]
// port remove [TICKER]
// port delete
//
// config
//
// market
//
// help
//
// exit

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    run().await?;
    // let _ = display_stock(6).await?;
    Ok(())
}

// Testing Reqwest get
async fn display_stock(todo_id: i32) -> Result<(), Error> {
    let url = format!("https://jsonplaceholder.typicode.com/todos/{}", todo_id);
    let res = reqwest::get(url).await?;
    println!("Status:{}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);
    Ok(())
}

fn print_logo() {
    let logo = r###"
     ___    __
    / _ \  / / ___  ___   __ _   __ __
   / _  | / / / _ \/ _ \ /  ' \ / // /
  /____/ /_/  \___/\___//_/_/_/ \_, /
                               /___/"###;
    println!("{}", logo.green());
}

// fn init() {

// }

async fn run() -> Result<(), Box<dyn error::Error>> {
    let _client = reqwest::Client::new();
    let args: Vec<String> = env::args().skip(1).collect();
    // load_config(&String::from("config.txt"))?;

    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut argparser: ArgParser;
    print_logo();

    let key = read_user_from_file("config.json").unwrap();
    let api_key = key.alpha_vantage;
    let fetcher = Fetcher::new("alpha_vantage".to_string(), api_key);

    loop {
        let mut buffer = String::new();
        write!(stdout, "$ bloomy cmd> ")?;
        stdout.flush()?;
        stdin.read_line(&mut buffer)?;
        // write!(stdout, "You typed {}", buffer);
        if buffer.trim() == "q" || buffer.trim() == "quit" || buffer.trim() == "exit" {
            break;
        }
        argparser = parsearg(&mut buffer)?;

        match argparser.command {
            Some(Command::Equity) => display_stock(5).await?,
            Some(Command::Portfolio) => fetcher.search_equity_demo("ibm".to_string()).await?,
            Some(Command::Market) => println!("{}", "Market".blue()),
            Some(Command::Help) => println!("{}", "Display Help".cyan()),
            Some(Command::Load) => println!("Loading Config..."),
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
        "equity" => match arguments[2].to_lowercase().as_str().trim() {
            "" => {
                return Ok(ArgParser {
                    command: Some(Command::Equity),
                });
            }
            "price" => {
                return Ok(ArgParser {
                    command: Some(Command::Equity),
                });
            }
            _ => {
                return Ok(ArgParser {
                    command: Some(Command::Equity),
                });
            }
        },
        "portfolio" | "port" => {
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
        "load" => {
            return Ok(ArgParser {
                command: Some(Command::Load),
            });
        }
        _ => {
            println!("Well, it's an error!");
            return Ok(ArgParser { command: None });
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}
