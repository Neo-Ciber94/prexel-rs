use math_engine::function::{BinaryFunction, Precedence, Associativity};
use math_engine::error::*;
use math_engine::context::{DefaultContext, Context};
use math_engine::evaluator::Evaluator;

struct AndOperator;
impl BinaryFunction<bool> for AndOperator{
    fn name(&self) -> &str {
        "and"
    }

    fn precedence(&self) -> Precedence {
        Precedence::VERY_LOW
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: bool, right: bool) -> Result<bool> {
        Ok(left && right)
    }
}

struct XorOperator;
impl BinaryFunction<bool> for XorOperator{
    fn name(&self) -> &str {
        "xor"
    }

    fn precedence(&self) -> Precedence {
        Precedence::LOW
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: bool, right: bool) -> Result<bool> {
        Ok(left ^ right)
    }
}

struct OrOperator;
impl BinaryFunction<bool> for OrOperator{
    fn name(&self) -> &str {
        "or"
    }

    fn precedence(&self) -> Precedence {
        Precedence::MEDIUM
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: bool, right: bool) -> Result<bool> {
        Ok(left || right)
    }
}

fn main(){
    let mut context = DefaultContext::new();
    context.add_constant("true", true);
    context.add_constant("false", false);
    context.add_binary_function(AndOperator);
    context.add_binary_function(OrOperator);
    context.add_binary_function(XorOperator);

    let evaluator = Evaluator::with_context(context);
    println!("{:?}", evaluator.eval("(true and true) xor false"));
}