pub type Decimal = rust_decimal::Decimal;

pub mod decimal_ext;

pub mod consts {
    use rust_decimal::Decimal;
    use rust_decimal_macros::*;

    pub(crate) const TAYLOR_SERIES_ITERATIONS: u32 = 100;
    pub(crate) const PRECISION: Decimal = dec!(0.0000000000000001);

    //////////////////////// Constants ////////////////////////
    /// Euler's number (e)
    pub const E: Decimal = dec!(2.7182818284590452353602874714);
    /// 1/e
    pub const E_INV: Decimal = dec!(0.3678794411714423215955237702);
    /// e²
    pub const E_POW_2: Decimal = dec!(7.3890560989306502272304274605);
    /// Archimedes' constant π
    pub const PI: Decimal = dec!(3.1415926535897932384626433833);
    /// 2π
    pub const PI_2: Decimal = dec!(6.2831853071795864769252867666);
    /// -π
    pub const PI_MINUS: Decimal = dec!(-3.1415926535897932384626433833);
    /// -2π
    pub const PI_2_MINUS: Decimal = dec!(-6.2831853071795864769252867666);
    /// π/2
    pub const PI_FRACT_2: Decimal = dec!(1.5707963267948966192313216916);
    /// π/3
    pub const PI_FRACT_3: Decimal = dec!(1.0471975511965977461542144610);
    /// π/4
    pub const PI_FRACT_4: Decimal = dec!(0.7853981633974483096156608458);
    /// π/6
    pub const PI_FRACT_6: Decimal = dec!(0.5235987755982988730771072305);
    /// π/8
    pub const PI_FRACT_8: Decimal = dec!(0.3926990816987241548078304229);
    /// 3π/2
    pub const PI_3_FRACT_2: Decimal = dec!(4.7123889803846898576939650750);
    /// Ln(2)
    pub const LN_2: Decimal = dec!(0.6931471805599453094172321215);
    /// Ln(10)
    pub const LN_10: Decimal = dec!(2.3025850929940456840179914546844);
    /// 1/Ln(10)
    pub const LN_10_INV: Decimal = dec!(0.4342944819032518276511289189);
    /// ✓2
    pub const SQRT_2: Decimal = dec!(1.4142135623730950488016887242097);
    /// 0.5
    pub const HALF: Decimal = dec!(0.5);
    /// 1/3
    pub const ONE_FRACT_3: Decimal = dec!(0.33333333333333333333333333333);
    /// -1
    pub const ONE_MINUS: Decimal = dec!(-1);
    /// 0
    pub const ZERO: Decimal = dec!(0);
    /// 1
    pub const ONE: Decimal = dec!(1);
    /// 2
    pub const TWO: Decimal = dec!(2);
    /// 3
    pub const THREE: Decimal = dec!(3);
    /// 10
    pub const TEN: Decimal = dec!(10);
}

pub mod ops {
    use num_traits::FromPrimitive;
    use rust_decimal::Decimal;

    use crate::decimal::consts;
    use crate::decimal::decimal_ext::DecimalExt;
    use crate::error::*;
    use crate::function::{
        Associativity, BinaryFunction, Function, Notation, Precedence, UnaryFunction,
    };

    pub struct AddOperator;
    impl BinaryFunction<Decimal> for AddOperator {
        #[inline]
        fn name(&self) -> &str {
            "+"
        }

        #[inline]
        fn precedence(&self) -> Precedence {
            Precedence::LOW
        }

        #[inline]
        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        #[inline]
        fn call(&self, left: Decimal, right: Decimal) -> Result<Decimal> {
            left.checked_add(right)
                .ok_or(Error::from(ErrorKind::Overflow))
        }
    }

    pub struct SubOperator;
    impl BinaryFunction<Decimal> for SubOperator {
        #[inline]
        fn name(&self) -> &str {
            "-"
        }

        #[inline]
        fn precedence(&self) -> Precedence {
            Precedence::LOW
        }

        #[inline]
        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        #[inline]
        fn call(&self, left: Decimal, right: Decimal) -> Result<Decimal> {
            left.checked_sub(right)
                .ok_or(Error::from(ErrorKind::Overflow))
        }
    }

    pub struct MulOperator;
    impl BinaryFunction<Decimal> for MulOperator {
        #[inline]
        fn name(&self) -> &str {
            "*"
        }

        #[inline]
        fn precedence(&self) -> Precedence {
            Precedence::MEDIUM
        }

        #[inline]
        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        #[inline]
        fn call(&self, left: Decimal, right: Decimal) -> Result<Decimal> {
            left.checked_mul(right)
                .ok_or(Error::from(ErrorKind::Overflow))
        }
    }

    pub struct DivOperator;
    impl BinaryFunction<Decimal> for DivOperator {
        #[inline]
        fn name(&self) -> &str {
            "/"
        }

        #[inline]
        fn precedence(&self) -> Precedence {
            Precedence::MEDIUM
        }

        #[inline]
        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        #[inline]
        fn call(&self, left: Decimal, right: Decimal) -> Result<Decimal> {
            left.checked_div(right)
                .ok_or(Error::from(ErrorKind::Overflow))
        }
    }

