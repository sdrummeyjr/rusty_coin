use crate::rates::Rate;
use crate::value::Value;
use crate::currency::{CurrCode};

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
//pub fn req_rate_of_return() {
//
//}


/// Cost of Equity
/// The return a company requires to decide if an investment meets capital return requirements
pub fn cost_of_equity(payout: &Value, share_price: &Value, rate_of_appr: &Rate) -> Rate {
    Rate::new((payout.amount / share_price.amount) + rate_of_appr.rate)
}


/// Return on Assets
/// A profitability ratio that provides how much profit a company is able to generate from its
/// assets. In other words, return on assets (ROA) measures how efficient a company's management
/// is in generating earnings from their economic resources or assets on their balance sheet. ROA
/// is shown as a percentage, and the higher the number, the more efficient a company's management
/// is at managing its balance sheet to generate profits.
pub fn return_on_assets(net_income: &Value, average_total_assets: &Value) -> Rate {
    Rate::new(net_income.amount / average_total_assets.amount)
}


/// Total Average Assets
/// Calculate the total average assets over a period of time
pub fn total_avg_assets(total_assets: Vec<&Value>, currency_code: CurrCode) -> Value {
    // todo - set the currency_code based on the currency code of the list of values
    Value::new(total_assets.iter().map(|v| v.amount ).sum::<f64>() /
                   total_assets.len() as f64, currency_code)
}
