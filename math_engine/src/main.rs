use math_engine::function::Function;
use math_engine::error::{Error, ErrorKind};
use math_engine::context::{DefaultContext, Context};
use math_engine::evaluator::Evaluator;
use std::fmt::Debug;
use math_engine::tokenizer::{Tokenizer, Tokenize};

fn main(){
    let mut c: DefaultContext<f64> = DefaultContext::new_checked();
    c.add_function(FirstFunction);

    let t = Tokenizer::with_context(&c);
    let tokens = t.tokenize("Max(1, 2, 3)").unwrap();
    println!("{:?}", math_engine::evaluator::infix_to_rpn(&tokens, &c));

    // let e = Evaluator::with_context(c);
    // println!("{:?}", e.eval("First(1, 2, 3, 4)"))
}

pub struct FirstFunction;
impl<N: Clone + Debug> Function<N> for FirstFunction{
    fn name(&self) -> &str {
        "first"
    }

    fn call(&self, args: &[N]) -> math_engine::error::Result<N> {
        println!("Args: {:?}", args);
        if args.len() == 0{
            Err(Error::from(ErrorKind::InvalidArgumentCount))
        }
        else{
            Ok(args[0].clone())
        }
    }
}