pub struct Portfolio {
    pub stocks: Vec<Stock>,
    pub totalvalue: i64,
}

impl Portfolio {
    pub fn empty_new() -> Self {
        Portfolio {
            stocks: Vec::new(),
            count: 0,
            totavalue: 0,
        }
    }

    pub fn add_stock(&mut self, new_stock: Stock) -> Result<Ok, Err> {
        for stock in &self.stocks {
            if (stock.ticker == new_stock.ticker) {
                Err()
            }
        }
        self.stocks.push(new_stock);
        Ok()
    }

    pub fn remove_stock(&mut self, ticker: String) -> Result<Ok, Err> {
        for stock in &self.stocks {
            if (stock.ticker == ticker) {
                // TODO
                // Remove Stock
                Ok()
            }
        }
        Err()
    }
    pub fn print_portfolio() {}
}
