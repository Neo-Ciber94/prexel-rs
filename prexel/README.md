# Prexel
An evaluator for math expressions.

## Usage
```toml
[dependencies]
prexel = "0.1.0"
```

## Examples
You can use `Evaluator<N>` for evaluate an expression.

```rust
fn main(){
    let evaluator: Evaluator<f64> = Evaluator::new();
    println!("{:?}", evaluator.eval("2 + 3 * 5"));
}
```

## Warning
This library is not stable and could have breaking changes in any time.

## Implementation
There are 3 steps for evaluating each expression:
- *Tokenization*: A string is converted into an array of tokens.

- *Conversion*: The array of tokens is converted from infix to postfix notation using the [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm).

- *Evaluation*: The resulting [RPN (Reverse Polish Notation)](https://en.wikipedia.org/wiki/Reverse_Polish_notation)
expression is evaluated.

This is done using the `Tokenizer`, `Evaluator` and `Context`. The `Tokenizer` converts an `str` to `Token`s
and the `Evaluator` process and evaluates the tokens.

The `Context` is where all the functions, variables, constants and additional information used for evaluation
is stored. You can use the implementation provided by `DefaultContext`.

## Precision
Some math functions implemented in `prexel::ops::math` like trigonometric functions
use internally `f64` for the calculations using the traits `FromPrimitive` and `ToPrimitive`
what may lead to precision errors.

If you need higher precision make use of the `decimal` feature to enable a
128 bits decimal number:

```rust
use prexel::context::DefaultContext;
use prexel::evaluator::Evaluator;

fn main(){
    let context = DefaultContext::new_decimal();
    let evaluator = Evaluator::with_context(context);

    println!("{:?}", evaluator.eval("Cos(180) * 10!"));
}
```