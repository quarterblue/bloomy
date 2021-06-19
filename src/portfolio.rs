use crate::equity::Equity;

pub enum Presult {
    ADDED,
    REMOVED,
    EXISTS,
    DNE,
}
pub struct PortList {
    pub portfolio_list: Vec<Portfolio>,
}

impl PortList {
    // Creates a new empty List of Portfolios
    pub fn empty_new() -> Self {
        PortList {
            portfolio_list: Vec::new(),
        }
    }

    // Adds a Portfolio to the global PortList, Presult is returned to indicate if successful
    // or unsuccesful if there already exists a portfolio of the indicated name
    pub fn add(&mut self, portfolio: String) -> Presult {
        if self.portfolio_list.iter().any(|p| p.name == portfolio) {
            println!("Portfolio name already exists. Please try another.");
            return Presult::EXISTS;
        } else {
            let new_portfolio = Portfolio::empty_new(portfolio);
            self.portfolio_list.push(new_portfolio);
            return Presult::ADDED;
        }
    }

    // *Implementation needs testing
    // Removes a Portfolio from the global PortList
    pub fn remove(&mut self, portfolio: String) -> Presult {
        if let Some(pos) = self
            .portfolio_list
            .iter()
            .position(|v| *v.name == portfolio)
        {
            self.portfolio_list.remove(pos);
            return Presult::REMOVED;
        } else {
            return Presult::DNE;
        }
    }

    // Prints the list of Portfolios
    pub fn list_portfolio(&mut self) {
        for port in self.portfolio_list.iter() {
            println!("{}", port.name);
        }
    }
}
pub struct Portfolio {
    pub name: String,
    pub stocks: Vec<Equity>,
    pub count: i32,
    pub totalvalue: i64,
}

impl Portfolio {
    pub fn empty_new(name: String) -> Self {
        Portfolio {
            name,
            stocks: Vec::new(),
            count: 0,
            totalvalue: 0,
        }
    }

    pub fn add_equity(&mut self, new_stock: String) -> Presult {
        for stock in &self.stocks {
            if stock.ticker == new_stock {
                return Presult::EXISTS;
            }
        }
        let new_equity = Equity::new(new_stock);
        self.stocks.push(new_equity);
        return Presult::ADDED;
    }

    pub fn remove_equity(&mut self, ticker: String) -> Presult {
        for stock in &self.stocks {
            if stock.ticker == ticker {
                // TODO
                // Remove Stock
                return Presult::REMOVED;
            }
        }
        return Presult::DNE;
    }

    pub fn calculate_total(&mut self) -> i64 {
        return 2000;
    }
    pub fn print_portfolio(&self) {}
}
