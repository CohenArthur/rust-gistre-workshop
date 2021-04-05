pub mod dollar;
pub mod euro;
pub mod ouguiya;

pub use dollar::Dollar;
pub use euro::Euro;
pub use ouguiya::Ouguiya;

use std::fmt;

pub trait Money: fmt::Display + From<f64> {
    // Represent the exchange rate between this money and dollars
    fn exchange_rate() -> f64;

    // Represent the amount of money held in that currency
    fn amount(&self) -> f64;

    fn dollar_value(&self) -> f64 {
        self.amount() * Self::exchange_rate()
    }
}
