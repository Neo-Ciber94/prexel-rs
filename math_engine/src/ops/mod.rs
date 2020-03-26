pub mod checked;
pub mod unchecked;

pub mod math {
    use num_traits::{FromPrimitive, Inv, ToPrimitive, Zero};
    use rand::random;

    use crate::Result;
    use crate::error::*;
    use crate::function::{
        Associativity, BinaryFunction, Function, Notation, Precedence, UnaryFunction,
    };
    use crate::utils::gamma::gamma;

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
            forward_func_impl!($func_name, $method_name, $method_name);
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
                        let result = try_to_float(&args[0])?.$method_name();
                        N::from_f64(result)
                            .ok_or(Error::from(ErrorKind::Overflow))
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

    pub struct RandFunction;
    impl<N: ToPrimitive + FromPrimitive> Function<N> for RandFunction{
        #[inline]
        fn name(&self) -> &str {
            "random"
        }

        fn call(&self, args: &[N]) -> Result<N> {
            match args.len(){
                0 => N::from_f64(random::<f64>()).ok_or(Error::from(ErrorKind::Overflow)),
                1 => {
                    let max = try_to_float(&args[0])?;
                    N::from_f64(random::<f64>() * max)
                        .ok_or(Error::from(ErrorKind::Overflow))
                },
                2 => {
                    let min = try_to_float(&args[0])?;
                    let max = try_to_float(&args[1])?;
                    let value = min + ((max - min) *  random::<f64>());
                    N::from_f64(value)
                        .ok_or(Error::from(ErrorKind::Overflow))
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
                                    Err(Error::from(ErrorKind::NAN))
                                } else {
                                    N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow))
                                }
                            },
                            None => Err(Error::from(ErrorKind::Overflow)),
                        }
                    }
                }
            }
        };
    }

    macro_rules! impl_trig_rec {
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
                                    Err(Error::from(ErrorKind::NAN))
                                } else {
                                    N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow))
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
    impl_trig!(SinFunction, sin);

    pub struct CosFunction;
    impl_trig!(CosFunction, cos);

    pub struct TanFunction;
    impl_trig!(TanFunction, tan);

    pub struct CscFunction;
    impl_trig_rec!(CscFunction, sin, csc);

    pub struct SecFunction;
    impl_trig_rec!(SecFunction, cos, sec);

    pub struct CotFunction;
    impl_trig_rec!(CotFunction, tan, cot);

    pub struct SinhFunction;
    impl_trig!(SinhFunction, sinh);

    pub struct CoshFunction;
    impl_trig!(CoshFunction, cosh);

    pub struct TanhFunction;
    impl_trig!(TanhFunction, tanh);

    pub struct CschFunction;
    impl_trig_rec!(CschFunction, sinh, csch);

    pub struct SechFunction;
    impl_trig_rec!(SechFunction, cosh, sech);

    pub struct CothFunction;
    impl_trig_rec!(CothFunction, tanh, coth);

    //////////////////// Inverse Trigonometric ////////////////////

    macro_rules! impl_arc_trig {
        ($t:ty, $method_name:ident) => {
            impl_arc_trig!($t, $method_name, $method_name);
        };

        ($t:ty, $method_name:ident, $name:ident) => {
            impl<N: ToPrimitive + FromPrimitive> Function<N> for $t {
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
                                    Err(Error::from(ErrorKind::NAN))
                                } else {
                                    N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow))
                                }
                            },
                            None => Err(Error::from(ErrorKind::Overflow)),
                        }
                    }
                }
            }
        };
    }

    macro_rules! impl_arc_trig_rec {
        ($t:ty, $method_name:ident) => {
            impl_arc_trig_rec!($t, $method_name, $method_name);
        };

        ($t:ty, $method_name:ident, $name:ident) => {
            impl<N: ToPrimitive + FromPrimitive> Function<N> for $t {
                fn name(&self) -> &str {
                    stringify!($name)
                }

                fn call(&self, args: &[N]) -> Result<N> {
                    if args.len() != 1 {
                        Err(Error::from(ErrorKind::InvalidArgumentCount))
                    } else {
                        match args[0].to_f64().map(f64::inv).map(f64::$method_name){
                            Some(n) => {
                                if n.is_nan() || n.is_infinite() {
                                    Err(Error::from(ErrorKind::NAN))
                                } else {
                                    N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow))
                                }
                            }
                            None => Err(Error::from(ErrorKind::Overflow)),
                        }
                    }
                }
            }
        };
    }

    pub struct ASinFunction;
    impl_arc_trig!(ASinFunction, asin);

    pub struct ACosFunction;
    impl_arc_trig!(ACosFunction, acos);

    pub struct ATanFunction;
    impl<N: ToPrimitive + FromPrimitive> Function<N> for ATanFunction {
        fn name(&self) -> &str {
            stringify!( atan )
        }

        fn call(&self, args: &[N]) -> Result<N> {
            match args.len(){
                1 => {
                    match args[0].to_f64().map(f64::atan) {
                        Some(n) => {
                            if n.is_nan() || n.is_infinite() {
                                Err(Error::from(ErrorKind::NAN))
                            } else {
                                N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow))
                            }
                        }
                        None => Err(Error::from(ErrorKind::Overflow)),
                    }
                },
                2 => {
                    let opy = args[0].to_f64();
                    let opx = args[1].to_f64();

                    if opy.is_none() || opx.is_none(){
                        Err(Error::from(ErrorKind::Overflow))
                    }
                    else{
                        let result = opy.unwrap().atan2(opx.unwrap());
                        if result.is_nan() ||  result.is_infinite(){
                            Err(Error::from(ErrorKind::NAN))
                        }
                        else{
                            N::from_f64(result)
                                .ok_or(Error::from(ErrorKind::Overflow))
                        }
                    }
                }
                _ =>  Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    pub struct ACscFunction;
    impl_arc_trig_rec!(ACscFunction, asin, acsc);

    pub struct ASecFunction;
    impl_arc_trig_rec!(ASecFunction, acos, asec);

    pub struct ACotFunction;
    impl_arc_trig_rec!(ACotFunction, atan, acot);

    pub struct ASinhFunction;
    impl_arc_trig!(ASinhFunction, asinh);

    pub struct ACoshFunction;
    impl_arc_trig!(ACoshFunction, acosh);

    pub struct ATanhFunction;
    impl_arc_trig!(ATanhFunction, atanh);

    pub struct ACschFunction;
    impl_arc_trig_rec!(ACschFunction, asinh, acsch);

    pub struct ASechFunction;
    impl_arc_trig_rec!(ASechFunction, acosh, asech);

    pub struct ACothFunction;
    impl_arc_trig_rec!(ACothFunction, atanh, acoth);

    #[inline(always)]
    pub(crate) fn try_to_float<N: ToPrimitive>(n: &N) -> Result<f64>{
        match n.to_f64(){
            Some(n) => {
                if n.is_nan() || n.is_infinite(){
                    Err(Error::from(ErrorKind::NAN))
                }
                else{
                    Ok(n)
                }
            },
            None => Err(Error::from(ErrorKind::Overflow))
        }
    }
}