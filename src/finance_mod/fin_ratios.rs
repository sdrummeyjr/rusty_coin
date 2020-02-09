use crate::rates::Rate;
use crate::value::Value;

/// Financial Ratios are used to help summarize financial statements and the health of a
/// company/project/enterprise

// https://www.investopedia.com/financial-ratios-4689817

/// Return on Investment - ROI
/// performance measure used to evaluate the efficiency of an investment or compare the efficiency
/// of a number of different investments. Measure the amount of return on a particular investment,
///relative to the investment’s cost.
pub fn return_on_investment(value_of_inv: &Value, cost_of_inv: &Value) -> Rate {
    Rate::new((value_of_inv.amount - cost_of_inv.amount) / cost_of_inv.amount)
}

/// Required Rate of Return – RRR
/// the minimum amount of profit (return) an investor will receive for assuming the risk of
/// investing in a stock or another type of security. RRR also can be used to calculate how
/// profitable a project might be relative to the cost of funding the project. RRR signals the level
/// of risk that's involved in committing to a given investment or project. The greater the return,
/// the greater the level of risk
pub fn req_rate_of_return() {

}


/// Cost of Equity
/// The return a company requires to decide if an investment meets capital return requirements
pub fn cost_of_equity() {

}

