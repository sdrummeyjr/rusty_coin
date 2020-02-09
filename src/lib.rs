mod finance_mod;

use finance_mod::rates;
use finance_mod::cashflow;
use finance_mod::value;
use finance_mod::currency;
use finance_mod::fin_ratios;
//use crate::finance_mod::currency::CurrCode::USD;


#[test]
fn test_currencies() {
    let cur_1 = currency::CurrCode::USD;
    println!("{:#?}", cur_1.currency());
//    assert_eq!(cur_1, cur)
    let cur_1_test = currency::Currency::new(
        "United States",
        currency::CurrType::Dollar,
        currency::CurrCode::USD,
    );
    assert_eq!(cur_1.currency(), cur_1_test);

    let cur_2 = currency::CurrCode::YER;
    println!("{:#?}", cur_2.currency());
//    assert_eq!(cur_1, cur)
    let cur_2_test = currency::Currency::new(
        "Yemen",
        currency::CurrType::Rial,
        currency::CurrCode::YER,
    );

    assert_eq!(cur_2.currency(), cur_2_test);

}

#[test]
fn test_npv() {
    let rate = rates::Rate{rate: 5.0, precision: rates::Precision::One};
    let cash_flow = vec![2.0, 3.0, 5.0, 6.0];
    println!("{}", &cashflow::net_present_value(rates::Rate{rate: 5.0, precision: rates::Precision::One}, vec![2.0, 3.0, 5.0, 6.0]));
    assert_eq!(cashflow::net_present_value(rate, cash_flow),  0.4444444444444444)
}

#[test]
fn test_per_rate() {
    let rate = rates::Rate{rate: 5.0, precision: rates::Precision::One};
    let periods = 12;
    println!("{}", &rate.periodic_rate(12));
    assert_eq!(rate.periodic_rate(periods).rate, 0.16103667237399422)
}

#[test]
fn test_values() {
    let val_amount = 5.0;
    let val_currency_code = currency::CurrCode::USD;

    let new_value = value::Value{currency_code: val_currency_code, amount: val_amount};
    println!("{}", new_value);

    assert_eq!(new_value.currency_code.currency().symbol, "$")
}


#[test]
fn test_fin_ratios() {
    let investment = value::Value::new(100.00, currency::CurrCode::USD);
    let sell_price = value::Value::new(150.00, currency::CurrCode::USD);
    let roi = fin_ratios::return_on_investment(&sell_price, &investment);
    let new_prec = rates::Rate{rate: roi.rate, precision: rates::Precision::One};
    println!("The ROI for investing at $100 and selling at $150 = {}", new_prec);
    assert_eq!(roi.rate, 0.5)
}