use num_traits::{FromPrimitive, Zero};

use crate::Result;
use crate::error::*;
use crate::num::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub};
use crate::function::{BinaryFunction, UnaryFunction, Function, Precedence, Associativity, Notation};

pub struct AddOperator;
impl<N: CheckedAdd> BinaryFunction<N> for AddOperator {
    fn name(&self) -> &str {
        "+"
    }

    fn precedence(&self) -> Precedence {
        Precedence::LOW
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: N, right: N) -> Result<N> {
        left.checked_add(&right)
            .ok_or(Error::from(ErrorKind::Overflow))
    }
}

pub struct SubOperator;
impl<N: CheckedSub> BinaryFunction<N> for SubOperator {
    fn name(&self) -> &str {
        "-"
    }

    fn precedence(&self) -> Precedence {
        Precedence::LOW
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: N, right: N) -> Result<N> {
        left.checked_sub(&right)
            .ok_or(Error::from(ErrorKind::Overflow))
    }
}

pub struct MulOperator;
impl<N: CheckedMul> BinaryFunction<N> for MulOperator {
    fn name(&self) -> &str {
        "*"
    }

    fn precedence(&self) -> Precedence {
        Precedence::MEDIUM
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: N, right: N) -> Result<N> {
        left.checked_mul(&right)
            .ok_or(Error::from(ErrorKind::Overflow))
    }
}

pub struct DivOperator;
impl<N: CheckedDiv + Zero> BinaryFunction<N> for DivOperator {
    fn name(&self) -> &str {
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

        left.checked_div(&right)
            .ok_or(Error::from(ErrorKind::Overflow))
    }
}

pub struct ModOperator;
impl<N: CheckedRem + Zero> BinaryFunction<N> for ModOperator {
    fn name(&self) -> &str {
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

        left.checked_rem(&right)
            .ok_or(Error::from(ErrorKind::Overflow))
    }
}

pub struct UnaryMinus;
impl<N: CheckedNeg> UnaryFunction<N> for UnaryMinus {
    fn name(&self) -> &str {
        "-"
    }

    fn notation(&self) -> Notation {
        Notation::Prefix
    }

    fn call(&self, value: N) -> Result<N> {
        value.checked_neg().ok_or(Error::from(ErrorKind::Overflow))
    }
}

pub struct AbsFunction;
impl<N: Zero + PartialOrd + CheckedNeg + Clone> Function<N> for AbsFunction {
    fn name(&self) -> &str {
        "abs"
    }

    fn call(&self, args: &[N]) -> Result<N> {
        if args.len() != 1 {
            Err(Error::from(ErrorKind::InvalidArgumentCount))
        }
        else if args[0] >= N::zero() {
            Ok(args[0].clone())
        }
        else {
            args[0]
                .checked_neg()
                .ok_or(Error::from(ErrorKind::Overflow))
        }
    }
}

pub struct SumFunction;
impl<N: CheckedAdd + Clone> Function<N> for SumFunction {
    fn name(&self) -> &str {
        "sum"
    }

    fn call(&self, args: &[N]) -> Result<N> {
        let mut result = None;

        for cur in args {
            match result {
                None => result = Some(cur.clone()),
                Some(ref n) => {
                    result = n.checked_add(&cur);
                }
            }
        }

        result.ok_or(Error::from(ErrorKind::InvalidArgumentCount))
    }
}

pub struct ProdFunction;
impl<N: CheckedMul + Clone> Function<N> for ProdFunction {
    fn name(&self) -> &str {
        "product"
    }

    fn call(&self, args: &[N]) -> Result<N> {
        let mut result = None;

        for cur in args {
            match result {
                None => result = Some(cur.clone()),
                Some(ref n) => {
                    result = n.checked_mul(&cur);
                }
            }
        }

        result.ok_or(Error::from(ErrorKind::InvalidArgumentCount))
    }
}

pub struct AvgFunction;
impl<N: CheckedAdd + CheckedDiv + FromPrimitive + Clone> Function<N> for AvgFunction {
    fn name(&self) -> &str {
        "avg"
    }

    fn call(&self, args: &[N]) -> Result<N> {
        let mut sum = None;

        for cur in args {
            match sum {
                None => sum = Some(cur.clone()),
                Some(ref n) => {
                    sum = n.checked_add(&cur);
                }
            }
        }

        match sum {
            Some(n) => {
                let result = n
                    .checked_div(&N::from_usize(args.len()).unwrap())
                    .ok_or(Error::from(ErrorKind::Overflow))?;

                Ok(result)
            }
            None => Err(Error::from(ErrorKind::InvalidArgumentCount)),
        }
    }
}