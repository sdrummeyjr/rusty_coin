#![macro_use]
use crate::rates::Rate;
use crate::currency::{CurrCode, Currency, curr_parity};
use std::fmt;
use std::error::Error;
use std::ops::{Deref, Add, AddAssign, SubAssign, Sub, Mul, MulAssign, Div, DivAssign};


#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Value {
    pub currency_code: CurrCode,
    pub amount: f64,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:.2}", self.currency_code.currency().symbol, self.amount)
    }
}

// todo - build a macro to ensure currency parity for equations (checks fn to ensure CurrCode is eq)
impl Value {
    pub fn new(amount: f64, currency_code: CurrCode) -> Value {
        Value{currency_code, amount }
    }

    pub fn new_vec(vector_of_num: Vec<f64>, currency_code: CurrCode) -> Vec<Value> {
        vector_of_num.iter().map(|f| Value::new(*f, currency_code)).collect()
    }

    pub fn convert(&self, conversion_rate: Rate, new_curr_code: CurrCode) -> Value {
        Value::new(self.amount * conversion_rate.rate,
                   new_curr_code)
    }

    // method that takes a string and parses it into a value
    // need to include a CurrCode b/c symbols are used with diff curr (e.g. $ dollar is common)
//    pub fn value_parse_from_str(string: &str, exp_curr_code: CurrCode) -> Result<Value, Box<dyn Error>> {
//        Ok()
//    } // todo return a result


    pub fn value_to_string(&self) -> String {
        self.to_string()
    }

    // involves leaking the memory of the String...use with caution
    pub fn value_to_str(&self) -> &'static str {
        Box::leak(self.to_string().into_boxed_str())
    }

}

/// Function that takes a vector to convert to new currency
pub fn convert_vec(values: &Vec<Value>, new_curr: CurrCode, exchange_rate: Rate) -> Vec<Value> {
    values.iter().map(|v| v.convert(exchange_rate, new_curr)).collect()
}

/// Function that takes two vectors of values to be combined, while converting one currency (vec_to)
/// to the other (vec_from)
pub fn combine_and_convert(vec_to: &Vec<Value>, vec_from: &Vec<Value>, exchange_rate: Rate) -> Vec<Value> {
    let new_curr_code = curr_parity(&vec_to);
    let new_val_vec: Vec<Value> = convert_vec(vec_from, new_curr_code, exchange_rate);
    vec_to.iter().copied().chain(new_val_vec.into_iter()).collect::<Vec<Value>>()
}


macro_rules! value {
    ($cur:expr, $amount:expr) => {
        Value::new(
            $cur,
            $amount
        )
    };
}


impl Add for Value {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if self.currency_code == other.currency_code {
            Self {
                amount: self.amount + other.amount,
                currency_code: self.currency_code
            }
        } else {
            panic!("No currency parity")  // todo - improve error handling
        }
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, other: Self) {
        if self.currency_code == other.currency_code {
            *self = Self {
                amount: self.amount + other.amount,
                currency_code: self.currency_code
            }
        } else {
            panic!("No currency parity")  // todo - improve error handling
        }
    }
}


impl Sub for Value {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        if self.currency_code == other.currency_code {
            Self {
                amount: self.amount - other.amount,
                currency_code: self.currency_code
            }
        } else {
            panic!("No currency parity")  // todo - improve error handling
        }
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, other: Self) {
        if self.currency_code == other.currency_code {
            *self = Self {
                amount: self.amount - other.amount,
                currency_code: self.currency_code
            }
        } else {
            panic!("No currency parity")  // todo - improve error handling
        }
    }
}


impl Mul for Value {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        if self.currency_code == other.currency_code {
            Self {
                amount: self.amount * other.amount,
                currency_code: self.currency_code
            }
        } else {
            panic!("No currency parity")  // todo - improve error handling
        }
    }
}

impl MulAssign for Value {
    fn mul_assign(&mut self, other: Self) {
        if self.currency_code == other.currency_code {
            *self = Self {
                amount: self.amount * other.amount,
                currency_code: self.currency_code
            }
        } else {
            panic!("No currency parity")  // todo - improve error handling
        }
    }
}


impl Div for Value {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        if self.currency_code == other.currency_code {
            Self {
                amount: self.amount / other.amount,
                currency_code: self.currency_code
            }
        } else {
            panic!("No currency parity")  // todo - improve error handling
        }
    }
}

impl DivAssign for Value {
    fn div_assign(&mut self, other: Self) {
        if self.currency_code == other.currency_code {
            *self = Self {
                amount: self.amount / other.amount,
                currency_code: self.currency_code
            }
        } else {
            panic!("No currency parity")  // todo - improve error handling
        }
    }
}
