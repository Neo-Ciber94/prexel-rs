pub type Complex<T> = num_complex::Complex<T>;

pub mod ops {
    use num_complex::Complex;
    use num_traits::{Float, FromPrimitive};
    use rand::random;
    use std::fmt::{Display, Debug};

    use crate::error::*;
    use crate::function::{Associativity, BinaryFunction, Function, Precedence};
    use crate::Result;

    #[cfg(feature = "docs")]
    use crate::descriptions::Description;

    pub struct PowOperator;
    impl<T> BinaryFunction<Complex<T>> for PowOperator where T: Float{
        fn name(&self) -> &str {
            "^"
        }

        fn precedence(&self) -> Precedence {
            Precedence::HIGH
        }

        fn associativity(&self) -> Associativity {
            Associativity::Right
        }

        fn call(&self, left: Complex<T>, right: Complex<T>) -> Result<Complex<T>> {
            Ok(Complex::powc(&left, right))
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Pow.into())
        }
    }

    pub struct LogFunction;
    impl<T> Function<Complex<T>> for LogFunction where T: Float + FromPrimitive{
        fn name(&self) -> &str {
            "log"
        }

        fn call(&self, args: &[Complex<T>]) -> Result<Complex<T>> {
            match args.len() {
                1 => {
                    let base = T::from_f64(10_f64)
                        .ok_or_else(|| Error::from(ErrorKind::Overflow))?;
                   Ok(args[0].log(base))
                },
                2 => match args[1].im {
                    n if !n.is_zero() => {
                        Ok(args[0].log(args[1].re))
                    },
                    _ => Err(Error::new(ErrorKind::InvalidInput, "Expected decimal base")),
                },
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
            }
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Log.into())
        }
    }

    pub struct RandFunction;
    impl<T> Function<Complex<T>> for RandFunction where T: Float + FromPrimitive + Debug + Display{
        fn name(&self) -> &str {
            "random"
        }

        fn call(&self, args: &[Complex<T>]) -> Result<Complex<T>> {
            #[inline(always)]
            fn try_get_real<N: Float>(c: &Complex<N>) -> Result<N>{
                if !c.im.is_zero(){
                    Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Random(...) only accepts real numbers as arguments")
                    )
                }else{
                    Ok(c.re)
                }
            }

            #[inline(always)]
            fn random_t<N: FromPrimitive>() -> Result<N>{
                N::from_f64(random::<f64>())
                    .ok_or_else(|| Error::from(ErrorKind::Overflow))
            }

            match args.len(){
                0 => {
                    let re = random_t::<T>()?;
                    let im = random_t::<T>()?;
                    Ok(Complex::new(re, im))
                },
                1 => {
                    let max = try_get_real(&args[0])?;
                    if max.is_sign_negative(){
                        return Err(Error::from(ErrorKind::NegativeValue))
                    }

                    let re = random_t::<T>()? * max;
                    let im = random_t::<T>()? * max;
                    Ok(Complex::new(re, im))
                },
                2 => {
                    let min = try_get_real(&args[0])?;
                    let max = try_get_real(&args[1])?;
                    if min.is_sign_negative() || max.is_sign_negative(){
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid `Random` arguments: `min > max`, {} > {}", min, max)
                        ));
                    }

                    let re = min + ((max - min) * random_t::<T>()?);
                    let im = min + ((max - min) * random_t::<T>()?);
                    Ok(Complex::new(re, im))
                }
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
            }
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Rand.into())
        }
    }

    #[cfg(not(feature = "docs"))]
    macro_rules! forward_impl_func {
        ($t:ty, $method_name:ident) => {
            forward_impl_func!($t, $method_name, $method_name);
        };

        ($t:ty, $method_name:ident, $name:ident) => {
            impl<T> Function<Complex<T>> for $t where T: Float{
                fn name(&self) -> &str {
                    stringify!($name)
                }

                fn call(&self, args: &[Complex<T>]) -> Result<Complex<T>> {
                    match args.len() {
                        1 => Ok(args[0].$method_name()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }
            }
        };
    }

    #[cfg(not(feature = "docs"))]
    macro_rules! forward_impl_func_inv {
        ($t:ty, $method_name:ident) => {
            forward_impl_func_inv!($t, $method_name, $method_name);
        };

        ($t:ty, $method_name:ident, $name:ident) => {
            impl<T> Function<Complex<T>> for $t where T: Float {
                fn name(&self) -> &str {
                    stringify!($name)
                }

                fn call(&self, args: &[Complex<T>]) -> Result<Complex<T>> {
                    match args.len() {
                        1 => Ok(args[0].$method_name().inv()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }
            }
        };
    }

    #[cfg(feature = "docs")]
    macro_rules! forward_impl_func {
        ($t:ty, $method_name:ident, $description:expr) => {
            forward_impl_func!($t, $method_name, $method_name, $description);
        };

        ($t:ty, $method_name:ident, $name:ident, $description:expr) => {
            impl<T> Function<Complex<T>> for $t where T: Float{
                fn name(&self) -> &str {
                    stringify!($name)
                }

                fn call(&self, args: &[Complex<T>]) -> Result<Complex<T>> {
                    match args.len() {
                        1 => Ok(args[0].$method_name()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }

                fn description(&self) -> Option<&str> {
                    Some($description.into())
                }
            }
        };
    }

    #[cfg(feature = "docs")]
    macro_rules! forward_impl_func_inv {
        ($t:ty, $method_name:ident, $description:expr) => {
            forward_impl_func_inv!($t, $method_name, $method_name);
        };

        ($t:ty, $method_name:ident, $name:ident, $description:expr) => {
            impl<T> Function<Complex<T>> for $t where T: Float {
                fn name(&self) -> &str {
                    stringify!($name)
                }

                fn call(&self, args: &[Complex<T>]) -> Result<Complex<T>> {
                    match args.len() {
                        1 => Ok(args[0].$method_name().inv()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }

                fn description(&self) -> Option<&str> {
                    Some($description.into())
                }
            }
        };
    }

    pub struct SqrtFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(SqrtFunction, sqrt);
    #[cfg(feature = "docs")]
    forward_impl_func!(SqrtFunction, sqrt, Description::Sqrt);

    pub struct LnFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(LnFunction, ln);
    #[cfg(feature = "docs")]
    forward_impl_func!(LnFunction, ln, Description::Ln);

    pub struct ExpFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(ExpFunction, exp);
    #[cfg(feature = "docs")]
    forward_impl_func!(ExpFunction, exp, Description::Exp);

    //////////////////// Trigonometric ////////////////////

    pub struct SinFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(SinFunction, sin);
    #[cfg(feature = "docs")]
    forward_impl_func!(SinFunction, sin, Description::Sin);

    pub struct CosFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(CosFunction, cos);
    #[cfg(feature = "docs")]
    forward_impl_func!(CosFunction, cos, Description::Cos);

    pub struct TanFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(TanFunction, tan);
    #[cfg(feature = "docs")]
    forward_impl_func!(TanFunction, tan, Description::Tan);

    pub struct CscFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(CscFunction, sin, csc);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(CscFunction, sin, csc, Description::Csc);

    pub struct SecFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(SecFunction, cos, sec);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(SecFunction, cos, sec, Description::Sec);

    pub struct CotFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(CotFunction, tan, cot);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(CotFunction, tan, cot, Description::Cot);

    //////////////////// Inverse Trigonometric ////////////////////

    pub struct ASinFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(ASinFunction, asin);
    #[cfg(feature = "docs")]
    forward_impl_func!(ASinFunction, asin, Description::ASin);

    pub struct ACosFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(ACosFunction, acos);
    #[cfg(feature = "docs")]
    forward_impl_func!(ACosFunction, acos, Description::ACos);

    pub struct ATanFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(ATanFunction, atan);
    #[cfg(feature = "docs")]
    forward_impl_func!(ATanFunction, atan, Description::ATan);

    pub struct ACscFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(ACscFunction, asin, acsc);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(ACscFunction, asin, acsc, Description::ACsc);

    pub struct ASecFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(ASecFunction, acos, asec);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(ASecFunction, acos, asec, Description::ASec);

    pub struct ACotFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(ACotFunction, atan, acot);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(ACotFunction, atan, acot, Description::ACot);

    //////////////////// Hyperbolic Trigonometric ////////////////////

    pub struct SinhFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(SinhFunction, sinh);
    #[cfg(feature = "docs")]
    forward_impl_func!(SinhFunction, sinh, Description::Sinh);

    pub struct CoshFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(CoshFunction, cosh);
    #[cfg(feature = "docs")]
    forward_impl_func!(CoshFunction, cosh, Description::Cosh);

    pub struct TanhFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(TanhFunction, tanh);
    #[cfg(feature = "docs")]
    forward_impl_func!(TanhFunction, tanh, Description::Tanh);

    pub struct CschFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(CschFunction, sinh, csch);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(CschFunction, sinh, csch, Description::Csch);

    pub struct SechFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(SechFunction, cosh, sech);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(SechFunction, cosh, sech, Description::Sech);

    pub struct CothFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(CothFunction, tanh, coth);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(CothFunction, tanh, coth, Description::Coth);

    //////////////////// Hyperbolic Inverse Trigonometric ////////////////////

    pub struct ASinhFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(ASinhFunction, asinh);
    #[cfg(feature = "docs")]
    forward_impl_func!(ASinhFunction, asinh, Description::ASinh);

    pub struct ACoshFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(ACoshFunction, acosh);
    #[cfg(feature = "docs")]
    forward_impl_func!(ACoshFunction, acosh, Description::ACosh);

    pub struct ATanhFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func!(ATanhFunction, atanh);
    #[cfg(feature = "docs")]
    forward_impl_func!(ATanhFunction, atanh, Description::ATanh);

    pub struct ACschFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(ACschFunction, asinh, acsch);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(ACschFunction, asinh, acsch, Description::ACsch);

    pub struct ASechFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(ASechFunction, acosh, asech);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(ASechFunction, acosh, asech, Description::ASech);

    pub struct ACothFunction;
    #[cfg(not(feature = "docs"))]
    forward_impl_func_inv!(ACothFunction, atanh, acoth);
    #[cfg(feature = "docs")]
    forward_impl_func_inv!(ACothFunction, atanh, acoth, Description::ACoth);
}

pub mod context {
    use num_complex::Complex;
    use num_traits::{FromPrimitive, Float};

    use crate::complex::ops::PowOperator;
    use crate::context::{Config, Context, DefaultContext};
    use crate::ops::unchecked::*;
    use crate::ops::math::UnaryPlus;
    use super::ops::*;
    use std::fmt::{Debug, Display};

    impl<'a, T> DefaultContext<'a, Complex<T>> where T: Float + FromPrimitive + Debug + Display {
        #[inline]
        pub fn new_complex() -> Self {
            Self::with_config_complex(Config::new()
                .with_complex_number(true))
        }

        pub fn with_config_complex(config: Config) -> Self {
            let mut context = DefaultContext::with_config(config.with_complex_number(true));
            context.add_constant("PI", Complex::from_f64(std::f64::consts::PI).unwrap()).unwrap();
            context.add_constant("E", Complex::from_f64(std::f64::consts::E).unwrap()).unwrap();
            context.add_constant("i", Complex::i()).unwrap();
            context.add_binary_function(AddOperator).unwrap();
            context.add_binary_function(SubOperator).unwrap();
            context.add_binary_function(MulOperator).unwrap();
            context.add_binary_function(DivOperator).unwrap();
            context.add_binary_function(ModOperator).unwrap();
            context.add_binary_function(PowOperator).unwrap();
            context.add_unary_function(UnaryPlus).unwrap();
            context.add_unary_function(UnaryMinus).unwrap();
            context.add_function(SumFunction).unwrap();
            context.add_function(AvgFunction).unwrap();
            context.add_function(ProdFunction).unwrap();
            context.add_function(SqrtFunction).unwrap();
            context.add_function(LnFunction).unwrap();
            context.add_function(LogFunction).unwrap();
            context.add_function(RandFunction).unwrap();
            context.add_function(ExpFunction).unwrap();
            context.add_function(SinFunction).unwrap();
            context.add_function(CosFunction).unwrap();
            context.add_function(TanFunction).unwrap();
            context.add_function(CscFunction).unwrap();
            context.add_function(SecFunction).unwrap();
            context.add_function(CotFunction).unwrap();
            context.add_function(ASinFunction).unwrap();
            context.add_function(ACosFunction).unwrap();
            context.add_function(ATanFunction).unwrap();
            context.add_function(ACscFunction).unwrap();
            context.add_function(ASecFunction).unwrap();
            context.add_function(ACotFunction).unwrap();
            context.add_function(SinhFunction).unwrap();
            context.add_function(CoshFunction).unwrap();
            context.add_function(TanhFunction).unwrap();
            context.add_function(CschFunction).unwrap();
            context.add_function(SechFunction).unwrap();
            context.add_function(CothFunction).unwrap();
            context.add_function(ASinhFunction).unwrap();
            context.add_function(ACoshFunction).unwrap();
            context.add_function(ATanhFunction).unwrap();
            context.add_function(ACschFunction).unwrap();
            context.add_function(ASechFunction).unwrap();
            context.add_function(ACothFunction).unwrap();
            context
        }
    }
}
