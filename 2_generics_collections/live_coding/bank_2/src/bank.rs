use crate::money::Money;
use std::collections::HashMap;

pub struct Bank<T: Money> {
    accounts: HashMap<String, T>,
}

impl<T: Money> Bank<T> {
    // or impl<T> Bank<T> where T: Money
    pub fn new() -> Bank<T> {
        Bank {
            accounts: HashMap::new(),
        }
    }

    /// Return true if the account was created successfully, false otherwise
    pub fn add_account(&mut self, account_name: &str) -> bool {
        match self.accounts.get(account_name) {
            None => {
                self.accounts
                    .insert(String::from(account_name), T::from(0f64));
                true
            }
            Some(_) => false,
        }
    }

    pub fn add_money(&mut self, account_name: &str, money_amount: T) {
        let current_amount = match self.accounts.get(account_name) {
            None => T::from(0f64),
            Some(amount) => T::from(amount.dollar_value()),
        };

        self.accounts.insert(
            String::from(account_name),
            T::from(current_amount.dollar_value() + money_amount.dollar_value()),
        );
    }

    fn mean(&self) -> f64 {
        // // Sequential way
        // let mut sum = 0;

        // for (_, v) in &self.accounts {
        //     sum += v;
        // }

        // sum / self.accounts.len()

        self.accounts.iter().fold(0f64, |sum, (_, current_amount)| {
            sum + current_amount.dollar_value()
        }) / self.accounts.len() as f64
    }
}

use std::fmt;

impl<T> fmt::Display for Bank<T>
where
    T: Money,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mean monies: {}$, account holders:", self.mean())?;
        for (account_name, money) in &self.accounts {
            write!(f, "\n{}: {}", account_name, money)?;
        }

        Ok(())
    }
}
