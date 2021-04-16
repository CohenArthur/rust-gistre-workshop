use super::Money;

// Let's make euros a bit more interesting and keep an amount of cents and an amount of
// integers
pub struct Euro {
    integer: u64,
    decimal: u64,
}

impl Euro {
    pub fn new(amount: u64) -> Euro {
        Euro {
            integer: amount,
            decimal: 0,
        }
    }
}

impl Money for Euro {
    fn amount(&self) -> f64 {
        self.integer as f64 + self.decimal as f64 / 100f64
    }

    fn exchange_rate() -> f64 {
        1.17
    }
}

impl From<f64> for Euro {
    fn from(amount: f64) -> Self {
        let amount = amount / Euro::exchange_rate();
        let integer_amount = amount as u64;
        let decimal_amount = amount - integer_amount as f64;

        Euro {
            integer: integer_amount,
            decimal: decimal_amount as u64,
        }
    }
}

use std::fmt;

impl fmt::Display for Euro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{} euros", self.integer, self.decimal)
    }
}
