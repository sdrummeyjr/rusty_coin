use crate::rates::Rate;

// Holds functions for calculation cashflow-related financial and accounting analysis


// https://www.reddit.com/r/rust/comments/2ghy90/for_loops_any_nice_way_to_get_number_of_iterations/
pub fn net_present_value(rate: Rate, values: Vec<f64>) -> f64 {
    let npv: f64 = values.iter()
        .enumerate()
        .map(|(ind, val)| val / (1.0 + rate.rate)
            .powf(1.0 + ind as f64))
        .sum();
    npv
}


pub fn d_net_present_value(rate: Rate, values: Vec<f64>) -> f64 {
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

// let power = num::checked_pow(ind);
// println!("{}", ind as f64);

//let periodic_rate = ((1.0 + rate as f64).powf(1.0 / values.len() as f64) - 1.0);
//    println!("{} \n", periodic_rate);

//    let new_npv = values.iter().enumerate()
//        .fold(0.0, |acc, (ind, val)|
//            val / (1.0 + r.periodic_rate(values.len())).powf(ind as f64));

//let mut npv = 0.0;

//    for (ind, val) in values.iter().enumerate() {
//
//        npv += val / (1.0 + r.periodic_rate(values.len())).powf(ind as f64);
//        println!("{}. {}", ind, npv);
//    }
//    (npv, new_npv)

//    let mut npv = 0.0;
//
//    for (ind, val) in values.iter().enumerate() {
//        let calc = val / (1.0 + r.rate).powf(1.0 + ind as f64);
//        println!("{}", &calc);
//        npv += calc;
//        println!("{}. {}", ind, val);
//}
//println!("{}", new_npv)
//    new_npv
//    let r = rates::Rate{rate: rate};
//    let pr = rates::Rate::new(r.periodic_rate(values.len()));