    pub struct ModOperator;
    impl BinaryFunction<Decimal> for ModOperator {
        #[inline]
        fn name(&self) -> &str {
            "mod"
        }

        #[inline]
        fn precedence(&self) -> Precedence {
            Precedence::MEDIUM
        }

        #[inline]
        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        #[inline]
        fn call(&self, left: Decimal, right: Decimal) -> Result<Decimal> {
            left.checked_rem(right)
                .ok_or(Error::from(ErrorKind::Overflow))
        }
    }

    pub struct PowOperator;
    impl BinaryFunction<Decimal> for PowOperator {
        #[inline]
        fn name(&self) -> &str {
            "^"
        }

        #[inline]
        fn precedence(&self) -> Precedence {
            Precedence::HIGH
        }

        #[inline]
        fn associativity(&self) -> Associativity {
            Associativity::Right
        }

        #[inline]
        fn call(&self, left: Decimal, right: Decimal) -> Result<Decimal> {
            left.checked_pow(right)
                .ok_or(Error::from(ErrorKind::Overflow))
        }
    }

    pub struct UnaryMinus;
    impl UnaryFunction<Decimal> for UnaryMinus {
        #[inline]
        fn name(&self) -> &str {
            "-"
        }

        #[inline]
        fn notation(&self) -> Notation {
            Notation::Prefix
        }

        #[inline]
        fn call(&self, value: Decimal) -> Result<Decimal> {
            Ok(-value)
        }
    }

    pub struct Factorial;
    impl UnaryFunction<Decimal> for Factorial {
        #[inline]
        fn name(&self) -> &str {
            "!"
        }

        #[inline]
        fn notation(&self) -> Notation {
            Notation::Postfix
        }

        #[inline]
        fn call(&self, value: Decimal) -> Result<Decimal> {
            value
                .checked_factorial()
                .ok_or(Error::from(ErrorKind::Overflow))
        }
    }

    pub struct SumFunction;
    impl Function<Decimal> for SumFunction {
        fn name(&self) -> &str {
            "sum"
        }

