use crate::context::{DefaultContext, Config, Context};
use num_complex::Complex64;
use ops::*;
use num_traits::FromPrimitive;
use crate::ops::unchecked::*;

impl <'a> DefaultContext<'a, Complex64>{
    #[inline]
    pub fn new_complex() -> Self{
        Self::new_complex_with_config(Config::new().with_complex_number())
    }

    pub fn new_complex_with_config(config: Config) -> Self{
        let mut context = DefaultContext::new_with_config(config.with_complex_number());
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

pub mod ops{
    use crate::function::{BinaryFunction, Precedence, Associativity, Function};
    use num_complex::{Complex, Complex64};
    use crate::error::*;
    use num_traits::Zero;

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

    pub struct SqrtFunction;
    impl Function<Complex64> for SqrtFunction {
        fn name(&self) -> &str {
            "sqrt"
        }

        fn call(&self, args: &[Complex64]) -> Result<Complex64> {
            match args.len(){
                1 => Ok(args[0].sqrt()),
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    pub struct LogFunction;
    impl Function<Complex64> for LogFunction{
        fn name(&self) -> &str {
            "log"
        }

        fn call(&self, args: &[Complex64]) -> Result<Complex64> {
            match args.len(){
                1 => Ok(args[0].log(10_f64)),
                2 => {
                    match args[1].im{
                        n if n.is_zero() => Ok(args[0].log(args[1].re)),
                        _ => Err(Error::new(ErrorKind::InvalidInput, "Expected decimal base"))
                    }
                },
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    pub struct LnFunction;
    impl Function<Complex64> for LnFunction{
        fn name(&self) -> &str {
            "ln"
        }

        fn call(&self, args: &[Complex64]) -> Result<Complex64> {
            match args.len(){
                1 => Ok(args[0].ln()),
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    pub struct ExpFunction;
    impl Function<Complex64> for ExpFunction{
        fn name(&self) -> &str {
            "exp"
        }

        fn call(&self, args: &[Complex64]) -> Result<Complex64> {
            match args.len(){
                1 => Ok(args[0].exp()),
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    pub struct SinFunction;
    impl Function<Complex64> for SinFunction{
        fn name(&self) -> &str {
            "sin"
        }

        fn call(&self, args: &[Complex64]) -> Result<Complex64> {
            match args.len(){
                1 => Ok(args[0].sin()),
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    pub struct CosFunction;
    impl Function<Complex64> for CosFunction{
        fn name(&self) -> &str {
            "cos"
        }

        fn call(&self, args: &[Complex64]) -> Result<Complex64> {
            match args.len(){
                1 => Ok(args[0].cos()),
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    pub struct TanFunction;
    impl Function<Complex64> for TanFunction{
        fn name(&self) -> &str {
            "tan"
        }

        fn call(&self, args: &[Complex64]) -> Result<Complex64> {
            match args.len(){
                1 => {
                    let cos = args[1].cos();

                    if cos.is_zero(){
                        return Err(Error::from(ErrorKind::NAN));
                    }

                    Ok(args[1].sin() / cos)
                },
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    pub struct CscFunction;
    impl Function<Complex64> for CscFunction{
        fn name(&self) -> &str {
            "csc"
        }

        fn call(&self, args: &[Complex64]) -> Result<Complex64> {
            match args.len(){
                1 => Ok(args[0].sin().inv()),
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    pub struct SecFunction;
    impl Function<Complex64> for SecFunction{
        fn name(&self) -> &str {
            "sec"
        }

        fn call(&self, args: &[Complex64]) -> Result<Complex64> {
            match args.len(){
                1 => Ok(args[0].cos().inv()),
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    pub struct CotFunction;
    impl Function<Complex64> for CotFunction{
        fn name(&self) -> &str {
            "cot"
        }

        fn call(&self, args: &[Complex64]) -> Result<Complex64> {
            match args.len(){
                1 => {
                    let sin = args[1].sin();

                    if sin.is_zero(){
                        return Err(Error::from(ErrorKind::NAN));
                    }

                    Ok(args[1].cos() / sin)
                },
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }
}