mod rc;

use rc::rates;
use rc::cashflow;
use rc::value;
use rc::currency;
use rc::fin_ratios;
use crate::rc::rates::{Rate, Precision};
use crate::rc::fin_ratios::{total_avg_assets, return_on_assets};
use crate::rc::value::{Value, combine_and_convert};
use crate::rc::currency::CurrCode;


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
    let vec_of_f = vec![2.0, 3.0, 5.0, 6.0];
    let mut cash_flow = Value::new_vec(vec_of_f, currency::CurrCode::USD);
//    cash_flow.push(Value::new(1.0, CurrCode::GBP));
//    println!("{:#?}", &cash_flow);
    let npv = cashflow::net_present_value(rate, &cash_flow);
//    println!("{}", &npv);
    let t_val = Value::new(0.4444444444444444, currency::CurrCode::USD);
//    assert_eq!(cashflow::net_present_value(rate, cash_flow),  t_val)
    assert_eq!(npv,  t_val)
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
fn test_fin_ratios_roi() {
    let investment = value::Value::new(100.00, currency::CurrCode::USD);
    let sell_price = value::Value::new(150.00, currency::CurrCode::USD);
    let roi = fin_ratios::return_on_investment(&sell_price, &investment);
    let new_prec = rates::Rate{rate: roi.rate, precision: rates::Precision::One};
    println!("The ROI for investing at $100 and selling at $150 = {}", new_prec);
    assert_eq!(roi.rate, 0.5)
}

#[test]
fn test_fin_ratios_coe() {
    let pay = value::Value::new(1.00, currency::CurrCode::USD);
    let share_price = value::Value::new(10.00, currency::CurrCode::USD);
    let apprec = rates::Rate::new(0.05);
    let coe = fin_ratios::cost_of_equity(&pay, &share_price, &apprec);

    println!("The COE is: {}", &coe.rate_to_string());

    assert_eq!(coe.rate_to_string(), "0.15%")
}

#[test]
fn test_change_prec() {
    let mut new_r = Rate::new(0.5);
    println!("Original Rate: {}", &new_r);
    assert_eq!(new_r.rate_to_string(), "0.50%");
    new_r.change_precision(Precision::One);
    println!("New Rate: {}", &new_r);
    assert_eq!(new_r.rate_to_string(), "0.5%")
}

#[test]
fn test_tot_avg_assets() {
    let a = value::Value::new(20.00, currency::CurrCode::USD);
    let b = value::Value::new(40.00, currency::CurrCode::GBP);
    let v = vec![&a, &b];
    let taa = total_avg_assets(v, currency::CurrCode::USD);
    println!("{}", &taa);
    assert_eq!(taa.value_to_string(), "$30.00")
}

#[test]
fn test_ret_on_assets() {
    let a = value::Value::new(20.00, currency::CurrCode::USD);
    let b = value::Value::new(40.00, currency::CurrCode::USD);
    let v = vec![&a, &b];
    let taa = total_avg_assets(v, currency::CurrCode::USD);
    let ni = value::Value::new(100.00, currency::CurrCode::USD);
    let roa = return_on_assets(&ni, &taa);
    println!("ROA = {}", &roa);
    assert_eq!(roa.rate_to_string(), "3.33%")
}

#[test]
fn test_to_str() {
    let r = Rate::new(0.05);
    let t = "0.05%";
    println!("{}", r.rate_to_str());
    assert_eq!(r.rate_to_str(), t)
}

#[test]
fn test_val_vec() {
    let v = vec![5.0, 10.0, 9 as f64];
    let vv = Value::new_vec(v, CurrCode::USD);
    println!("{:#?}", vv);
    let t_vv = vec![Value::new(5.0, CurrCode::USD),
                    Value::new(10.0, CurrCode::USD),
                    Value::new(9.0, CurrCode::USD)];
    assert_eq!(vv, t_vv)
}

#[test]
fn test_conversions() {
    // test exchange_rate and convert methods and the convert_vec func through the
    // combine_and_convert function
    let v_one = Value::new(10.00, CurrCode::USD);
    let v_two = Value::new(12.00, CurrCode::GBP);
    let er = Rate::exchange_rate(v_one, v_two);
    println!("Exchange Rate: {}", &er);

    let us_vec = Value::new_vec(vec![5.00, 10.00, 20.00], CurrCode::USD);
    let gb_vec = Value::new_vec(vec![10.00, 20.00, 6.00], CurrCode::GBP);
    let cc = combine_and_convert(&gb_vec, &us_vec, er);
    for c in cc {
        println!("{}", c)
    }
}
