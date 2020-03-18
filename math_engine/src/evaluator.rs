use crate::context::{Context, DefaultContext};
use crate::error::{Error, ErrorKind, Result};
use crate::token::Token;
use crate::token::Token::*;
use crate::tokenizer::{Tokenize, Tokenizer};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::str::FromStr;
use crate::num::checked::CheckedNum;

/// A trait for evaluate an expression of `Token`.
pub trait Evaluate<N> {
    /// Evaluates the expression provided as `Token`s.
    fn eval_tokens(&self, tokens: &[Token<N>]) -> Result<N>;
}

/// Represents the default `MathEvaluator`.
pub struct Evaluator<'a, N, C: Context<'a, N> = DefaultContext<'a, N>> {
    context: C,
    _marker: &'a PhantomData<N>,
}

impl<'a, N: CheckedNum> Evaluator<'a, N, DefaultContext<'a, N>> {
    #[inline]
    pub fn new() -> Self {
        Evaluator {
            context: DefaultContext::new_checked(),
            _marker: &PhantomData,
        }
    }
}

impl<'a, N, C> Evaluator<'a, N, C> where C: Context<'a, N> {
    #[inline]
    pub fn with_context(context: C) -> Self {
        Evaluator {
            context,
            _marker: &PhantomData,
        }
    }

    #[inline]
    pub fn context(&self) -> &C {
        &self.context
    }

    #[inline]
    pub fn mut_context(&mut self) -> &mut C {
        &mut self.context
    }
}

impl<'a, N, C> Evaluator<'a, N, C> where C: Context<'a, N>, N: FromStr + Debug + Clone {
    #[inline]
    pub fn eval(&'a self, expression: &str) -> Result<N> {
        let context = self.context();
        let tokenizer = Tokenizer::with_context(context);
        let tokens = Tokenize::tokenize(&tokenizer, expression)?;
        eval_tokens_raw(&tokens, context)
    }
}

impl<'a, C, N> Evaluate<N> for Evaluator<'a, N, C> where C: Context<'a, N>, N: Debug + Clone {
    #[inline]
    fn eval_tokens(&self, tokens: &[Token<N>]) -> Result<N> {
        eval_tokens_raw(tokens, self.context())
    }
}

fn eval_tokens_raw<'a, N, C>(tokens: &[Token<N>], context: &C) -> Result<N>
where
    N: Debug + Clone,
    C: Context<'a, N>,
{
    let rpn = helper::infix_to_rpn(tokens, context)?;
    let mut values: Vec<N> = Vec::new();
    let mut arg_count: Option<u32> = None;

    for token in &rpn {
        match token {
            Number(n) => values.push(n.clone()),
            Variable(name) => {
                let n = context
                    .get_variable(name.as_str())
                    .ok_or(Error::new(
                        ErrorKind::InvalidInput,
                        format!("Variable `{}` not found", name),
                    ))?
                    .clone();

                values.push(n);
            }
            Constant(name) => {
                let n = context
                    .get_constant(name.as_str())
                    .ok_or(Error::new(
                        ErrorKind::InvalidInput,
                        format!("Constant `{}` not found", name),
                    ))?
                    .clone();

                values.push(n);
            }
            ArgCount(n) => {
                assert_eq!(arg_count, None);
                arg_count = Some(*n);
            }
            UnaryOperator(c) => {
                let mut buf = [0u8; 4];
                let func =
                    context
                        .get_unary_function(c.encode_utf8(&mut buf))
                        .ok_or(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Unary operator `{}` not found", c),
                        ))?;

                match values.pop() {
                    Some(value) => {
                        let result = func.call(value)?;
                        values.push(result);
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidExpression,
                            format!("{:?}", &tokens),
                        ));
                    }
                }
            }
            InfixFunction(name) => {
                let func = context.get_binary_function(name).ok_or(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Infix function `{}` not found", name),
                ))?;

                match (values.pop(), values.pop()) {
                    (Some(right), Some(left)) => {
                        let result = func.call(left, right)?;
                        values.push(result);
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidExpression,
                            format!("{:?}", &tokens),
                        ));
                    }
                }
            }
            BinaryOperator(c) => {
                let mut buf = [0u8; 4];
                let func =
                    context
                        .get_binary_function(c.encode_utf8(&mut buf))
                        .ok_or(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Binary operator `{}` not found", c),
                        ))?;

                match (values.pop(), values.pop()) {
                    (Some(right), Some(left)) => {
                        let result = func.call(left, right)?;
                        values.push(result);
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidExpression,
                            format!("{:?}", &tokens),
                        ));
                    }
                }
            }
            Function(name) => {
                let func = context.get_function(name).ok_or(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Function `{}` not found", name),
                ))?;

                let n = arg_count.ok_or(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "Cannot evaluate function `{}`, unknown number of arguments",
                        name
                    ),
                ))?;

                let mut args = Vec::new();
                for _ in 0..n {
                    match values.pop() {
                        Some(n) => args.push(n.clone()),
                        None => {
                            Error::new(
                                ErrorKind::InvalidArgumentCount,
                                format!("Expected {} arguments but {} was get", n, args.len()),
                            );
                        }
                    }
                }

                args.reverse();
                let result = func.call(&args)?;
                values.push(result.clone());

                arg_count = None;
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Unknown token: `{:?}`", token),
                ));
            }
        }
    }

    if values.len() == 1 {
        Ok(values[0].clone())
    } else {
        Err(Error::from(ErrorKind::InvalidExpression))
    }
}

