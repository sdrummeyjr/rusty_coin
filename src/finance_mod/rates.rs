use crate::value::Value;
use std::fmt;
use std::fmt::Error;
//use std::f64::;
//use math::
//use crate::finance_mod::value::Value;

// https://doc.rust-lang.org/std/default/trait.Default.html

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Precision {  // todo change to precision
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Precision {
    fn set_round(&self) -> &str {
        match self {
            Precision::One => "{:.1}",
            Precision::Two => "{:.2}",
            Precision::Three => "{:.3}",
            Precision::Four => "{:.4}",
            Precision::Five => "{:.5}",
        }
    }
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

    // todo change from returning f64 to returning Result<Rate, Error>
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

    pub fn exchange_rate(&self, start_cur_amount: Value, new_cur_amount: Value) -> Rate {
        Rate::new(start_cur_amount.amount / new_cur_amount.amount)
    }

//    pub fn internal_rate_of_return()


}


//
//    fn get_round(r: &Rate) -> Precision {
//        match r.precision {
//            1 => Precision::One,
//            2 => Precision::Two,
//            3 => Precision::Three,
//            4 => Precision::Four,
//            5 => Precision::Five,
//            _ => panic!("Pick 1 - 5")
//        }
//    }

//    fn round(&self, r: &Rate) -> f64 {
//        match self {
//            Precision::One => r.rate.round(1),
//            Precision::Two => "{:.2}",
//            Precision::Three => "{:.3}",
//            Precision::Four => "{:.4}",
//            Precision::Five => "{:.5}",
//        }
//    }

