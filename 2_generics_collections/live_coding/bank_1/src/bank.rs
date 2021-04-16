use std::collections::HashMap;

pub type Money = u64;

pub struct Bank {
    accounts: HashMap<String, Money>,
}

impl Bank {
    pub fn new() -> Bank {
        Bank {
            accounts: HashMap::new(),
        }
    }

    /// Return true if the account was created successfully, false otherwise
    pub fn add_account(&mut self, account_name: &str) -> bool {
        match self.accounts.get(account_name) {
            None => {
                self.accounts.insert(String::from(account_name), 0);
                true
            }
            Some(_) => false,
        }
    }

    /// Return true if the account was updated successfully, false if it didn't
    /// exist already
    pub fn add_money(&mut self, account_name: &str, money_amount: Money) -> bool {
        match self.accounts.get_mut(account_name) {
            None => false,
            Some(amount) => {
                *amount += money_amount;
                true
            }
        }
    }

    fn mean(&self) -> Money {
        // // Sequential way
        // let mut sum = 0;

        // for (_, v) in &self.accounts {
        //     sum += v;
        // }

        // sum / self.accounts.len()

        self.accounts
            .iter()
            .fold(0, |sum, (_, current_amount)| sum + current_amount)
            / self.accounts.len() as u64
    }
}

use std::fmt;

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mean monies: {}, account holders:", self.mean())?;
        for (account_name, _) in &self.accounts {
            write!(f, "\n{}", account_name)?;
        }

        Ok(())
    }
}
