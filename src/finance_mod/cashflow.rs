use crate::rates::Rate;

//! Cash Flow
/// Functions for calculation cashflow-related financial and accounting analysis


/// Returns the difference between the present value of cash inflows and the present value of cash
/// outflows over a period of time in the form of a f64. A positive NPV indicates that the projected
/// generated (in present dollars) exceeds the anticipated costs (in present dollars).
///
/// Function takes a Rate (instantiated by rates::Rate). Should use the periodic_rate method of Rate
/// to obtain the rate over the period of time/number of payments.
///
pub fn net_present_value(rate: Rate, values: Vec<f64>) -> f64 {
    let npv: f64 = values.iter()
        .enumerate()
        .map(|(ind, val)| val / (1.0 + rate.rate)
            .powf(1.0 + ind as f64))
        .sum();
    npv
}


pub fn discounted_net_present_value(rate: Rate, values: Vec<f64>) -> f64 {
    let dnpv: f64 = values.iter()
        .enumerate()
        .map(|(ind, val)| val * -(ind as f64) * (1.0 + rate.rate)
            .powf(ind as f64 - 1.0) / (1.0 + rate.rate)
            .powf((2.0 * ind as f64)))
        .sum();
    dnpv
}

//pub fn internal_rate_of_return(values: Vec<f64>, rate_est: Rate) {
//
//}

// DEP CODE ------------------------------------------------------------------------------------- \\
