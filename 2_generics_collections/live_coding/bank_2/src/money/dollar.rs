pub struct Dollar {
    amount: f64,
}

impl Dollar {
    pub fn new(amount: f64) -> Dollar {
        Dollar { amount }
    }
}

impl super::Money for Dollar {
    fn exchange_rate() -> f64 {
        1f64 // or 1.0
    }

    fn amount(&self) -> f64 {
        self.amount
    }
}

impl From<f64> for Dollar {
    fn from(amount: f64) -> Self {
        Dollar { amount }
    }
}

use std::fmt;

impl fmt::Display for Dollar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}$", self.amount)
    }
}
