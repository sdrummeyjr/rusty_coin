use std::fmt;
use std::fmt::Error;

// https://doc.rust-lang.org/std/default/trait.Default.html
enum Rounding {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Rounding {
    fn set_round(&self) -> &str {
        match self {
            Rounding::One => "{:.1}",
            Rounding::Two => "{:.2}",
            Rounding::Three => "{:.3}",
            Rounding::Four => "{:.4}",
            Rounding::Five => "{:.5}",
        }
    }
}

impl Default for Rounding {
    fn default() -> Self { Rounding::Two }
}


pub struct Rate {
    pub rate: f64
}

impl fmt::Display for Rate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}%", self.rate)
    }
}

impl Rate {
    pub fn new(new_rate: f64) -> Rate {
        Rate { rate: new_rate }
    }


    pub fn periodic_rate(&self, num_periods: usize) -> f64 {
        ((1.0 + self.rate as f64).powf(1.0 / num_periods as f64) - 1.0)
    }


    pub fn effective_rate(&self, num_periods: usize) -> f64 {
        if num_periods <= 0 {
            // https://doc.rust-lang.org/std/macro.panic.html
            panic!("The number of periods must be greater than 0. Got {}", num_periods)
        }
        (1.0 + self.rate / num_periods as f64).powf(num_periods as f64) - 1.0
    }


    pub fn nominal_rate(&self, num_periods: usize) -> f64 {
        if num_periods <= 0 {
            // https://doc.rust-lang.org/std/macro.panic.html
            panic!("The number of periods must be greater than 0. Got {}", num_periods)
        }
        num_periods as f64 * ((self.rate + 1.0).powf(1.0 / num_periods as f64) - 1.0)
    }
}