        fn call(&self, args: &[Decimal]) -> Result<Decimal> {
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
    impl Function<Decimal> for ProdFunction {
        fn name(&self) -> &str {
            "product"
        }

        fn call(&self, args: &[Decimal]) -> Result<Decimal> {
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
    impl Function<Decimal> for AvgFunction {
        fn name(&self) -> &str {
            "avg"
        }

        fn call(&self, args: &[Decimal]) -> Result<Decimal> {
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
                Some(n) => {
                    Ok(n / Decimal::from_usize(args.len()).unwrap())
                }
                None => Err(Error::from(ErrorKind::InvalidArgumentCount)),
            }
        }
    }

    macro_rules! forward_checked_func_impl {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name{
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len(){
                        1 => args[0].$method_name().ok_or(Error::from(ErrorKind::Overflow)),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            forward_checked_func_impl!($struct_name, $method_name, $method_name)
        };
    }

    macro_rules! forward_func_impl {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name{
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len(){
                        1 => Ok(args[0].$method_name()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            forward_func_impl!($struct_name, $method_name, $method_name);
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

    pub struct SqrtFunction;
    forward_checked_func_impl!(SqrtFunction, checked_sqrt, sqrt);

    pub struct CbrtFunction;
    forward_checked_func_impl!(CbrtFunction, checked_cbrt, cbrt);

    pub struct ExpFunction;
    forward_checked_func_impl!(ExpFunction, checked_exp, exp);

    pub struct LnFunction;
    forward_checked_func_impl!(LnFunction, checked_ln, ln);

    pub struct LogFunction;
    impl Function<Decimal> for LogFunction{
        fn name(&self) -> &str {
            "log"
        }

        fn call(&self, args: &[Decimal]) -> Result<Decimal> {
            match args.len(){
                1 => args[0].checked_log(consts::TEN).ok_or(Error::from(ErrorKind::Overflow)),
                2 => args[0].checked_log(args[1]).ok_or(Error::from(ErrorKind::Overflow)),
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    //////////////////// Trigonometric ////////////////////
    macro_rules! impl_checked_trig {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            forward_checked_func_impl!($struct_name, $method_name, $name);
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_checked_trig!($struct_name, $method_name, $method_name);
        };
    }

    macro_rules! impl_checked_trig_rec {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name{
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len(){
                        1 => args[0].$method_name()
                            .map(Decimal::inv)
                            .ok_or(Error::from(ErrorKind::Overflow)),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_checked_trig_rec!($struct_name, $method_name, $method_name);
        };
    }

    macro_rules! impl_trig {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            forward_func_impl!($struct_name, $method_name, $name);
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_trig!($struct_name, $method_name, $method_name);
        };
    }

    macro_rules! impl_trig_rec {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name{
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len(){
                        1 => Ok(args[0].$method_name().inv()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_trig_rec!($struct_name, $method_name, $method_name);
        };
    }

    pub struct SinFunction;
    impl_trig!(SinFunction, sin);

    pub struct CosFunction;
    impl_trig!(CosFunction, cos);

    pub struct TanFunction;
    impl_checked_trig!(TanFunction, tan);

    pub struct CscFunction;
    impl_trig_rec!(CscFunction, sin, csc);

    pub struct SecFunction;
    impl_trig_rec!(SecFunction, cos, sec);

    pub struct CotFunction;
    impl_checked_trig_rec!(CotFunction, tan, cot);

    //////////////////// Inverse Trigonometric ////////////////////
    macro_rules! impl_arc_trig_rec {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name{
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len(){
                        1 => Ok(args[0].$method_name()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_arc_trig_rec!($struct_name, $method_name, $method_name);
        };
    }

    macro_rules! impl_checked_arc_trig_rec {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name{
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len(){
                        1 => args[0].inv()
                            .$method_name()
                            .ok_or(Error::from(ErrorKind::Overflow)),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_checked_arc_trig_rec!($struct_name, $method_name, $method_name);
        };
    }

    pub struct ASinFunction;
    impl_checked_trig!(ASinFunction, asin);

    pub struct ACosFunction;
    impl_checked_trig!(ACosFunction, acos);

    pub struct ATanFunction;
    impl Function<Decimal> for ATanFunction {
        #[inline]
        fn name(&self) -> &str {
            stringify!( atan )
        }

        #[inline]
        fn call(&self, args: &[Decimal]) -> Result<Decimal> {
            match args.len() {
                1 => Ok(args[0].atan()),
                2 => Ok(args[0].atan2(args[1])),
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
            }
        }
    }

    pub struct ACscFunction;
    impl_checked_arc_trig_rec!(ACscFunction, asin, acsc);

    pub struct ASecFunction;
    impl_checked_arc_trig_rec!(ASecFunction, acos, asec);

    pub struct ACotFunction;
    impl_arc_trig_rec!(ACotFunction, atan, acot);

    //////////////////// Hyperbolic Trigonometric ////////////////////
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

    //////////////////// Inverse Hyperbolic Trigonometric ////////////////////
    pub struct ASinhFunction;
    impl_trig!(ASinhFunction, asinh);

    pub struct ACoshFunction;
    impl_checked_trig!(ACoshFunction, acosh);

    pub struct ATanhFunction;
    impl_checked_trig!(ATanhFunction, atanh);

    pub struct ACschFunction;
    impl_arc_trig_rec!(ACschFunction, asinh, acsch);

    pub struct ASechFunction;
    impl_checked_arc_trig_rec!(ASechFunction, acosh, asech);

    pub struct ACothFunction;
    impl_checked_arc_trig_rec!(ACothFunction, atanh, acoth);
}

pub mod context{
    use crate::context::{DefaultContext, Config, Context};
    use rust_decimal::Decimal;
    use crate::decimal::ops::*;
    use crate::ops::math::{MaxFunction, MinFunction, RandFunction, UnaryPlus};
    use crate::decimal::consts;

    impl <'a> DefaultContext<'a, Decimal>{
        #[inline]
        pub fn new_decimal() -> Self{
            Self::new_decimal_with_config(Config::new())
        }

        pub fn new_decimal_with_config(config: Config) -> Self{
            let mut context = Self::empty_with_config(config);
            context.add_constant("PI", consts::PI);
            context.add_constant("E", consts::E);
            context.add_binary_function(AddOperator);
            context.add_binary_function(SubOperator);
            context.add_binary_function(MulOperator);
            context.add_binary_function(DivOperator);
            context.add_binary_function(PowOperator);
            context.add_binary_function(ModOperator);
            context.add_unary_function(UnaryPlus);
            context.add_unary_function(UnaryMinus);
            context.add_unary_function(Factorial);
            context.add_function(SumFunction);
            context.add_function(AvgFunction);
            context.add_function(ProdFunction);
            context.add_function(MaxFunction);
            context.add_function(MinFunction);
            context.add_function(SqrtFunction);
            context.add_function(LnFunction);
            context.add_function(LogFunction);
            context.add_function(RandFunction);
            context.add_function(ExpFunction);
            context.add_function(SinFunction);
            context.add_function(CosFunction);
            context.add_function(TanFunction);
            context.add_function(CscFunction);
            context.add_function(SecFunction);
            context.add_function(CotFunction);
            context.add_function(ASinFunction);
            context.add_function(ACosFunction);
            context.add_function(ATanFunction);
            context.add_function(ACscFunction);
            context.add_function(ASecFunction);
            context.add_function(ACotFunction);
            context.add_function(SinhFunction);
            context.add_function(CoshFunction);
            context.add_function(TanhFunction);
            context.add_function(CschFunction);
            context.add_function(SechFunction);
            context.add_function(CothFunction);
            context.add_function(ASinhFunction);
            context.add_function(ACoshFunction);
            context.add_function(ATanhFunction);
            context.add_function(ACschFunction);
            context.add_function(ASechFunction);
            context.add_function(ACothFunction);
            context
        }
    }
}
