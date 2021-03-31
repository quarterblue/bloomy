extern crate reqwest;
// use reqwest::Error;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
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
fn main() -> Result<(), std::io::Error> {
    run().expect("Something went wrong!");
    Ok(())
}

fn run() -> Result<(), std::io::Error> {
    let _client = reqwest::Client::new();
    let args: Vec<String> = env::args().skip(1).collect();
    load_config(&String::from("config.txt")).expect("Something went wrong!");
    Ok(())
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
