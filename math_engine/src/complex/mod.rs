pub type Complex64 = num_complex::Complex64;

pub mod ops {
    use num_complex::{Complex, Complex64};
    use num_traits::Zero;

    use crate::error::*;
    use crate::function::{Associativity, BinaryFunction, Function, Precedence};
    use crate::Result;

    pub struct PowOperator;
    impl BinaryFunction<Complex64> for PowOperator {
        fn name(&self) -> &str {
            "^"
        }

        fn precedence(&self) -> Precedence {
            Precedence::HIGH
        }

        fn associativity(&self) -> Associativity {
            Associativity::Right
        }

        fn call(&self, left: Complex64, right: Complex64) -> Result<Complex64> {
            Ok(Complex::powc(&left, right))
        }
    }

    pub struct LogFunction;
    impl Function<Complex64> for LogFunction {
        fn name(&self) -> &str {
            "log"
        }

        fn call(&self, args: &[Complex64]) -> Result<Complex64> {
            match args.len() {
                1 => Ok(args[0].log(10_f64)),
                2 => match args[1].im {
                    n if n.is_zero() => Ok(args[0].log(args[1].re)),
                    _ => Err(Error::new(ErrorKind::InvalidInput, "Expected decimal base")),
                },
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
            }
        }
    }

    macro_rules! forward_impl_func {
        ($t:ty, $method_name:ident) => {
            forward_impl_func!($t, $method_name, $method_name);
        };

        ($t:ty, $method_name:ident, $name:ident) => {
            impl Function<Complex64> for $t {
                fn name(&self) -> &str {
                    stringify!($name)
                }

                fn call(&self, args: &[Complex64]) -> Result<Complex64> {
                    match args.len() {
                        1 => Ok(args[0].$method_name()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }
            }
        };
    }

    macro_rules! forward_impl_func_inv {
        ($t:ty, $method_name:ident) => {
            forward_impl_func_inv!($t, $method_name, $method_name);
        };

        ($t:ty, $method_name:ident, $name:ident) => {
            impl Function<Complex64> for $t {
                fn name(&self) -> &str {
                    stringify!($name)
                }

                fn call(&self, args: &[Complex64]) -> Result<Complex64> {
                    match args.len() {
                        1 => Ok(args[0].$method_name().inv()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }
            }
        };
    }

    pub struct SqrtFunction;
    forward_impl_func!(SqrtFunction, sqrt);

    pub struct LnFunction;
    forward_impl_func!(LnFunction, ln);

    pub struct ExpFunction;
    forward_impl_func!(ExpFunction, exp);

    //////////////////// Trigonometric ////////////////////

    pub struct SinFunction;
    forward_impl_func!(SinFunction, sin);

    pub struct CosFunction;
    forward_impl_func!(CosFunction, cos);

    pub struct TanFunction;
    forward_impl_func!(TanFunction, tan);

    pub struct CscFunction;
    forward_impl_func_inv!(CscFunction, sin, csc);

    pub struct SecFunction;
    forward_impl_func_inv!(SecFunction, cos, sec);

    pub struct CotFunction;
    forward_impl_func_inv!(CotFunction, tan, cot);

    //////////////////// Inverse Trigonometric ////////////////////

    pub struct ASinFunction;
    forward_impl_func!(ASinFunction, asin);

    pub struct ACosFunction;
    forward_impl_func!(ACosFunction, acos);

    pub struct ATanFunction;
    forward_impl_func!(ATanFunction, atan);

    pub struct ACscFunction;
    forward_impl_func_inv!(ACscFunction, asin, acsc);

    pub struct ASecFunction;
    forward_impl_func_inv!(ASecFunction, acos, asec);

    pub struct ACotFunction;
    forward_impl_func_inv!(ACotFunction, atan, acot);

    //////////////////// Hyperbolic Trigonometric ////////////////////

    pub struct SinhFunction;
    forward_impl_func!(SinhFunction, sinh);

    pub struct CoshFunction;
    forward_impl_func!(CoshFunction, cosh);

    pub struct TanhFunction;
    forward_impl_func!(TanhFunction, tanh);

    pub struct CschFunction;
    forward_impl_func_inv!(CschFunction, sinh, csch);

    pub struct SechFunction;
    forward_impl_func_inv!(SechFunction, cosh, sech);

    pub struct CothFunction;
    forward_impl_func_inv!(CothFunction, tanh, coth);

    //////////////////// Hyperbolic Inverse Trigonometric ////////////////////

    pub struct ASinhFunction;
    forward_impl_func!(ASinhFunction, asinh);

    pub struct ACoshFunction;
    forward_impl_func!(ACoshFunction, acosh);

    pub struct ATanhFunction;
    forward_impl_func!(ATanhFunction, atanh);

    pub struct ACschFunction;
    forward_impl_func_inv!(ACschFunction, asinh, acsch);

    pub struct ASechFunction;
    forward_impl_func_inv!(ASechFunction, acosh, asech);

    pub struct ACothFunction;
    forward_impl_func_inv!(ACothFunction, atanh, acoth);
}

pub mod context {
    use num_complex::Complex64;
    use num_traits::FromPrimitive;

    use crate::complex::ops::PowOperator;
    use crate::context::{Config, Context, DefaultContext};
    use crate::ops::unchecked::*;

    impl<'a> DefaultContext<'a, Complex64> {
        #[inline]
        pub fn new_complex() -> Self {
            Self::new_complex_with_config(Config::new().with_complex_number())
        }

        pub fn new_complex_with_config(config: Config) -> Self {
            let mut context = DefaultContext::empty_with_config(config.with_complex_number());
            context.add_constant("PI", Complex64::from_f64(std::f64::consts::PI).unwrap());
            context.add_constant("E", Complex64::from_f64(std::f64::consts::E).unwrap());
            context.add_constant("i", Complex64::i());
            context.add_binary_function(AddOperator);
            context.add_binary_function(SubOperator);
            context.add_binary_function(MulOperator);
            context.add_binary_function(DivOperator);
            context.add_binary_function(ModOperator);
            context.add_binary_function(PowOperator);
            context.add_function(SumFunction);
            context.add_function(ProdFunction);
            context
        }
    }
}
