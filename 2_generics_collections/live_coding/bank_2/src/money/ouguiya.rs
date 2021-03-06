use super::Money;

pub struct Ouguiya {
    amount: u64,
}

impl Ouguiya {
    pub fn new(amount: u64) -> Ouguiya {
        Ouguiya { amount }
    }
}

impl Money for Ouguiya {
    fn exchange_rate() -> f64 {
        0.3
    }

    fn amount(&self) -> f64 {
        self.amount as f64
    }
}

impl From<f64> for Ouguiya {
    fn from(amount: f64) -> Self {
        Ouguiya {
            amount: (amount / Ouguiya::exchange_rate()) as u64,
        }
    }
}

use std::fmt;

impl fmt::Display for Ouguiya {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ouguiya", self.amount)
    }
}
