mod finance_mod;

use finance_mod::rates;
use finance_mod::cashflow;


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

    assert_eq!(rate.periodic_rate(periods), 0.16103667237399422)
}
