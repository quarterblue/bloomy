use std::collections::HashMap;

const equity_api_url: String = String::from("https://jsonplaceholder.typicode.com/todos/");

// Stores a HashMap of all the platforms and their JSON API pairings
pub struct Fetcher {
  current: String,
  platforms: HashMap<String, HashMap<String, String>>,
}

impl Fetcher {

  fn new(provider: String) -> Self {

  }

  async fn fetch_equity(ticker: String) -> Result<(), reqwest::Error> {
      let url = format!("https://jsonplaceholder.typicode.com/todos/{}", todo_id);
      let res = reqwest::get(url).await?;
      println!("Status:{}", res.status());
      let body = res.text().await?;
      println!("Body:\n\n{}", body);
      Ok(())
  }

  async fn search_equity(ticker: String) -> Result<(), reqwest::Error> {
  }
}
