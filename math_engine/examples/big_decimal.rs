use bigdecimal::BigDecimal;
use math_engine::num::unchecked::UncheckedNum;
use std::panic::{RefUnwindSafe, UnwindSafe};

fn main(){
    compute::<BigDecimal>("100!");
    compute::<BigDecimal>("sqrt(0.7)");
    compute::<BigDecimal>("sin(180)");
    compute::<BigDecimal>("cos(180)");
}

fn compute<T>(expr: &str) where T: 'static
    + UncheckedNum
    + RefUnwindSafe
    + UnwindSafe {
    match math_engine::eval::<T>(expr){
        Ok(n) => println!("{} = {}", expr, n),
        Err(e) => println!("{} = {:?}", expr, e),
    }
}