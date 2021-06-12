use crate::config::ApiList;
use crate::equity::Equity;
use serde_json::Value;
use std::collections::HashMap;
use std::error;
use std::io;

#[derive(Debug)]
// Stores a HashMap of all the platforms and their JSON API pairings
pub struct Fetcher {
    pub current: String,
    pub platforms: HashMap<String, HashMap<String, String>>,
    pub apilist: ApiList,
    pub global_quote: Vec<(String, String)>,
    api_key: String,
}

impl Fetcher {
    pub fn new(provider: String, api_key: String) -> Self {
        let global_quote: Vec<(String, String)> = vec![
            ("01. symbol".to_string(), "ticker".to_string()),
            ("02. open".to_string(), "open".to_string()),
            ("03. high".to_string(), "high".to_string()),
            ("04. low".to_string(), "low".to_string()),
            ("05. price".to_string(), "price".to_string()),
            ("06. volume".to_string(), "volume".to_string()),
            ("07. latest trading day".to_string(), "ltd".to_string()),
        ];

        let mut endpoints: HashMap<String, String> = HashMap::new();
        endpoints.insert("quote".to_string(), "GLOBAL_QUOTE&".to_string());

        let mut plats: HashMap<String, HashMap<String, String>> = HashMap::new();
        plats.insert("alpha_vantage".to_string(), endpoints);

        Fetcher {
            current: provider,
            platforms: plats,
            apilist: ApiList::new(),
            global_quote,
            api_key,
        }
    }

    pub async fn fetch_equity(ticker: String) -> Result<(), Error> {
        let url = format!("https://alphavantage.co/query?function={}", 5);
        let res = reqwest::get(url).await?;
        println!("Status:{}", res.status());
        let body = res.text().await?;
        let data = &body[..];
        let v: Value = serde_json::from_str(data)?;
        println!("Body:\n\n{}", body);
        Ok(())
    }

    pub async fn fetch_equity_price(&self, equity: &Equity) -> Result<(), Error> {
        let url = format!(
            "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            &equity.ticker, &self.api_key
        );
        let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
        println!("{:?}", resp);
        Ok(())
    }

    pub async fn search_equity(&self, ticker: String) -> Result<(), Error> {
        let url = format!(
            "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            ticker, &self.api_key
        );
        let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
        let quotes = resp.get("Global Quote").unwrap();
        for x in &self.global_quote {
            let value = quotes.get(&x.0).unwrap();
            let title = &x.1;
            println!("{}: {}", title, value);
        }
        println!("");
        Ok(())
    }

    // Searches an equity by ticker and outputs a list of global quote information:
    // Ticker: IBM
    // Open: 150.4300
    // High: 151.8450
    // Low: 150.3700
    // Price: 150.2800
    // Volume: 3421395
    // Latest Trading Day: 2021-06-11
    // Previous Close: 150.5400
    // Change: 0.700
    // Change Percent: 0.4916%
    pub async fn search_equity_demo(&self, ticker: String) -> Result<(), Error> {
        let url = format!(
            "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=IBM&apikey=demo",
        );
        let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
        let quotes = resp.get("Global Quote").unwrap();
        for x in &self.global_quote {
            let value = quotes.get(&x.0).unwrap();
            let title = &x.1;
            println!("{}: {}", title, value);
        }
        println!("");
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_price() {}
}
