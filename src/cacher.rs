use crate::equity::Equity;
pub trait ListHolder {
    fn add_equity(&mut self, ticker: String) -> Cresult;
    fn remove_equity(&mut self, ticker: String) -> Cresult;
    fn find_equity(&mut self, ticker: String) -> Cresult;
}

enum Cresult {
    Added,
    Moved,
    DoesNotExist,
    Exist,
}

// Cacher creates and holds the equity data when the user first initiates the search
// When the user adds the equity to their portfolio, the cacher moves the cached equity to its respective portfolio
pub struct Cacher {
    equity_list: Vec<Equity>,
}

impl ListHolder for Cacher {
    // fn add_portfolio(&mut self, name: String) -> Cresult {
    //     if self.equity_list.iter().any(|e| e.ticker == item.ticker()) {
    //         println!("Portfolio name already exists. Please try another.");
    //         return Cresult::Exist;
    //     } else {
    //         let new_portfolio = Portfolio::empty_new(portfolio);
    //         self.portfolio_list.push(new_portfolio);
    //         return Cresult::Added;
    //     }
    // }

    fn add_equity(&mut self, ticker: String) -> Cresult {
        if self.equity_list.iter().any(|e| e.ticker == ticker) {
            println!("Equity already exists.");
            return Cresult::Exist;
        } else {
            let new_equity = Equity::new(ticker);
            self.equity_list.push(new_equity);
            return Cresult::Added;
        }
    }

    fn remove_equity(&mut self, ticker: String) -> Cresult {
        todo!()
    }

    fn find_equity(&mut self, ticker: String) -> Cresult {
        let index = self.equity_list.iter().position(|r| r.ticker == ticker);
        if let Some(x) = index {
            return Cresult::Exist;
        } else {
            return Cresult::DoesNotExist;
        }
    }
}
