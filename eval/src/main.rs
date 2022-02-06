use std::env::args;
use bigdecimal::BigDecimal;
use prexel::context::DefaultContext;
use prexel::evaluator::Evaluator;

fn main() -> prexel::Result<()> {
    let args = args().skip(1).collect::<Vec<_>>();
    let expr = args.join(" ");
    let result = eval(&expr)?;
    println!("{}", result);
    Ok(())
}

fn eval(expr: &str) -> prexel::Result<BigDecimal> {
    let context = DefaultContext::new_unchecked();
    let evaluator = Evaluator::with_context(context);
    evaluator.eval(expr)
}
