# MathEngine
An evaluator for math expressions.

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

## Precision
Some of the math functions implemented in `math_engine::ops::math` like trigonometric functions
use internally `f64` for the calculations using the traits `FromPrimitive` and `ToPrimitive`
what may lead to precision errors.

If you need higher precision make use of the `decimal` feature to enable a
128 bits decimal number:

```rust
use math_engine::context::DefaultContext;
use math_engine::evaluator::Evaluator;

fn main(){
    let context = DefaultContext::new_decimal();
    let evaluator = Evaluator::with_context(context);

    println!("{:?}", evaluator.eval("Cos(180) * 10!"));
}
```

## Future
This library is waiting for an stable version of the [Specialization RFC](https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md)
which enable a more generic use `DefaultContext::new()`, currently the library
have:

- `DefaultContext::new_unchedked()`
- `DefaultContext::new_checked()`
- `DefaultContext::new_decimal()`
- `DefaultContext::new_complex()`

But with the specialization feature all can be delegate to a single `new` allowing
to provide better support for other types.