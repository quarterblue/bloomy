mod argparser;
mod config;
mod equity;
mod fetcher;
mod portfolio;

use argparser::parsearg;
use argparser::*;
use colored::*;
use config::read_user_from_file;
use fetcher::Fetcher;
use portfolio::{PortList, Portfolio, Presult};
use std::env;
use std::error;
use std::io::{self, Write};
use std::string::String;

// The main entry thread of the application
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    run().await?;
    Ok(())
}

// Testing function for Reqwest get, prints a Todo list
async fn display_stock(todo_id: i32) -> Result<(), Error> {
    let url = format!("https://jsonplaceholder.typicode.com/todos/{}", todo_id);
    let res = reqwest::get(url).await?;
    println!("Status:{}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);
    Ok(())
}

// Prints the Bloomy logo
fn print_logo() {
    let logo = r###"
     ___    __
    / _ \  / / ___  ___   __ _   __ __
   / _  | / / / _ \/ _ \ /  ' \ / // /
  /____/ /_/  \___/\___//_/_/_/ \_, /
                               /___/"###;
    println!("{}", logo.green());
}

pub fn init() {
    println!("Initialized App");
}

// The main loop of the application, it is called in the main() function
async fn run() -> Result<(), Box<dyn error::Error>> {
    let _client = reqwest::Client::new();
    let mut port_tracker = PortList::empty_new();
    let args: Vec<String> = env::args().skip(1).collect();
    // load_config(&String::from("config.txt"))?;

    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut argparser: ArgParser;
    print_logo();

    let key = read_user_from_file("config.json").unwrap();
    let api_key = key.alpha_vantage;
    let fetcher = Fetcher::new("alpha_vantage".to_string(), api_key);

    // Main loop for gathering user command and parsing
    loop {
        let mut buffer = String::new();
        write!(stdout, "$ bloomy cmd> ")?;
        stdout.flush()?;
        stdin.read_line(&mut buffer)?;
        // write!(stdout, "You typed {}", buffer);
        if buffer.trim() == "q" || buffer.trim() == "quit" || buffer.trim() == "exit" {
            break;
        }
        // Argparser parses the user arguments and returns argparser with Enum Commands
        argparser = parsearg(&mut buffer)?;

        // Enum commands are matched and corresponding fetcher or renderer executes the commands
        match argparser.command {
            Some(Command::Equity(ECmd)) => match ECmd {
                ECmd::Price(ticker) => {
                    fetcher.search_equity(ticker).await?;
                }
                ECmd::Overview(ticker) => {
                    fetcher.equity_overview(ticker).await?;
                }
                _ => {
                    println!("Error: Equity")
                }
            },
            Some(Command::Portfolio(PCmd)) => match PCmd {
                PCmd::List => {
                    println!("List")
                }
                PCmd::ListPort(port) => {
                    println!("List Portfolio: {}", port)
                }
                PCmd::Make(port) => {
                    if let Presult::ADDED = port_tracker.add(port) {
                        println!("Portfolio Made");
                    } else {
                        println!("Portfolio name already exists, try a different name!");
                    }
                }
                _ => {
                    println!("Error: Make")
                }
            },
            Some(Command::Market) => display_stock(5).await?,
            Some(Command::Help) => println!("{}", "Display Help".cyan()),
            Some(Command::Load) => fetcher.search_equity_demo("ibm".to_string()).await?,
            _ => println!("Error"),
        }
        stdout.flush()?;
    }
    Ok(())
}

// Derives custom errors using thiserror crate
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
