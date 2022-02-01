use math_engine::context::{Context, DefaultContext};
use math_engine::evaluator::Evaluator;
use math_engine::function::{Associativity, BinaryFunction, Precedence};
use math_engine::Result;

fn main() {
    const USE_U8: bool = true;

    if USE_U8 {
        let mut context: DefaultContext<u8> = DefaultContext::new();
        context.add_constant("true", 1);
        context.add_constant("false", 0);
        context.add_binary_function(AndOperator);
        context.add_binary_function(OrOperator);
        context.add_binary_function(XorOperator);

        let evaluator = Evaluator::with_context(context);

        let expression1 = "(1 or 0) and (1 xor 0)";
        let expression2 = "(0 and 1)";

        assert_eq!(evaluator.eval(expression1).unwrap(), 1);
        assert_eq!(evaluator.eval(expression2).unwrap(), 0);

        // Prints the result
        println!("{} = {}", expression1, evaluator.eval(expression1).unwrap());
        println!("{} = {}", expression2, evaluator.eval(expression2).unwrap());
    } else {
        let mut context: DefaultContext<bool> = DefaultContext::new();
        context.add_constant("true", true);
        context.add_constant("false", false);
        context.add_binary_function(AndOperator);
        context.add_binary_function(OrOperator);
        context.add_binary_function(XorOperator);

        let evaluator = Evaluator::with_context(context);

        let expression1 = "(true or false) and (true xor false)";
        let expression2 = "(false and true)";

        assert_eq!(evaluator.eval(expression1).unwrap(), true);
        assert_eq!(evaluator.eval(expression2).unwrap(), false);

        // Prints the result
        println!("{} = {}", expression1, evaluator.eval(expression1).unwrap());
        println!("{} = {}", expression2, evaluator.eval(expression2).unwrap());
    }
}

struct AndOperator;
struct XorOperator;
struct OrOperator;

//////////////// Implementation for bool ////////////////
impl BinaryFunction<bool> for AndOperator {
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

impl BinaryFunction<bool> for XorOperator {
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

impl BinaryFunction<bool> for OrOperator {
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

//////////////// Implementation for u8 ////////////////
impl BinaryFunction<u8> for AndOperator {
    fn name(&self) -> &str {
        "and"
    }

    fn precedence(&self) -> Precedence {
        Precedence::VERY_LOW
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: u8, right: u8) -> Result<u8> {
        Ok(left & right)
    }
}

impl BinaryFunction<u8> for XorOperator {
    fn name(&self) -> &str {
        "xor"
    }

    fn precedence(&self) -> Precedence {
        Precedence::LOW
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: u8, right: u8) -> Result<u8> {
        Ok(left ^ right)
    }
}

impl BinaryFunction<u8> for OrOperator {
    fn name(&self) -> &str {
        "or"
    }

    fn precedence(&self) -> Precedence {
        Precedence::MEDIUM
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: u8, right: u8) -> Result<u8> {
        Ok(left | right)
    }
}
