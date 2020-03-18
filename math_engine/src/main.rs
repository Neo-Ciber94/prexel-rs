use math_engine::evaluator::Evaluator;

fn main(){
    let evaluator : Evaluator<i32> = Evaluator::new();
    println!("{:?}", evaluator.eval("Random(0, 100)"));
}