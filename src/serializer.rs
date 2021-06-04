pub trait Serialize {
    fn quotes() -> Vec<(String, String)>;
    fn earnings() -> Vec<(String, String)>;
    fn company_overview() -> Vec<(String, String)>;
    fn quote_search() -> Vec<(String, String)>;
}

pub struct AlphaVantage {
    pub quotes: Vec<(String, String)>,
    pub earnings: Vec<(String, String)>,
    pub company_overview: Vec<(String, String)>,
    pub quote_search: Vec<(String, String)>,
}

impl Serialize for AlphaVantage {
    fn quotes() -> Vec<(String, String)> {}
}

impl AlphaVantage {
    pub fn new() {
        let global_quotes: Vec<(String, String)> = vec![
            ("01. symbol".to_string(), "ticker".to_string()),
            ("02. open".to_string(), "open".to_string()),
            ("03. high".to_string(), "high".to_string()),
            ("04. low".to_string(), "low".to_string()),
            ("05. price".to_string(), "price".to_string()),
            ("06. volume".to_string(), "volume".to_string()),
            ("07. latest trading day".to_string(), "ltd".to_string()),
            (
                "08. previous close".to_string(),
                "previous_close".to_string(),
            ),
            ("09. change".to_string(), "change".to_string()),
            (
                "10. change percent".to_string(),
                "change_percent".to_string(),
            ),
        ];
    }
}
