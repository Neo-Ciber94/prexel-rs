use math_engine::evaluator::Evaluator;
use math_engine::decimal::*;
use math_engine::context::DefaultContext;
use rust_decimal::Decimal;
use rust_decimal_macros::*;

fn main(){
    // let context = DefaultContext::new_decimal();
    // let evaluator = Evaluator::with_context(context);
    // println!("{:?}", evaluator.eval("100!"));
    //
    // println!("{}", 0f64.atan2(0f64));
    let decimal = dec!(4.9999999999999999999999991986);
    println!("{}", decimal.round());
}