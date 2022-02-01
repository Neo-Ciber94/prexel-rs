use bigdecimal::BigDecimal;
use math_engine::num::unchecked::UncheckedNum;
use math_engine::context::DefaultContext;
use math_engine::evaluator::Evaluator;

fn main(){
    compute::<BigDecimal>("100!");
    compute::<BigDecimal>("sqrt(0.7)");
    compute::<BigDecimal>("sin(180)");
    compute::<BigDecimal>("cos(180)");
}

fn compute<T>(expr: &str) where T: 'static + UncheckedNum {
    let context = DefaultContext::<T>::new_unchecked();
    let evaluator = Evaluator::with_context(context);
    match evaluator.eval(expr) {
        Ok(n) => println!("{} = {}", expr, n),
        Err(e) => println!("{} = {:?}", expr, e),
    }
}