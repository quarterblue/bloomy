use std::io;

pub enum Command {
    Equity(ECmd),
    Portfolio(PCmd),
    Market,
    Lookup,
    Help,
    Load,
}

pub enum ECmd {
    Price(String),
    LogArg,
    Overview(String),
    Err,
    Chart,
    ListCommand,
    DCF,
    Comp,
}

pub enum PCmd {
    Add(String),
    Remove(String),
    ListPort(String),
    List,
    Make(String),
    Delete(String),
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
// equity [TICKER] dcf
// - Perform DCF calculation
// equity [TICKER] comp
// - Perform Comp calculation
//
// Portfolio Commands:
// port
// port list
// - List all portfolios
// post list [PORTFOLIO]
// - List all equity in [PORTFOLIO]
// port make [PORTFOLIO]
// - Make a new portfolio named [PORTFOLIO]
// port switch [PORTFOLIO]
// - Switch to portfolio name [PORTFOLIO]
// port add [TICKER]
// - Add [TICKER] equity into current portfolio
// port remove [TICKER]
// - Remove [TICKER] equity from current portfolio
// port delete [PORTFOLIO]
//
// Backtest Commands:
// backtest [PORTFOLIO] [START_DATE] [END_DATE]
// - Backtest a portfolio from start to end date
//
// config
//
// market
// - List all markets available for fetching
//
// help
// - List all available commands
//
// exit
// - Exits the program

// Argument parsing function for user typed commands
// It expects a string of text (words seperated by spaces)
// Example command would be:
// $ bloomy cmd> equity TSLA price
pub fn parsearg(input: &mut String) -> Result<ArgParser, Error> {
    let split = input.split(" ");
    let arguments: Vec<&str> = split.collect();
    let arguments_cleaned: Vec<String> = arguments
        .iter()
        .map(|el| el.trim().to_string().to_lowercase())
        .collect();
    match arguments[0].to_lowercase().as_str().trim() {
        "equity" => {
            if arguments.len() < 3 {
                println!("Error: not enough arguments");
                return Ok(ArgParser {
                    command: Some(Command::Equity(ECmd::Err)),
                });
            } else {
                match arguments[1].to_lowercase().as_str().trim() {
                    "overview" => {
                        return Ok(ArgParser {
                            command: Some(Command::Equity(ECmd::Overview(
                                arguments[2].trim().to_string(),
                            ))),
                        });
                    }
                    "price" => {
                        return Ok(ArgParser {
                            command: Some(Command::Equity(ECmd::Price(
                                arguments[2].trim().to_string(),
                            ))),
                        });
                    }
                    "dcf" => {
                        return Ok(ArgParser {
                            command: Some(Command::Equity(ECmd::DCF)),
                        });
                    }
                    "comp" => {
                        return Ok(ArgParser {
                            command: Some(Command::Equity(ECmd::Comp)),
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
            if arguments.len() < 3 {
                println!("Error: not enough arguments");
                return Ok(ArgParser {
                    command: Some(Command::Equity(ECmd::Err)),
                });
            } else if arguments.len() == 2 {
                return Ok(ArgParser {
                    command: Some(Command::Equity(ECmd::Err)),
                });
            } else {
                match arguments[1].to_lowercase().as_str().trim() {
                    "make" => {
                        return Ok(ArgParser {
                            command: Some(Command::Portfolio(PCmd::Make(
                                arguments[2].trim().to_string(),
                            ))),
                        });
                    }
                    "list" => {
                        return Ok(ArgParser {
                            command: Some(Command::Portfolio(PCmd::ListPort(
                                arguments[2].trim().to_string(),
                            ))),
                        });
                    }
                    "switch" => {
                        return Ok(ArgParser {
                            command: Some(Command::Portfolio(PCmd::Add(
                                arguments[2].trim().to_string(),
                            ))),
                        });
                    }
                    "add" => {
                        return Ok(ArgParser {
                            command: Some(Command::Portfolio(PCmd::Add(
                                arguments[2].trim().to_string(),
                            ))),
                        });
                    }
                    "remove" => {
                        return Ok(ArgParser {
                            command: Some(Command::Equity(ECmd::Err)),
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
