use bigdecimal::BigDecimal;
use math_engine::num;
use math_engine::utils::approx::Approx;

fn main(){
    compute::<BigDecimal>("sin(90)");
    compute::<BigDecimal>("cos(90)");
    compute::<BigDecimal>("sin(180)");
    compute::<BigDecimal>("cos(180)");
    
    println!();
    println!("{}", (0.1_f64 + 0.2_f64).round());
    println!("{}", (0.1_f64 + 0.2_f64).ceil());
    println!("{}", (0.1_f64 + 0.2_f64).floor());
    println!("{}", (0.1_f64 + 0.2_f64).approx());

    // println!("{}", approx(0.00123));
    // println!("{}", approx(0.000_1291));
    // println!("{}", approx(90_f64.to_radians().sin()));
    // println!("{}", approx(90_f64.to_radians().cos()));
    // println!("{}", approx(180_f64.to_radians().sin()));
    // println!("{}", approx(180_f64.to_radians().cos()));
}

fn compute<T>(expr: &str) where
    T: num::unchecked::UncheckedNum + std::panic::RefUnwindSafe + std::panic::UnwindSafe + 'static{

    println!("{} = {:?}", expr, math_engine::eval::<T>(expr));
}