extern crate reqwest;
// use reqwest::Error;
use std::env;

enum Currency {
    CAD,
    USD,
}

struct StockArgs {
    stock: String,
    chart: String,
    datalog: String,
    currency: String,
}

fn main() {
    let _client = reqwest::Client::new();
    let args: Vec<String> = env::args().skip(1).collect();

    let stockArgs = parse_stock(&args);

    println!("{:?}", args);
}

fn parse_stock(args: &[String]) -> StockArgs {
    let stock = args[0].clone();
    let chart = args[1].clone();
    let datalog = args[2].clone();
    let currency = args[3].clone();

    StockArgs {
        stock,
        chart,
        datalog,
        currency,
    }
}
