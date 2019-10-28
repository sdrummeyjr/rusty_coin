mod finance_mod;

use finance_mod::rates;
use finance_mod::cashflow;
use finance_mod::value;


#[test]
fn test_npv() {
    let rate = rates::Rate{rate: 5.0};
    let cash_flow = vec![2.0, 3.0, 5.0, 6.0];
    assert_eq!(cashflow::net_present_value(rate, cash_flow),  0.4444444444444444)
}

#[test]
fn test_per_rate() {
    let rate = rates::Rate{rate: 5.0};
    let periods = 12;

    assert_eq!(rate.periodic_rate(periods).rate, 0.16103667237399422)
}

#[test]
fn test_values() {
    let val_amount = 5.0;
    let val_currency = value::Currency::USD;

    let new_value = value::Value{currency: val_currency, amount: val_amount};
    println!("{}", new_value);

    assert_eq!(new_value.currency.symbol(), "$")
}
