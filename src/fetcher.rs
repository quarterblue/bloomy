use serde_json::Value;
use std::collections::HashMap;

use std::io;

// Stores a HashMap of all the platforms and their JSON API pairings
pub struct Fetcher {
    pub current: String,
    platforms: HashMap<String, HashMap<String, String>>,
}

impl Fetcher {
    pub fn new(provider: String) -> Self {
        let mut endpoints: HashMap<String, String> = HashMap::new();
        endpoints.insert("quote".to_string(), "GLOBAL_QUOTE&".to_string());
        let mut plats: HashMap<String, HashMap<String, String>> = HashMap::new();
        plats.insert("alpha_vantage".to_string(), endpoints);

        Fetcher {
            current: provider,
            platforms: plats,
        }
    }

    async fn fetch_equity(ticker: String) -> Result<(), Error> {
        let url = format!("https://jsonplaceholder.typicode.com/todos/{}", 5);
        let res = reqwest::get(url).await?;
        println!("Status:{}", res.status());
        let body = res.text().await?;
        let data = &body[..];
        let v: Value = serde_json::from_str(data)?;
        println!("Body:\n\n{}", body);
        Ok(())
    }

    // async fn search_equity(ticker: String) -> Result<(), reqwest::Error> {}
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
