pub enum Command {
    Stock,
    Portfolio,
    Market,
    Lookup,
    Help,
    Load,
}

pub struct ArgParser {
    pub command: Option<Command>,
}

pub struct Config {
    pub url: String,
    pub key: String,
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