pub mod helper {
    use crate::context::Context;
    use crate::error::{Error, ErrorKind, Result};
    use crate::function::{Associativity, Notation};
    use crate::token::Token;
    use crate::token::Token::BinaryOperator;
    use std::fmt::Debug;

    /// Converts an `infix` notation expression to `rpn` (Reverse Polish Notation) using
    /// the shunting yard algorithm.
    ///
    /// See: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
    pub fn infix_to_rpn<'a, N: Clone + Debug>(
        tokens: &[Token<N>],
        context: &impl Context<'a, N>,
    ) -> Result<Vec<Token<N>>> {
        let mut output = Vec::new();
        let mut operators = Vec::new();
        let mut arg_count: Vec<u32> = Vec::new();

        let mut token_iterator = tokens.iter().peekable();
        while let Some(token) = token_iterator.next() {
            match token {
                Token::Number(_) | Token::Variable(_) | Token::Constant(_) => {
                    push_number(context, &mut output, &mut operators, token)
                }
                Token::BinaryOperator(c) => {
                    let mut buf = [0u8; 4];
                    push_binary_function(
                        context,
                        &mut output,
                        &mut operators,
                        token,
                        c.encode_utf8(&mut buf),
                    );
                }
                Token::InfixFunction(name) => {
                    push_binary_function(context, &mut output, &mut operators, token, name)
                }
                Token::UnaryOperator(c) => {
                    push_unary_function(context, &mut output, &mut operators, token, c)?
                }
                Token::Function(_) => {
                    arg_count.push(0);
                    operators.push(token.clone());
                }
                Token::GroupingOpen(_) => operators.push(token.clone()),
                Token::GroupingClose(c) => {
                    push_grouping_close(context, *c, &mut output, &mut operators, &mut arg_count)?
                }
                Token::Comma => push_comma(&mut output, &mut operators, &mut arg_count)?,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("Invalid token: {:?}", token),
                    ))
                }
            }

            //println!("Current: {:?}\nOut: {:?}\nOp: {:?}\nArgCount: {:?}\n\n", token, output, operators, arg_count);

            // If implicit multiplication
            if context.config().implicit_mul() {
                if token.is_number() {
                    // 2Max, 2PI, 2x, 2(4)
                    if let Some(next_token) = token_iterator.peek() {
                        match *next_token {
                            Token::Function(_)
                            | Token::Constant(_)
                            | Token::Variable(_)
                            | Token::GroupingOpen(_) => {
                                operators.push(BinaryOperator('*'));
                            }
                            _ => {}
                        }
                    }
                } else if token.is_grouping_close() {
                    //(2)2, (2)PI, (2)x, (4)(2)
                    if let Some(next_token) = token_iterator.peek() {
                        if next_token.is_grouping_open() {
                            operators.push(BinaryOperator('*'));
                        }
                    }
                }
            }
        }

        while let Some(t) = operators.pop() {
            if t.is_grouping_close() || t.is_grouping_close() {
                return Err(Error::new(
                    ErrorKind::InvalidExpression,
                    "Misplace parentheses",
                ));
            }

            output.push(t)
        }

        //println!("Current: -\nOut: {:?}\nOp: {:?}\nArgCount: {:?}\n\n", output, operators, arg_count);

        Ok(output)
    }

    fn push_number<'a, N: Clone + Debug>(
        context: &impl Context<'a, N>,
        output: &mut Vec<Token<N>>,
        operators: &mut Vec<Token<N>>,
        token: &Token<N>,
    ) -> () {
        output.push(token.clone());
        match operators.last() {
            Some(t) => match t {
                Token::UnaryOperator(c) => {
                    let mut buf = [0u8; 4];
                    let name = c.encode_utf8(&mut buf);

                    if let Some(_) = context.get_unary_function(name) {
                        output.push(operators.pop().unwrap());
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn push_unary_function<'a, N: Clone + Debug>(
        context: &impl Context<'a, N>,
        output: &mut Vec<Token<N>>,
        operators: &mut Vec<Token<N>>,
        token: &Token<N>,
        c: &char,
    ) -> Result<()> {
        let mut buf = [0u8; 4];
        let name = c.encode_utf8(&mut buf);

        if let Some(unary) = context.get_unary_function(name) {
            match unary.notation() {
                Notation::Prefix => {
                    //+6
                    operators.push(token.clone());
                }
                Notation::Postfix => {
                    // 5!
                    if output.len() > 0 {
                        output.push(token.clone())
                    } else {
                        return Err(Error::new(
                            ErrorKind::InvalidExpression,
                            "Misplace unary operator",
                        ));
                    }
                }
            }

            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unary operator `{}` not found", c),
            ))
        }
    }

    fn push_binary_function<'a, N: Clone + Debug>(
        context: &impl Context<'a, N>,
        output: &mut Vec<Token<N>>,
        operators: &mut Vec<Token<N>>,
        token: &Token<N>,
        name: &str,
    ) -> () {
        let cur_operator = context.get_binary_function(name).unwrap();

        while let Some(t) = operators.last() {
            match t {
                Token::GroupingOpen(_) => {
                    break;
                }
                _ => {}
            }

            if t.is_function() {
                output.push(operators.pop().unwrap());
            } else {
                let top_operator = match t {
                    Token::BinaryOperator(c) => {
                        let mut buf = [0u8; 4];
                        let name = c.encode_utf8(&mut buf);
                        context.get_binary_function(name)
                    }
                    Token::InfixFunction(n) => context.get_binary_function(n),
                    _ => {
                        break;
                    }
                };

                match top_operator {
                    Some(top) => {
                        if (top.precedence() > cur_operator.precedence())
                            || (top.precedence() == cur_operator.precedence()
                                && top.associativity() == Associativity::Left)
                        {
                            output.push(operators.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }
        }

        operators.push(token.clone());
    }

    fn push_grouping_close<'a, N: Clone + Debug>(
        context: &impl Context<'a, N>,
        group_close: char,
        output: &mut Vec<Token<N>>,
        operators: &mut Vec<Token<N>>,
        arg_count: &mut Vec<u32>,
    ) -> Result<()> {
        let mut is_group_open = false;
        while let Some(t) = operators.pop() {
            match t {
                Token::GroupingOpen(c) => {
                    if let Some(grouping) = context.config().get_group_symbol(c) {
                        if grouping.group_close == group_close {
                            is_group_open = true;
                            if arg_count.len() > 0 {
                                if let Some(top) = operators.last() {
                                    match top {
                                        Token::Function(_) => {
                                            let count = arg_count.pop().unwrap() + 1;
                                            output.push(Token::ArgCount(count));
                                            output.push(operators.pop().unwrap());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }

                    break;
                }
                _ => output.push(t.clone()),
            }
        }

        if !is_group_open {
            Err(Error::new(
                ErrorKind::InvalidExpression,
                "Misplace grouping symbol",
            ))
        } else {
            Ok(())
        }
    }

    fn push_comma<N: Clone + Debug>(
        output: &mut Vec<Token<N>>,
        operators: &mut Vec<Token<N>>,
        arg_count: &mut Vec<u32>,
    ) -> Result<()> {
        //*arg_count.last_mut().unwrap() += 1;
        match arg_count.last_mut() {
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidExpression,
                    "Comma found but not function",
                ))
            }
            Some(n) => *n += 1,
        }

        let mut is_group_open = false;
        while let Some(t) = operators.last() {
            match t {
                Token::GroupingOpen(_) => {
                    is_group_open = true;
                    break;
                }
                _ => {
                    output.push(operators.pop().unwrap());
                }
            }
        }

        if !is_group_open {
            Err(Error::new(ErrorKind::InvalidExpression, "Misplace comma"))
        } else {
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::context::{Config, DefaultContext};
        use crate::token::Token::*;

        #[test]
        fn unary_ops_test1() {
            let context = &DefaultContext::new_checked();

            assert_eq!(
                infix_to_rpn(
                    // -(+10) -> 10 + -
                    &[
                        UnaryOperator('-'),
                        GroupingOpen('('),
                        UnaryOperator('+'),
                        Number(10),
                        GroupingClose(')')
                    ],
                    context
                )
                .unwrap(),
                [Number(10), UnaryOperator('+'), UnaryOperator('-')]
            );
        }

        #[test]
        fn binary_ops_test1() {
            let context = &DefaultContext::new_checked();

            assert_eq!(
                infix_to_rpn(
                    // 3 + 2 -> 3 2 +
                    &[Number(3), BinaryOperator('+'), Number(2)],
                    context
                )
                .unwrap(),
                [Number(3), Number(2), BinaryOperator('+')]
            );
        }

        #[test]
        fn binary_ops_test2() {
            let context = &DefaultContext::new_checked();

            assert_eq!(
                infix_to_rpn(
                    // 2 + 3 * 5 -> 2 3 5 + *
                    &[
                        Number(2),
                        BinaryOperator('+'),
                        Number(3),
                        BinaryOperator('*'),
                        Number(5)
                    ],
                    context
                )
                .unwrap(),
                [
                    Number(2),
                    Number(3),
                    Number(5),
                    BinaryOperator('*'),
                    BinaryOperator('+')
                ]
            );
        }

        #[test]
        fn binary_ops_test3() {
            let context = &DefaultContext::new_checked();

            assert_eq!(
                infix_to_rpn(
                    // 2 ^ 3 ^ 4 - 1
                    &[
                        Number(2),
                        BinaryOperator('^'),
                        Number(3),
                        BinaryOperator('^'),
                        Number(4),
                        BinaryOperator('-'),
                        Number(1)
                    ],
                    context
                )
                .unwrap(),
                [
                    Number(2),
                    Number(3),
                    Number(4),
                    BinaryOperator('^'),
                    BinaryOperator('^'),
                    Number(1),
                    BinaryOperator('-')
                ]
            );
        }

        #[test]
        fn binary_ops_test4() {
            let context = &DefaultContext::new_checked();

            assert_eq!(
                infix_to_rpn(
                    // (5 + (-3)) ^ Max(1, 2*5, (30/2))
                    &[
                        GroupingOpen('('),
                        Number(5),
                        BinaryOperator('+'),
                        GroupingOpen('('),
                        UnaryOperator('-'),
                        Number(3),
                        GroupingClose(')'),
                        GroupingClose(')'),
                        BinaryOperator('^'),
                        Function("Max".to_string()),
                        GroupingOpen('('),
                        Number(1),
                        Comma,
                        Number(2),
                        BinaryOperator('*'),
                        Number(5),
                        Comma,
                        GroupingOpen('('),
                        Number(30),
                        BinaryOperator('/'),
                        Number(2),
                        GroupingClose(')'),
                        GroupingClose(')'),
                    ],
                    context
                )
                .unwrap(),
                [
                    Number(5),
                    Number(3),
                    UnaryOperator('-'),
                    BinaryOperator('+'),
                    Number(1),
                    Number(2),
                    Number(5),
                    BinaryOperator('*'),
                    Number(30),
                    Number(2),
                    BinaryOperator('/'),
                    ArgCount(3),
                    Function("Max".to_string()),
                    BinaryOperator('^')
                ]
            );
        }

        #[test]
        fn infix_ops_test() {
            let context = &DefaultContext::new_checked();

            assert_eq!(
                infix_to_rpn(
                    // 10 mod 2 -> 10 2 mod
                    &[Number(10), InfixFunction(String::from("mod")), Number(2)],
                    context
                )
                .unwrap(),
                [Number(10), Number(2), InfixFunction(String::from("mod"))]
            );
        }

        #[test]
        fn function_test() {
            let context = &DefaultContext::new_checked();

            assert_eq!(
                infix_to_rpn(
                    // 5 * Sum(2, 3) -> 2 3 2arg Sum 5 *
                    &[
                        Number(5),
                        BinaryOperator('*'),
                        Function(String::from("Sum")),
                        GroupingOpen('('),
                        Number(2),
                        Comma,
                        Number(3),
                        GroupingClose(')'),
                    ],
                    context
                )
                .unwrap(),
                [
                    Number(5),
                    Number(2),
                    Number(3),
                    ArgCount(2),
                    Function(String::from("Sum")),
                    BinaryOperator('*'),
                ]
            );
        }

        #[test]
        fn implicit_mul_test1() {
            let config = Config::new().with_implicit_mul();
            let context = DefaultContext::new_checked_with_config(config);

            let infix = &[Token::Number(10), Token::Constant("PI".to_string())];
            let rpn = infix_to_rpn(infix, &context).unwrap();
            assert_eq!(
                rpn,
                &[
                    Token::Number(10),
                    Token::Constant("PI".to_string()),
                    Token::BinaryOperator('*')
                ]
            );
        }

        #[test]
        fn implicit_mul_test2() {
            let config = Config::new().with_implicit_mul();
            let context = DefaultContext::new_checked_with_config(config);

            let infix = &[
                Token::GroupingOpen('('),
                Token::Number(2),
                Token::GroupingClose(')'),
                Token::GroupingOpen('('),
                Token::Number(3),
                Token::GroupingClose(')'),
            ];

            let rpn = infix_to_rpn(infix, &context).unwrap();
            assert_eq!(
                rpn,
                &[
                    Token::Number(2),
                    Token::Number(3),
                    Token::BinaryOperator('*')
                ]
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Config;

    #[test]
    fn evaluate_test() {
        let config = Config::new().with_group_symbol('[', ']');
        let evaluator: Evaluator<i64> =
            Evaluator::with_context(DefaultContext::new_checked_with_config(config));

        assert_eq!(evaluator.eval("(2 ^ 3) ^ 4").unwrap(), 4096);
        assert_eq!(evaluator.eval("Min(10, 2) + Max(10, 2)").unwrap(), 12);
        assert_eq!(
            evaluator
                .eval("Sum(1, 2, 3) * 2 - Max(2, 10/2, 2^3)")
                .unwrap(),
            4
        );

        assert!(evaluator.eval("5").is_ok());
        assert!(evaluator.eval("-2").is_ok());
        assert!(evaluator.eval("(10)").is_ok());
        assert!(evaluator.eval("-(+(6))").is_ok());
        assert!(evaluator.eval("+10").is_ok());
        assert!(evaluator.eval("((10)+(((2)))*(3))").is_ok());
        assert!(evaluator.eval("-(2)^(((4)))").is_ok());
        assert!(evaluator.eval("-(+(-(+(5))))").is_ok());
        assert!(evaluator.eval("10--+2").is_ok());
        assert!(evaluator.eval("+2!").is_ok());

        assert!(evaluator.eval("((20) + 2").is_err());
        assert!(evaluator.eval("(1,23) + 1").is_err());
        assert!(evaluator.eval("2^").is_err());
        assert!(evaluator.eval("2 3 +").is_err());
        assert!(evaluator.eval("^10!").is_err());
        assert!(evaluator.eval("8+").is_err());
        assert!(evaluator.eval("([10)]").is_err());
    }

    #[test]
    fn evaluate_implicit_mul_test() {
        let config = Config::new().with_implicit_mul();
        let context = DefaultContext::new_checked_with_config(config);
        let mut evaluator: Evaluator<i64> = Evaluator::with_context(context);

        evaluator.mut_context().set_variable("x", 10);
        assert_eq!(evaluator.eval("2x").unwrap(), 20);

        evaluator.mut_context().set_variable("x", 5);
        assert_eq!(evaluator.eval("3x").unwrap(), 15);

        assert!(evaluator.eval("2Sin(50)").is_ok());
        assert!(evaluator.eval("(2)(4)").is_ok());
        assert!(evaluator.eval("Cos(30)(2)").is_ok());

        // not allowed due looks like function call
        assert!(evaluator.eval("5x(2)").is_err());
        assert!(evaluator.eval("3 2Sin(50)").is_err());
    }

    #[test]
    fn evaluate_tokens_test() {
        let evaluator = Evaluator::new();

        // 2 + 3
        assert_eq!(
            evaluator
                .eval_tokens(&[
                    Token::Number(3),
                    Token::BinaryOperator('+'),
                    Token::Number(2)
                ])
                .unwrap(),
            5
        );

        // 2 ^ 3 ^ 2
        assert_eq!(
            evaluator
                .eval_tokens(&[
                    Token::Number(2),
                    Token::BinaryOperator('^'),
                    Token::Number(3),
                    Token::BinaryOperator('^'),
                    Token::Number(2)
                ])
                .unwrap(),
            512
        );

        // (2 ^ 3) ^ 4
        assert_eq!(
            evaluator
                .eval_tokens(&[
                    Token::GroupingOpen('('),
                    Token::Number(2),
                    Token::BinaryOperator('^'),
                    Token::Number(3),
                    Token::GroupingClose(')'),
                    Token::BinaryOperator('^'),
                    Token::Number(4)
                ])
                .unwrap(),
            4096
        );
    }

    #[test]
    fn evaluate_variable(){
        let mut evaluator = Evaluator::new();
        evaluator.mut_context().set_variable("x", 10);

        assert_eq!(evaluator.eval("x + 2").unwrap(), 12);
    }
}