use crate::rates::Rate;
use crate::coin::value::Value;
use std::collections::HashSet;
use crate::coin::currency::{Currency, CurrCode, curr_parity};

/// Cash Flow
/// Functions for calculation cash flow-related financial and accounting analysis

/// Returns the difference between the present value of cash inflows and the present value of cash
/// outflows over a period of time in the form of a f64. A positive NPV indicates that the projected
/// generated (in present dollars) exceeds the anticipated costs (in present dollars).
///
/// Function takes a Rate (instantiated by rates::Rate). Should use the periodic_rate method of Rate
/// to obtain the rate over the period of time/number of payments.
///
pub fn net_present_value(rate: Rate, values: &Vec<Value>) -> Value {
    // Check for currency code parity before performing calc
    let mut curr_code = curr_parity(&values);
    Value::new( values.iter()
        .enumerate()
        .map(|(ind, val)| val.amount / (1.0 + rate.rate)
            .powf(1.0 + ind as f64))
        .sum(), curr_code)
}

pub fn discounted_net_present_value(rate: Rate, values: &Vec<Value>) -> Value {
    // Check for currency code parity before performing calc
    let mut curr_code = curr_parity(&values);
    Value::new(values.iter()
        .enumerate()
        .map(|(ind, val)| val.amount * -(ind as f64) * (1.0 + rate.rate)
            .powf(ind as f64 - 1.0) / (1.0 + rate.rate)
            .powf((2.0 * ind as f64)))
        .sum(), curr_code)
}

//pub fn internal_rate_of_return(values: Vec<f64>, rate_est: Rate) {
//
//}

// DEP CODE ------------------------------------------------------------------------------------- \\
