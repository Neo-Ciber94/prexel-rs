use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::{FromPrimitive, Zero};

use crate::error::*;
use crate::function::{
    Associativity, BinaryFunction, Function, Notation, Precedence, UnaryFunction,
};
use crate::Result;

pub struct AddOperator;
impl<N: Add<N, Output = N>> BinaryFunction<N> for AddOperator {
    fn name(&self) -> &'static str {
        "+"
    }

    fn precedence(&self) -> Precedence {
        Precedence::LOW
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: N, right: N) -> Result<N> {
        Ok(left + right)
    }
}

pub struct SubOperator;
impl<N: Sub<N, Output = N>> BinaryFunction<N> for SubOperator {
    fn name(&self) -> &'static str {
        "-"
    }

    fn precedence(&self) -> Precedence {
        Precedence::LOW
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: N, right: N) -> Result<N> {
        Ok(left - right)
    }
}

pub struct MulOperator;
impl<N: Mul<N, Output = N>> BinaryFunction<N> for MulOperator {
    fn name(&self) -> &'static str {
        "*"
    }

    fn precedence(&self) -> Precedence {
        Precedence::MEDIUM
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: N, right: N) -> Result<N> {
        Ok(left * right)
    }
}

pub struct DivOperator;
impl<N: Div<N, Output = N> + Zero> BinaryFunction<N> for DivOperator {
    fn name(&self) -> &'static str {
        "/"
    }

    fn precedence(&self) -> Precedence {
        Precedence::MEDIUM
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: N, right: N) -> Result<N> {
        if right.is_zero() {
            return Err(Error::from(ErrorKind::DivisionByZero));
        }

        Ok(left / right)
    }
}

pub struct ModOperator;
impl<N: Rem<N, Output = N> + Zero> BinaryFunction<N> for ModOperator {
    fn name(&self) -> &'static str {
        "mod"
    }

    fn precedence(&self) -> Precedence {
        Precedence::MEDIUM
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: N, right: N) -> Result<N> {
        if right.is_zero() {
            return Err(Error::from(ErrorKind::DivisionByZero));
        }

        Ok(left % right)
    }
}

pub struct UnaryMinus;
impl<N: Neg<Output = N>> UnaryFunction<N> for UnaryMinus {
    fn name(&self) -> &str {
        "-"
    }

    fn notation(&self) -> Notation {
        Notation::Prefix
    }

    fn call(&self, value: N) -> Result<N> {
        Ok(value.neg())
    }
}

pub struct SumFunction;
impl<N: Add<N, Output = N> + Clone> Function<N> for SumFunction {
    fn name(&self) -> &str {
        "sum"
    }

    fn call(&self, args: &[N]) -> Result<N> {
        let mut result = None;

        for cur in args {
            match result {
                None => result = Some(cur.clone()),
                Some(ref n) => {
                    result = Some(n.clone() + cur.clone());
                }
            }
        }

        result.ok_or(Error::from(ErrorKind::InvalidArgumentCount))
    }
}

pub struct ProdFunction;
impl<N: Mul<N, Output = N> + Clone> Function<N> for ProdFunction {
    fn name(&self) -> &str {
        "product"
    }

    fn call(&self, args: &[N]) -> Result<N> {
        let mut result = None;

        for cur in args {
            match result {
                None => result = Some(cur.clone()),
                Some(ref n) => {
                    result = Some(n.clone() * cur.clone());
                }
            }
        }

        result.ok_or(Error::from(ErrorKind::InvalidArgumentCount))
    }
}

pub struct AvgFunction;
impl<N: Add<N, Output = N> + Div<N, Output = N> + FromPrimitive + Clone> Function<N>
    for AvgFunction
{
    fn name(&self) -> &str {
        "avg"
    }

    fn call(&self, args: &[N]) -> Result<N> {
        let mut sum = None;

        for cur in args {
            match sum {
                None => sum = Some(cur.clone()),
                Some(ref n) => {
                    sum = Some(n.clone() + cur.clone());
                }
            }
        }

        match sum {
            Some(n) => Ok(n / N::from_usize(args.len()).unwrap()),
            None => Err(Error::from(ErrorKind::InvalidArgumentCount)),
        }
    }
}
