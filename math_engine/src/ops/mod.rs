pub mod checked;
pub mod unchecked;

pub mod math {
    use crate::error::*;
    use crate::function::{
        Associativity, BinaryFunction, Function, Notation, Precedence, UnaryFunction,
    };
    use crate::utils::gamma::gamma;
    use num_traits::{FromPrimitive, Inv, ToPrimitive, Zero};

    pub struct UnaryPlus;
    impl<N> UnaryFunction<N> for UnaryPlus {
        fn name(&self) -> &str {
            "+"
        }

        fn notation(&self) -> Notation {
            Notation::Prefix
        }

        fn call(&self, value: N) -> Result<N> {
            Ok(value)
        }
    }

    pub struct Factorial;
    impl<N: Zero + PartialOrd + ToPrimitive + FromPrimitive> UnaryFunction<N> for Factorial {
        fn name(&self) -> &str {
            "!"
        }

        fn notation(&self) -> Notation {
            Notation::Postfix
        }

        fn call(&self, value: N) -> Result<N> {
            if value < N::zero() {
                return Err(Error::from(ErrorKind::NegativeValue));
            }

            let mut total = value.to_f64().ok_or(Error::from(ErrorKind::Overflow))?;
            let mut n: f64 = total - 1.0;

            while n > 1.0 {
                total *= n;
                n -= 1.0;
            }

            if !n.is_zero() {
                total *= gamma(n + 1.0);
            }

            let result = N::from_f64(total).ok_or(Error::from(ErrorKind::Overflow))?;
            Ok(result)
        }
    }

    pub struct PowOperator;
    impl<N: ToPrimitive + FromPrimitive> BinaryFunction<N> for PowOperator {
        fn name(&self) -> &str {
            "^"
        }

        fn precedence(&self) -> Precedence {
            Precedence::HIGH
        }

        fn associativity(&self) -> Associativity {
            Associativity::Right
        }

        fn call(&self, left: N, right: N) -> Result<N> {
            let base = left.to_f64().ok_or(Error::from(ErrorKind::Overflow))?;
            let exponent = right.to_f64().ok_or(Error::from(ErrorKind::Overflow))?;

            // Result
            N::from_f64(f64::powf(base, exponent)).ok_or(Error::from(ErrorKind::Overflow))
        }
    }

    pub struct MaxFunction;
    impl<N: PartialOrd + Clone> Function<N> for MaxFunction {
        fn name(&self) -> &str {
            "max"
        }

        fn call(&self, args: &[N]) -> Result<N> {
            let mut result = None;

            for cur in args {
                match result {
                    None => result = Some(cur.clone()),
                    Some(ref max) => {
                        if cur > max {
                            result = Some(cur.clone())
                        }
                    }
                }
            }

            result.ok_or(Error::from(ErrorKind::InvalidArgumentCount))
        }
    }

    pub struct MinFunction;
    impl<N: PartialOrd + Clone> Function<N> for MinFunction {
        fn name(&self) -> &str {
            "min"
        }

        fn call(&self, args: &[N]) -> Result<N> {
            let mut result = None;

            for cur in args {
                match result {
                    None => result = Some(cur.clone()),
                    Some(ref min) => {
                        if cur < min {
                            result = Some(cur.clone());
                        }
                    }
                }
            }

            result.ok_or(Error::from(ErrorKind::InvalidArgumentCount))
        }
    }

