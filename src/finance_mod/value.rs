use crate::rates::Rate;
use crate::currency::{CurrCode, Currency};
use std::fmt;
//use crate::finance_mod::currency::CurrCode;


pub struct Value {
    pub currency_code: CurrCode,
    pub amount: f64,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:.2}", self.currency_code.currency().symbol, self.amount)
    }
}

impl Value {
    pub fn new(amount: f64, currency_code: CurrCode) -> Value {
        Value{currency_code, amount }
    }

    pub fn convert(&self, start_cur_amount: Value, conversion_rate: Rate) -> Value {
        Value::new(start_cur_amount.amount * conversion_rate.rate,
                   start_cur_amount.currency_code)
    }

}
