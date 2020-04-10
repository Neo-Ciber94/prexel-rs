use num_traits::{FromPrimitive, Zero};

use crate::error::*;
use crate::function::{
    Associativity, BinaryFunction, Function, Notation, Precedence, UnaryFunction,
};
use crate::num::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub};
use crate::Result;

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
        } else if args[0] >= N::zero() {
            Ok(args[0].clone())
        } else {
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
                    result = Some(n.checked_add(&cur)
                        .ok_or(Error::from(ErrorKind::Overflow))?);
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
                    result = Some(n.checked_mul(&cur)
                        .ok_or(Error::from(ErrorKind::Overflow))?);
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
                    sum = Some(n.checked_add(&cur)
                        .ok_or(Error::from(ErrorKind::Overflow))?);
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

#[cfg(test)]
mod tests{
    use super::*;
    fn empty_array<T>() -> Box<[T]>{
        vec![].into_boxed_slice()
    }

    #[test]
    fn add_test(){
        let instance = AddOperator;

        assert_eq!(instance.call(10_f64, 4_f64), Ok(14_f64));
        assert_eq!(instance.call(3, 7), Ok(10));
        assert!(instance.call(i32::max_value(), 10).is_err());
    }

    #[test]
    fn sub_test(){
        let instance = SubOperator;

        assert_eq!(instance.call(10_f64, 4_f64), Ok(6_f64));
        assert_eq!(instance.call(3, 7), Ok(-4));
        assert!(instance.call(i32::min_value(), 10).is_err());
    }

    #[test]
    fn mul_test(){
        let instance = MulOperator;

        assert_eq!(instance.call(10_f64, 4_f64), Ok(40_f64));
        assert_eq!(instance.call(3, 7), Ok(21));
        assert!(instance.call(i32::max_value(), 10).is_err());
    }

    #[test]
    fn div_test(){
        let instance = DivOperator;

        assert_eq!(instance.call(10_f64, 4_f64), Ok(2.5_f64));
        assert_eq!(instance.call(20, 4), Ok(5));
        assert!(instance.call(5, 0).is_err());
    }

    #[test]
    fn mod_test(){
        let instance = ModOperator;

        assert_eq!(instance.call(10_f64, 4_f64), Ok(2_f64));
        assert_eq!(instance.call(20, 4), Ok(0));
        assert!(instance.call(5, 0).is_err());
    }

    #[test]
    fn unary_minus_test(){
        let instance = UnaryMinus;

        assert_eq!(instance.call(10_f64), Ok(-10_f64));
        assert_eq!(instance.call(-5), Ok(5));

        assert!(instance.call(i32::min_value()).is_err());
    }

    #[test]
    fn abs_test(){
        let instance = AbsFunction;

        assert_eq!(instance.call(&[-5_f64]), Ok(5_f64));
        assert_eq!(instance.call(&[3]), Ok(3));

        assert!(instance.call(&[i32::min_value()]).is_err());
    }

    #[test]
    fn sum_test(){
        let instance = SumFunction;

        assert_eq!(instance.call(&[1_f64, 2_f64, 3_f64]), Ok(6_f64));
        assert_eq!(instance.call(&[2, 4, 6]), Ok(12));

        assert!(instance.call(&[2]).is_ok());
        assert!(instance.call(empty_array::<i64>().as_ref()).is_err());
        assert!(instance.call(&[i32::max_value(), 10, 20]).is_err());
    }

    #[test]
    fn prod_test(){
        let instance = ProdFunction;

        assert_eq!(instance.call(&[2_f64, 3_f64, 4_f64]), Ok(24_f64));
        assert_eq!(instance.call(&[2, 4, 6]), Ok(48));

        assert!(instance.call(&[2]).is_ok());
        assert!(instance.call(empty_array::<i64>().as_ref()).is_err());
        assert!(instance.call(&[i32::max_value(), 10, 20]).is_err());
    }

    #[test]
    fn avg_test(){
        let instance = AvgFunction;

        assert_eq!(instance.call(&[1_f64, 2_f64, 3_f64, 4_f64]), Ok(2.5_f64));
        assert_eq!(instance.call(&[2, 4, 6]), Ok(4));

        assert!(instance.call(&[2]).is_ok());
        assert!(instance.call(empty_array::<i64>().as_ref()).is_err());
        assert!(instance.call(&[i32::max_value(), 10, 20, 30]).is_err());
    }
}