use crate::rates::Rate;
use std::fmt;

// have default to USD
// add crypto
pub enum Currency {
    USD,
    GBP,
    EUR,
    JPY,
}

// https://stevedonovan.github.io/rust-gentle-intro/2-structs-enums-lifetimes.html#simple-enums
impl Currency {
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::USD => "$",
            Currency::GBP => "£",
            Currency::EUR => "€",
            Currency::JPY => "¥",
        }
    }

}

pub struct Value {
    pub currency: Currency,
    pub amount: f64,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:.2}", self.currency.symbol(), self.amount)
    }
}

impl Value {
    pub fn new(amount: f64, currency: Currency) -> Value {
        Value{currency: currency, amount: amount}
    }

    pub fn convert(&self, start_cur_amount: Value, conversion_rate: Rate) -> Value {
        Value::new(start_cur_amount.amount * conversion_rate.rate,
                   start_cur_amount.currency)
    }

}