    macro_rules! forward_func_impl {
        ($func_name:ident, $method_name:ident) => {
            impl<N: ToPrimitive + FromPrimitive> Function<N> for $func_name {
                fn name(&self) -> &str {
                    stringify!($method_name)
                }

                fn call(&self, args: &[N]) -> Result<N> {
                    if args.len() != 1 {
                        Err(Error::from(ErrorKind::InvalidArgumentCount))
                    } else {
                        match args[0].to_f64().map(f64::$method_name) {
                            Some(n) => {
                                if n.is_nan() || n.is_infinite() {
                                    return Err(Error::from(ErrorKind::NAN));
                                } else {
                                    return N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow));
                                }
                            },
                            None => Err(Error::from(ErrorKind::Overflow)),
                        }
                    }
                }
            }
        };

        ($func_name:ident, $method_name:ident, $name:ident) => {
            impl<N: ToPrimitive + FromPrimitive> Function<N> for $func_name {
                fn name(&self) -> &str {
                    stringify!($name)
                }

                fn call(&self, args: &[N]) -> Result<N> {
                    if args.len() != 1 {
                        Err(Error::from(ErrorKind::InvalidArgumentCount))
                    } else {
                        match args[0].to_f64().map(f64::$method_name) {
                            Some(n) => {
                                if n.is_nan() || n.is_infinite() {
                                    return Err(Error::from(ErrorKind::NAN));
                                } else {
                                    return N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow));
                                }
                            },
                            None => Err(Error::from(ErrorKind::Overflow)),
                        }
                    }
                }
            }
        };
    }

    pub struct FloorFunction;
    forward_func_impl!(FloorFunction, floor);

    pub struct CeilFunction;
    forward_func_impl!(CeilFunction, ceil);

    pub struct TruncateFunction;
    forward_func_impl!(TruncateFunction, trunc, truncate);

    pub struct RoundFunction;
    forward_func_impl!(RoundFunction, round);

    pub struct SignFunction;
    forward_func_impl!(SignFunction, signum, sign);

    pub struct SqrtFunction;
    forward_func_impl!(SqrtFunction, sqrt);

    pub struct ExpFunction;
    forward_func_impl!(ExpFunction, exp);

    pub struct LnFunction;
    forward_func_impl!(LnFunction, ln);

    pub struct LogFunction;
    impl<N: ToPrimitive + FromPrimitive> Function<N> for LogFunction{
        fn name(&self) -> &str {
            "log"
        }

        fn call(&self, args: &[N]) -> Result<N> {
            match args.len(){
                1 => {
                    match args[0].to_f64().map(f64::log10){
                        Some(n) => {
                            if n.is_nan() || n.is_infinite(){
                                Err(Error::from(ErrorKind::NAN))
                            }
                            else{
                                N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow))
                            }
                        }
                        None => Err(Error::from(ErrorKind::Overflow))
                    }
                },
                2 => {
                    let x = args[0].to_f64();
                    let y = args[1].to_f64();

                    match (x, y){
                        (Some(value), Some(base)) => {
                            let result = value.log(base);
                            if result.is_nan() || result.is_infinite(){
                                Err(Error::from(ErrorKind::NAN))
                            }
                            else{
                                N::from_f64(result).ok_or(Error::from(ErrorKind::Overflow))
                            }
                        },
                        _ => Err(Error::from(ErrorKind::Overflow))
                    }
                },
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    //////////////////// Trigonometric ////////////////////

    macro_rules! impl_trig {
        ($t:ty, $method_name:ident) => {
            impl<N: ToPrimitive + FromPrimitive> Function<N> for $t {
                fn name(&self) -> &str {
                    stringify!($method_name)
                }

                fn call(&self, args: &[N]) -> Result<N> {
                    if args.len() != 1 {
                        Err(Error::from(ErrorKind::InvalidArgumentCount))
                    } else {
                        match args[0].to_f64().map(f64::to_radians).map(f64::$method_name) {
                            Some(n) => {
                                if n.is_nan() || n.is_infinite() {
                                    return Err(Error::from(ErrorKind::NAN));
                                } else {
                                    return N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow));
                                }
                            },
                            None => Err(Error::from(ErrorKind::Overflow)),
                        }
                    }
                }
            }
        };
    }

    macro_rules! impl_trig_inv {
        ($t:ty, $method_name:ident, $name:ident) => {
            impl<N: ToPrimitive + FromPrimitive> Function<N> for $t {
                fn name(&self) -> &str {
                    stringify!($name)
                }

                fn call(&self, args: &[N]) -> Result<N> {
                    if args.len() != 1 {
                        Err(Error::from(ErrorKind::InvalidArgumentCount))
                    } else {
                        match args[0].to_f64().map(f64::to_radians).map(f64::$method_name).map(f64::inv){
                            Some(n) => {
                                if n.is_nan() || n.is_infinite() {
                                    return Err(Error::from(ErrorKind::NAN));
                                } else {
                                    return N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow));
                                }
                            }
                            None => Err(Error::from(ErrorKind::Overflow)),
                        }
                    }
                }
            }
        };
    }

    pub struct SinFunction;
    pub struct CosFunction;
    pub struct TanFunction;

    pub struct CscFunction;
    pub struct SecFunction;
    pub struct CotFunction;

    impl_trig!(SinFunction, sin);
    impl_trig!(CosFunction, cos);
    impl_trig!(TanFunction, tan);
    impl_trig_inv!(CscFunction, sin, csc);
    impl_trig_inv!(SecFunction, cos, sec);
    impl_trig_inv!(CotFunction, tan, cot);

    pub struct ASinFunction;
    pub struct ACosFunction;
    pub struct ATanFunction;

    pub struct ACscFunction;
    pub struct ASecFunction;
    pub struct ACotFunction;

    impl_trig!(ASinFunction, asin);
    impl_trig!(ACosFunction, acos);
    impl_trig!(ATanFunction, atan);
    impl_trig_inv!(ACscFunction, asin, acsc);
    impl_trig_inv!(ASecFunction, acos, asec);
    impl_trig_inv!(ACotFunction, atan, acot);

    pub struct SinhFunction;
    pub struct CoshFunction;
    pub struct TanhFunction;

    pub struct CschFunction;
    pub struct SechFunction;
    pub struct CothFunction;

    impl_trig!(SinhFunction, sinh);
    impl_trig!(CoshFunction, cosh);
    impl_trig!(TanhFunction, tanh);
    impl_trig_inv!(CschFunction, sinh, csch);
    impl_trig_inv!(SechFunction, cosh, sech);
    impl_trig_inv!(CothFunction, tanh, coth);

    pub struct ASinhFunction;
    pub struct ACoshFunction;
    pub struct ATanhFunction;

    pub struct ACschFunction;
    pub struct ASechFunction;
    pub struct ACothFunction;

    impl_trig!(ASinhFunction, asinh);
    impl_trig!(ACoshFunction, acosh);
    impl_trig!(ATanhFunction, atanh);
    impl_trig_inv!(ACschFunction, asinh, acsch);
    impl_trig_inv!(ASechFunction, acosh, asech);
    impl_trig_inv!(ACothFunction, atanh, acoth);
}
