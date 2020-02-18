use crate::value::Value;
use std::fmt;
use std::fmt::Error;


#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Precision {  // todo change to precision
    One,
    Two,
    Three,
    Four,
    Five,
    None,
}


#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Rate {
    pub rate: f64,
    pub precision: Precision
}

impl fmt::Display for Rate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.precision {
            Precision::One => write!(f, "{:.1}%", self.rate),
            Precision::Two => write!(f, "{:.2}%", self.rate),
            Precision::Three => write!(f, "{:.3}%", self.rate),
            Precision::Four => write!(f, "{:.4}%", self.rate),
            Precision::Five => write!(f, "{:.5}%", self.rate),
            Precision::None => write!(f, "{}%", self.rate)
        }
    }
}

impl Rate {
    pub fn new(new_rate: f64) -> Rate {
        Rate {
            rate: new_rate,
            precision: Precision::Two
        }
    }

    pub fn new_vec(vector_of_num: Vec<f64>) -> Vec<Rate> {
        vector_of_num.iter().map(|f| Rate::new(*f)).collect()
    }

    pub fn exchange_rate(start_cur_amount: Value, new_cur_amount: Value) -> Rate {
        Rate::new(start_cur_amount.amount / new_cur_amount.amount)
    }

    pub fn rate_to_string(&self) -> String {
        self.to_string()
    }

    /// method to change an rate instance's precision
    pub fn change_precision(&mut self, precision: Precision) {
        self.precision = precision
    }

    // involves leaking the memory of the String...use with caution
    pub fn rate_to_str(&self) -> &'static str {
        Box::leak(self.to_string().into_boxed_str())
    }

    pub fn periodic_rate(&self, num_periods: usize) -> Rate {
        Rate::new(((1.0 + self.rate as f64).powf(1.0 / num_periods as f64) - 1.0))
    }

    pub fn effective_rate(&self, num_periods: usize) -> Rate {
        if num_periods <= 0 {
            // https://doc.rust-lang.org/std/macro.panic.html
            panic!("The number of periods must be greater than 0. Got {}", num_periods)
        }
        Rate::new((1.0 + self.rate / num_periods as f64).powf(num_periods as f64) - 1.0)
    }

    pub fn nominal_rate(&self, num_periods: usize) -> Rate {
        if num_periods <= 0 {
            // https://doc.rust-lang.org/std/macro.panic.html
            panic!("The number of periods must be greater than 0. Got {}", num_periods)
        }
        Rate::new(num_periods as f64 *
            ((self.rate + 1.0).powf(1.0 / num_periods as f64) - 1.0))
    }

//    pub fn internal_rate_of_return()

}

