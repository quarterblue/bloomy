extern crate reqwest;
// use reqwest::Error;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::String;

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

enum Command {
    Stock,
    Portfolio,
    Market,
    Lookup,
    Help,
    Load,
}

struct ArgParser {
    command: Option<Command>,
}

struct Stock {
    name: String,
    price: f32,
    marketcap: i64,
    high: f32,
    low: f32,
    volume: i32,
    ownedpercent: f32,
    ownedshares: u32,
}

struct Portfolio {
    stocks: Vec<Stock>,
    totalvalue: i64,
}

struct Config {
    url: String,
    key: String,
}

enum Currency {
    CAD,
    USD,
    EUR,
    GBP,
    JPY,
    DEFAULT,
}

struct StockArgs {
    stock: String,
    chart: bool,
    datalog: bool,
    currency: Currency,
}

impl StockArgs {
    fn new(args: &[String]) -> Result<StockArgs, &'static str> {
        if args.len() == 1 {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") {
                println!(
                    "Program Usage: bloomy stockname options
                \r\n      -h or -help to show help message
                \r\n      -c to show full charts
                \r\n      -d to show full data logs
                \r\n      -USD to show currency in us dollar 
                \r\n      -CAD to show currency in canadian dollar"
                );
                return Err("help");
            } else {
                return Ok(StockArgs {
                    stock: args[1].clone(),
                    chart: false,
                    datalog: false,
                    currency: Currency::DEFAULT,
                });
            }
        }
        if args.len() < 1 {
            return Err("Program Usage: bloomy stockname options");
        } else if args.len() > 5 {
            return Err("Too many arguments.");
        }
        let stock = args[1].clone();
        let options = args[2].clone();
        return Ok(StockArgs {
            stock,
            chart: false,
            datalog: false,
            currency: Currency::DEFAULT,
        });
    }
}

// Main entry function
fn main() -> Result<(), Error> {
    run()?;
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
        argparser = parsearg(&mut buffer)?;

        if buffer.trim() == "q" {
            break;
        }
        stdout.flush()?;
    }
    Ok(())
}

fn parsearg(input: &mut String) -> Result<ArgParser, Error> {
    let split = input.split(" ");
    let arguments: Vec<&str> = split.collect();
    match arguments[0].to_lowercase().as_str() {
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
