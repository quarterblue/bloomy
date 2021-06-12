use std::io;

pub enum Command {
    Equity(ECmd),
    Portfolio,
    Market,
    Lookup,
    Help,
    Load,
}

pub enum ECmd {
    Price,
    LogArg,
    Overview,
    Err,
}

pub struct ArgParser {
    pub command: Option<Command>,
}

pub enum Currency {
    CAD,
    USD,
    EUR,
    GBP,
    JPY,
    DEFAULT,
}

pub struct StockArgs {
    stock: String,
    chart: bool,
    datalog: bool,
    currency: Currency,
}

impl StockArgs {
    pub fn new(args: &[String]) -> Result<StockArgs, &'static str> {
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

// Argument parsing function for user typed commands
// It expects a string of text (words seperated by spaces)
// Example command would be:
// $ bloomy cmd> equity TSLA price
pub fn parsearg(input: &mut String) -> Result<ArgParser, Error> {
    let split = input.split(" ");
    let arguments: Vec<&str> = split.collect();
    match arguments[0].to_lowercase().as_str().trim() {
        "equity" => {
            if arguments.len() < 3 {
                println!("Error: not enough arguments");
                return Ok(ArgParser {
                    command: Some(Command::Equity(ECmd::Err)),
                });
            } else {
                match arguments[2].to_lowercase().as_str().trim() {
                    "overview" => {
                        return Ok(ArgParser {
                            command: Some(Command::Equity(ECmd::Overview)),
                        });
                    }
                    "price" => {
                        return Ok(ArgParser {
                            command: Some(Command::Equity(ECmd::Price)),
                        });
                    }
                    _ => {
                        return Ok(ArgParser {
                            command: Some(Command::Equity(ECmd::Err)),
                        });
                    }
                }
            }
        }
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

// Derives custom errors using thiserror crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}
