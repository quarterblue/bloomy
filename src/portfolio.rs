use crate::equity::Equity;

pub enum PResult {
    ADDED,
    REMOVED,
    EXISTS,
    DNE,
}
pub struct Portfolio {
    pub stocks: Vec<Equity>,
    pub count: i32,
    pub totalvalue: i64,
}

impl Portfolio {
    pub fn empty_new() -> Self {
        Portfolio {
            stocks: Vec::new(),
            count: 0,
            totalvalue: 0,
        }
    }

    pub fn add_equity(&mut self, new_stock: Equity) -> PResult {
        for stock in &self.stocks {
            if stock.ticker == new_stock.ticker {
                return PResult::EXISTS;
            }
        }
        self.stocks.push(new_stock);
        return PResult::ADDED;
    }

    pub fn remove_equity(&mut self, ticker: String) -> PResult {
        for stock in &self.stocks {
            if stock.ticker == ticker {
                // TODO
                // Remove Stock
                return PResult::REMOVED;
            }
        }
        return PResult::DNE;
    }

    pub fn calculate_total(&mut self) -> i64 {
        return 2000;
    }
    pub fn print_portfolio() {}
}
