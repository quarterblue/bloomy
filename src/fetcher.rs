use crate::config::ApiList;
use crate::equity::Equity;
use serde_json::Value;
use std::collections::HashMap;
use std::io;

// Stores a HashMap of all the platforms and their JSON API pairings
pub struct Fetcher {
    pub current: String,
    pub platforms: HashMap<String, HashMap<String, String>>,
    pub apilist: ApiList,
    pub global_quote: Vec<(String, String)>,
}

impl Fetcher {
    pub fn new(provider: String) -> Self {
        let global_quote: Vec<(String, String)> =
            vec![("01. symbol".to_string(), "ticker".to_string())];
        let mut endpoints: HashMap<String, String> = HashMap::new();
        endpoints.insert("quote".to_string(), "GLOBAL_QUOTE&".to_string());
        let mut plats: HashMap<String, HashMap<String, String>> = HashMap::new();
        plats.insert("alpha_vantage".to_string(), endpoints);

        Fetcher {
            current: provider,
            platforms: plats,
            apilist: ApiList::new(),
            global_quote,
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

    pub async fn fetch_equity_price(equity: &Equity) -> Result<(), Error> {
        let url = format!(
            "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey=demo",
            &equity.ticker
        );
        let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
        Ok(())
    }

    pub async fn search_equity(ticker: String) -> Result<(), Error> {
        Ok(())
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
