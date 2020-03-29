# MathEngine
Provides an evaluator for math expressions.

## Usage
```toml
[dependencies]
math_engine = "0.0.1"
```

## Examples
You can use `math_engine::eval<T>(...)` for evaluate an expression.
```rust
fn main(){
    let result = math_engine::eval::<f64>("2 + 3 * 5");
    println!("{:?}", result);
}
```

Or use directly the `Evaluator<T>`
```rust
fn main(){
    let evaluator: Evaluator<f64> = Evaluator::new();
    println!("{:?}", evaluator.eval("2 + 3 * 5"));
}
```

## Implementation
There are 3 steps for evaluating each expression:
- *Tokenization*: A string is converted into an array of tokens.

- *Conversion*: The array of tokens is converted from infix to postfix notation using
using the [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm).

- *Evaluation*: The resulting [RPN (Reverse Polish Notation)](https://en.wikipedia.org/wiki/Reverse_Polish_notation)
expression is evaluated.

This is done using the `Tokenizer`, `Evaluator` and `Context`. The `Tokenizer` converts an `str` to `Token`s
and the `Evaluator` process and evaluates the tokens.

The `Context` is where all the functions, variables, constants and additional information used for evaluation
is stored. You can use the implementation provided by `DefaultContext`.