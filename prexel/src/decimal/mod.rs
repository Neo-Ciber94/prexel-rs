/// The decimal type.
pub type Decimal = rust_decimal::Decimal;

///
pub mod decimal_ext;

pub mod consts {
    use rust_decimal::Decimal;
    use rust_decimal_macros::*;

    pub(crate) const TAYLOR_SERIES_ITERATIONS: u32 = 100;
    pub(crate) const PRECISION: Decimal = dec!(0.0000000000000001);

    //////////////////////// Constants ////////////////////////
    /// Euler's number (e)
    pub const E: Decimal = Decimal::E;
    /// 1/e
    pub const E_INV: Decimal = Decimal::E_INVERSE;
    /// e²
    pub const E_POW_2: Decimal = dec!(7.3890560989306502272304274605);
    /// Archimedes' constant π
    pub const PI: Decimal = Decimal::PI;
    /// 2π
    pub const PI_2: Decimal = Decimal::TWO_PI;
    /// -π
    pub const PI_MINUS: Decimal = dec!(-3.1415926535897932384626433833);
    /// -2π
    pub const PI_2_MINUS: Decimal = dec!(-6.2831853071795864769252867666);
    /// π/2
    pub const PI_FRACT_2: Decimal = Decimal::HALF_PI;
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
    pub const LN_10: Decimal = dec!(2.3025850929940456840179914546);
    /// 1/Ln(10)
    pub const LN_10_INV: Decimal = dec!(0.4342944819032518276511289189);
    /// ✓2
    pub const SQRT_2: Decimal = dec!(1.4142135623730950488016887242);
    /// 0.5
    pub const HALF: Decimal = dec!(0.5);
    /// 1/3
    pub const ONE_FRACT_3: Decimal = dec!(0.3333333333333333333333333333);
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

#[cfg(not(feature = "docs"))]
macro_rules! forward_checked_func_impl {
    ($struct_name:ident, $method_name:ident, $name:ident) => {
        impl Function<Decimal> for $struct_name {
            #[inline]
            fn name(&self) -> &str {
                stringify!($name)
            }

            #[inline]
            fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                match args.len() {
                    1 => args[0]
                        .$method_name()
                        .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                    _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                }
            }
        }
    };

    ($struct_name:ident, $method_name:ident) => {
        forward_checked_func_impl!($struct_name, $method_name, $method_name);
    };
}

#[cfg(not(feature = "docs"))]
macro_rules! forward_checked_func_inv_impl {
    ($struct_name:ident, $method_name:ident, $name:ident) => {
        impl Function<Decimal> for $struct_name {
            #[inline]
            fn name(&self) -> &str {
                stringify!($name)
            }

            #[inline]
            fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                match args.len() {
                    1 => args[0]
                        .$method_name()
                        .map(Decimal::checked_inv)
                        .flatten()
                        .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                    _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                }
            }
        }
    };

    ($struct_name:ident, $method_name:ident) => {
        forward_checked_func_inv_impl!($struct_name, $method_name, $method_name)
    };
}

#[cfg(not(feature = "docs"))]
macro_rules! forward_func_impl {
    ($struct_name:ident, $method_name:ident, $name:ident) => {
        impl Function<Decimal> for $struct_name {
            #[inline]
            fn name(&self) -> &str {
                stringify!($name)
            }

            #[inline]
            fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                match args.len() {
                    1 => Ok(args[0].$method_name()),
                    _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                }
            }
        }
    };

    ($struct_name:ident, $method_name:ident) => {
        forward_func_impl!($struct_name, $method_name, $method_name);
    };
}

#[cfg(not(feature = "docs"))]
macro_rules! forward_func_inv_impl {
    ($struct_name:ident, $method_name:ident, $name:ident) => {
        impl Function<Decimal> for $struct_name {
            #[inline]
            fn name(&self) -> &str {
                stringify!($name)
            }

            #[inline]
            fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                match args.len() {
                    1 => Ok(args[0].$method_name().inv()),
                    _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                }
            }
        }
    };

    ($struct_name:ident, $method_name:ident) => {
        forward_func_inv_impl!($struct_name, $method_name, $method_name);
    };
}

#[cfg(feature = "docs")]
macro_rules! forward_checked_func_impl {
    ($struct_name:ident, $method_name:ident, $name:ident, $description:expr) => {
        impl Function<Decimal> for $struct_name {
            #[inline]
            fn name(&self) -> &str {
                stringify!($name)
            }

            #[inline]
            fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                match args.len() {
                    1 => args[0]
                        .$method_name()
                        .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                    _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                }
            }

            fn description(&self) -> Option<&str> {
                Some($description.into())
            }
        }
    };

    ($struct_name:ident, $method_name:ident, $description:expr) => {
        forward_checked_func_impl!($struct_name, $method_name, $method_name, $description);
    };
}

#[cfg(feature = "docs")]
macro_rules! forward_checked_func_inv_impl {
    ($struct_name:ident, $method_name:ident, $name:ident, $description:expr) => {
        impl Function<Decimal> for $struct_name {
            #[inline]
            fn name(&self) -> &str {
                stringify!($name)
            }

            #[inline]
            fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                match args.len() {
                    1 => args[0]
                        .$method_name()
                        .map(Decimal::checked_inv)
                        .flatten()
                        .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                    _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                }
            }

            fn description(&self) -> Option<&str> {
                Some($description.into())
            }
        }
    };

    ($struct_name:ident, $method_name:ident, $description:expr) => {
        forward_checked_func_inv_impl!($struct_name, $method_name, $method_name, $description);
    };
}

#[cfg(feature = "docs")]
macro_rules! forward_func_impl {
    ($struct_name:ident, $method_name:ident, $name:ident, $description:expr) => {
        impl Function<Decimal> for $struct_name {
            #[inline]
            fn name(&self) -> &str {
                stringify!($name)
            }

            #[inline]
            fn call(&self, args: &[Decimal]) -> Result<Decimal> {
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

    ($struct_name:ident, $method_name:ident, $description:expr) => {
        forward_func_impl!($struct_name, $method_name, $method_name, $description);
    };
}

#[cfg(feature = "docs")]
macro_rules! forward_func_inv_impl {
    ($struct_name:ident, $method_name:ident, $name:ident, $description:expr) => {
        impl Function<Decimal> for $struct_name {
            #[inline]
            fn name(&self) -> &str {
                stringify!($name)
            }

            #[inline]
            fn call(&self, args: &[Decimal]) -> Result<Decimal> {
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

    ($struct_name:ident, $method_name:ident, $description:expr) => {
        forward_func_inv_impl!($struct_name, $method_name, $method_name, $description);
    };
}

pub mod ops {
    pub use super::math_ops::*;
    pub use super::trig_ops::*;
}

mod math_ops {
    use crate::decimal::consts;
    use num_traits::FromPrimitive;
    use rust_decimal::Decimal;

    use crate::decimal::decimal_ext::DecimalExt;
    use crate::error::*;
    use crate::function::{
        Associativity, BinaryFunction, Function, Notation, Precedence, UnaryFunction,
    };
    use crate::Result;

    #[cfg(feature = "docs")]
    use crate::descriptions::Description;

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
                .ok_or_else(|| Error::from(ErrorKind::Overflow))
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Add.into())
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
                .ok_or_else(|| Error::from(ErrorKind::Overflow))
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Sub.into())
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
                .ok_or_else(|| Error::from(ErrorKind::Overflow))
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Mul.into())
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
                .ok_or_else(|| Error::from(ErrorKind::Overflow))
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Div.into())
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
                .ok_or_else(|| Error::from(ErrorKind::Overflow))
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Mod.into())
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
            left.checked_powd(right)
                .ok_or_else(|| Error::from(ErrorKind::Overflow))
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Pow.into())
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

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Neg.into())
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
                .ok_or_else(|| Error::from(ErrorKind::Overflow))
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Factorial.into())
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
                    None => result = Some(*cur),
                    Some(ref n) => {
                        result = Some(*n + *cur);
                    }
                }
            }

            result.ok_or_else(|| Error::from(ErrorKind::InvalidArgumentCount))
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Sum.into())
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
                    None => result = Some(*cur),
                    Some(ref n) => {
                        result = Some(n * cur);
                    }
                }
            }

            result.ok_or_else(|| Error::from(ErrorKind::InvalidArgumentCount))
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Prod.into())
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
                    None => sum = Some(*cur),
                    Some(ref n) => {
                        sum = Some(*n + *cur);
                    }
                }
            }

            match sum {
                Some(n) => Ok(n / Decimal::from_usize(args.len()).unwrap()),
                None => Err(Error::from(ErrorKind::InvalidArgumentCount)),
            }
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Avg.into())
        }
    }

    pub struct FloorFunction;
    #[cfg(not(feature = "docs"))]
    forward_func_impl!(FloorFunction, floor);
    #[cfg(feature = "docs")]
    forward_func_impl!(FloorFunction, floor, Description::Floor);

    pub struct CeilFunction;
    #[cfg(not(feature = "docs"))]
    forward_func_impl!(CeilFunction, ceil);
    #[cfg(feature = "docs")]
    forward_func_impl!(CeilFunction, ceil, Description::Ceil);

    pub struct TruncateFunction;
    #[cfg(not(feature = "docs"))]
    forward_func_impl!(TruncateFunction, trunc, truncate);
    #[cfg(feature = "docs")]
    forward_func_impl!(TruncateFunction, trunc, truncate, Description::Truncate);

    pub struct RoundFunction;
    #[cfg(not(feature = "docs"))]
    forward_func_impl!(RoundFunction, round);
    #[cfg(feature = "docs")]
    forward_func_impl!(RoundFunction, round, Description::Round);

    pub struct SqrtFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_impl!(SqrtFunction, checked_sqrt, sqrt);
    #[cfg(feature = "docs")]
    forward_checked_func_impl!(SqrtFunction, checked_sqrt, sqrt, Description::Sqrt);

    pub struct CbrtFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_impl!(CbrtFunction, checked_cbrt, cbrt);
    #[cfg(feature = "docs")]
    forward_checked_func_impl!(CbrtFunction, checked_cbrt, cbrt, Description::Cbrt);

    pub struct ExpFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_impl!(ExpFunction, checked_exp, exp);
    #[cfg(feature = "docs")]
    forward_checked_func_impl!(ExpFunction, checked_exp, exp, Description::Exp);

    pub struct LnFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_impl!(LnFunction, checked_ln, ln);
    #[cfg(feature = "docs")]
    forward_checked_func_impl!(LnFunction, checked_ln, ln, Description::Ln);

    pub struct LogFunction;
    impl Function<Decimal> for LogFunction {
        fn name(&self) -> &str {
            "log"
        }

        fn call(&self, args: &[Decimal]) -> Result<Decimal> {
            match args.len() {
                1 => args[0]
                    .checked_log10(consts::TEN)
                    .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                2 => args[0]
                    .checked_log10(args[1])
                    .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
            }
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::Log.into())
        }
    }

    pub struct ToRadiansFunction;
    #[cfg(not(feature = "docs"))]
    forward_func_impl!(ToRadiansFunction, to_radians, to_radians);
    #[cfg(feature = "docs")]
    forward_func_impl!(ToRadiansFunction, to_radians, to_radians, Description::ToRadians);

    pub struct ToDegreesFunction;
    #[cfg(not(feature = "docs"))]
    forward_func_impl!(ToDegreesFunction, to_degrees, to_degrees);
    #[cfg(feature = "docs")]
    forward_func_impl!(ToDegreesFunction, to_degrees, to_degrees, Description::ToDegrees);
}

mod trig_ops {
    use crate::decimal::decimal_ext::DecimalExt;
    use crate::error::*;
    use crate::function::Function;
    use crate::Result;
    use rust_decimal::Decimal;

    #[cfg(feature = "docs")]
    use crate::descriptions::Description;

    #[cfg(not(feature = "docs"))]
    macro_rules! impl_checked_trig {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name {
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len() {
                        1 => args[0]
                            .to_radians()
                            .$method_name()
                            .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_checked_trig!($struct_name, $method_name, $method_name);
        };
    }

    #[cfg(not(feature = "docs"))]
    macro_rules! impl_checked_trig_rec {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name {
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len() {
                        1 => args[0]
                            .to_radians()
                            .$method_name()
                            //.map(Decimal::checked_inv)
                            .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_checked_trig_rec!($struct_name, $method_name, $method_name);
        };
    }

    #[cfg(not(feature = "docs"))]
    macro_rules! impl_trig {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name {
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len() {
                        1 => Ok(args[0].to_radians().$method_name()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_trig!($struct_name, $method_name, $method_name);
        };
    }

    #[cfg(not(feature = "docs"))]
    macro_rules! impl_trig_rec {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name {
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len() {
                        1 => Ok(args[0].to_radians().$method_name().inv()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_trig_rec!($struct_name, $method_name, $method_name);
        };
    }

    #[cfg(feature = "docs")]
    macro_rules! impl_checked_trig {
        ($struct_name:ident, $method_name:ident, $name:ident, $description:expr) => {
            impl Function<Decimal> for $struct_name {
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len() {
                        1 => args[0]
                            .to_radians()
                            .$method_name()
                            .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }

                fn description(&self) -> Option<&str> {
                    Some($description.into())
                }
            }
        };

        ($struct_name:ident, $method_name:ident, $description:expr) => {
            impl_checked_trig!($struct_name, $method_name, $method_name, $description);
        };
    }

    #[cfg(feature = "docs")]
    macro_rules! impl_checked_trig_rec {
        ($struct_name:ident, $method_name:ident, $name:ident, $description:expr) => {
            impl Function<Decimal> for $struct_name {
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len() {
                        1 => args[0]
                            .to_radians()
                            .$method_name()
                            //.map(Decimal::checked_inv)
                            .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }

                fn description(&self) -> Option<&str> {
                    Some($description.into())
                }
            }
        };

        ($struct_name:ident, $method_name:ident, $description:expr) => {
            impl_checked_trig_rec!($struct_name, $method_name, $method_name, $description);
        };
    }

    // #[cfg(feature = "docs")]
    // macro_rules! impl_trig {
    //     ($struct_name:ident, $method_name:ident, $name:ident, $description:expr) => {
    //         impl Function<Decimal> for $struct_name {
    //             #[inline]
    //             fn name(&self) -> &str {
    //                 stringify!($name)
    //             }
    //
    //             #[inline]
    //             fn call(&self, args: &[Decimal]) -> Result<Decimal> {
    //                 match args.len() {
    //                     1 => Ok(args[0].to_radians().$method_name()),
    //                     _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
    //                 }
    //             }
    //
    //             fn description(&self) -> Option<&str> {
    //                 Some($description.into())
    //             }
    //         }
    //     };
    //
    //     ($struct_name:ident, $method_name:ident, $description:expr) => {
    //         impl_trig!($struct_name, $method_name, $method_name, $description);
    //     };
    // }
    //
    // #[cfg(feature = "docs")]
    // macro_rules! impl_trig_rec {
    //     ($struct_name:ident, $method_name:ident, $name:ident, $description:expr) => {
    //         impl Function<Decimal> for $struct_name {
    //             #[inline]
    //             fn name(&self) -> &str {
    //                 stringify!($name)
    //             }
    //
    //             #[inline]
    //             fn call(&self, args: &[Decimal]) -> Result<Decimal> {
    //                 match args.len() {
    //                     1 => Ok(args[0].to_radians().$method_name().inv()),
    //                     _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
    //                 }
    //             }
    //
    //             fn description(&self) -> Option<&str> {
    //                 Some($description.into())
    //             }
    //         }
    //     };
    //
    //     ($struct_name:ident, $method_name:ident, $description:expr) => {
    //         impl_trig_rec!($struct_name, $method_name, $method_name, $description);
    //     };
    // }

    pub struct SinFunction;
    #[cfg(not(feature = "docs"))]
    impl_checked_trig!(SinFunction, checked_sin);
    #[cfg(feature = "docs")]
    impl_checked_trig!(SinFunction, checked_sin, Description::Sin);

    pub struct CosFunction;
    #[cfg(not(feature = "docs"))]
    impl_checked_trig!(CosFunction, checked_cos);
    #[cfg(feature = "docs")]
    impl_checked_trig!(CosFunction, checked_cos, Description::Cos);

    pub struct TanFunction;
    #[cfg(not(feature = "docs"))]
    impl_checked_trig!(TanFunction, checked_tan);
    #[cfg(feature = "docs")]
    impl_checked_trig!(TanFunction, checked_tan, Description::Tan);

    pub struct CscFunction;
    #[cfg(not(feature = "docs"))]
    impl_checked_trig_rec!(CscFunction, checked_sin, csc);
    #[cfg(feature = "docs")]
    impl_checked_trig_rec!(CscFunction, checked_sin, csc, Description::Csc);

    pub struct SecFunction;
    #[cfg(not(feature = "docs"))]
    impl_checked_trig_rec!(SecFunction, checked_cos, sec);
    #[cfg(feature = "docs")]
    impl_checked_trig_rec!(SecFunction, checked_cos, sec, Description::Sec);

    pub struct CotFunction;
    #[cfg(not(feature = "docs"))]
    impl_checked_trig_rec!(CotFunction, checked_tan, cot);
    #[cfg(feature = "docs")]
    impl_checked_trig_rec!(CotFunction, checked_tan, cot, Description::Cot);

    //////////////////// Inverse Trigonometric ////////////////////
    #[cfg(not(feature = "docs"))]
    macro_rules! impl_arc_trig_rec {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name {
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len() {
                        1 => Ok(args[0].$method_name()),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_arc_trig_rec!($struct_name, $method_name, $method_name);
        };
    }

    #[cfg(not(feature = "docs"))]
    macro_rules! impl_checked_arc_trig_rec {
        ($struct_name:ident, $method_name:ident, $name:ident) => {
            impl Function<Decimal> for $struct_name {
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len() {
                        1 => args[0]
                            .inv()
                            .$method_name()
                            .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }
            }
        };

        ($struct_name:ident, $method_name:ident) => {
            impl_checked_arc_trig_rec!($struct_name, $method_name, $method_name);
        };
    }

    #[cfg(feature = "docs")]
    macro_rules! impl_arc_trig_rec {
        ($struct_name:ident, $method_name:ident, $name:ident, $description:expr) => {
            impl Function<Decimal> for $struct_name {
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
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

        ($struct_name:ident, $method_name:ident, $description:expr) => {
            impl_arc_trig_rec!($struct_name, $method_name, $method_name, $description);
        };
    }

    #[cfg(feature = "docs")]
    macro_rules! impl_checked_arc_trig_rec {
        ($struct_name:ident, $method_name:ident, $name:ident, $description:expr) => {
            impl Function<Decimal> for $struct_name {
                #[inline]
                fn name(&self) -> &str {
                    stringify!($name)
                }

                #[inline]
                fn call(&self, args: &[Decimal]) -> Result<Decimal> {
                    match args.len() {
                        1 => args[0]
                            .inv()
                            .$method_name()
                            .ok_or_else(|| Error::from(ErrorKind::Overflow)),
                        _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
                    }
                }

                fn description(&self) -> Option<&str> {
                    Some($description.into())
                }
            }
        };

        ($struct_name:ident, $method_name:ident, $description:expr) => {
            impl_checked_arc_trig_rec!($struct_name, $method_name, $method_name, $description);
        };
    }

    pub struct ASinFunction;
    #[cfg(not(feature = "docs"))]
    impl_checked_trig!(ASinFunction, asin);
    #[cfg(feature = "docs")]
    impl_checked_trig!(ASinFunction, asin, Description::ASin);

    pub struct ACosFunction;
    #[cfg(not(feature = "docs"))]
    impl_checked_trig!(ACosFunction, acos);
    #[cfg(feature = "docs")]
    impl_checked_trig!(ACosFunction, acos, Description::ACos);

    pub struct ATanFunction;
    impl Function<Decimal> for ATanFunction {
        #[inline]
        fn name(&self) -> &str {
            stringify!(atan)
        }

        #[inline]
        fn call(&self, args: &[Decimal]) -> Result<Decimal> {
            match args.len() {
                1 => Ok(args[0].atan()),
                2 => Ok(args[0].atan2(args[1])),
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
            }
        }

        #[cfg(feature = "docs")]
        fn description(&self) -> Option<&str> {
            Some(Description::ATan.into())
        }
    }

    pub struct ACscFunction;
    #[cfg(not(feature = "docs"))]
    impl_checked_arc_trig_rec!(ACscFunction, asin, acsc);
    #[cfg(feature = "docs")]
    impl_checked_arc_trig_rec!(ACscFunction, asin, acsc, Description::ACsc);

    pub struct ASecFunction;
    #[cfg(not(feature = "docs"))]
    impl_checked_arc_trig_rec!(ASecFunction, acos, asec);
    #[cfg(feature = "docs")]
    impl_checked_arc_trig_rec!(ASecFunction, acos, asec, Description::ASec);

    pub struct ACotFunction;
    #[cfg(not(feature = "docs"))]
    impl_arc_trig_rec!(ACotFunction, atan, acot);
    #[cfg(feature = "docs")]
    impl_arc_trig_rec!(ACotFunction, atan, acot, Description::ACot);

    //////////////////// Hyperbolic Trigonometric ////////////////////
    pub struct SinhFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_impl!(SinhFunction, sinh);
    #[cfg(feature = "docs")]
    forward_checked_func_impl!(SinhFunction, sinh, Description::Sinh);

    pub struct CoshFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_impl!(CoshFunction, cosh);
    #[cfg(feature = "docs")]
    forward_checked_func_impl!(CoshFunction, cosh, Description::Cosh);

    pub struct TanhFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_impl!(TanhFunction, tanh);
    #[cfg(feature = "docs")]
    forward_checked_func_impl!(TanhFunction, tanh, Description::Tanh);

    pub struct CschFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_inv_impl!(CschFunction, sinh, csch);
    #[cfg(feature = "docs")]
    forward_checked_func_inv_impl!(CschFunction, sinh, csch, Description::Csch);

    pub struct SechFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_inv_impl!(SechFunction, cosh, sech);
    #[cfg(feature = "docs")]
    forward_checked_func_inv_impl!(SechFunction, cosh, sech, Description::Sech);

    pub struct CothFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_inv_impl!(CothFunction, tanh, coth);
    #[cfg(feature = "docs")]
    forward_checked_func_inv_impl!(CothFunction, tanh, coth, Description::Coth);

    //////////////////// Inverse Hyperbolic Trigonometric ////////////////////
    pub struct ASinhFunction;
    #[cfg(not(feature = "docs"))]
    forward_func_impl!(ASinhFunction, asinh);
    #[cfg(feature = "docs")]
    forward_func_impl!(ASinhFunction, asinh, Description::ASinh);

    pub struct ACoshFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_impl!(ACoshFunction, acosh);
    #[cfg(feature = "docs")]
    forward_checked_func_impl!(ACoshFunction, acosh, Description::ACosh);

    pub struct ATanhFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_impl!(ATanhFunction, atanh);
    #[cfg(feature = "docs")]
    forward_checked_func_impl!(ATanhFunction, atanh, Description::ATanh);

    pub struct ACschFunction;
    #[cfg(not(feature = "docs"))]
    forward_func_inv_impl!(ACschFunction, asinh, acsch);
    #[cfg(feature = "docs")]
    forward_func_inv_impl!(ACschFunction, asinh, acsch, Description::ACsch);

    pub struct ASechFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_inv_impl!(ASechFunction, acosh, asech);
    #[cfg(feature = "docs")]
    forward_checked_func_inv_impl!(ASechFunction, acosh, asech, Description::ASech);

    pub struct ACothFunction;
    #[cfg(not(feature = "docs"))]
    forward_checked_func_inv_impl!(ACothFunction, atanh, acoth);
    #[cfg(feature = "docs")]
    forward_checked_func_inv_impl!(ACothFunction, atanh, acoth, Description::ACoth);
}

pub mod context {
    use crate::context::{Config, Context, DefaultContext};
    use crate::decimal::consts;
    use crate::decimal::ops::*;
    use crate::ops::math::{MaxFunction, MinFunction, RandFunction, UnaryPlus};
    use rust_decimal::Decimal;

    impl<'a> DefaultContext<'a, Decimal> {
        #[inline]
        pub fn new_decimal() -> Self {
            Self::with_config_decimal(Config::new())
        }

        pub fn with_config_decimal(config: Config) -> Self {
            let mut context = Self::with_config(config);
            context.add_constant("PI", consts::PI).unwrap();
            context.add_constant("E", consts::E).unwrap();
            context.add_binary_function(AddOperator).unwrap();
            context.add_binary_function(SubOperator).unwrap();
            context.add_binary_function(MulOperator).unwrap();
            context.add_binary_function(DivOperator).unwrap();
            context.add_binary_function(PowOperator).unwrap();
            context.add_binary_function(ModOperator).unwrap();
            context.add_unary_function(UnaryPlus).unwrap();
            context.add_unary_function(UnaryMinus).unwrap();
            context.add_unary_function(Factorial).unwrap();
            context.add_function(SumFunction).unwrap();
            context.add_function(AvgFunction).unwrap();
            context.add_function(ProdFunction).unwrap();
            context.add_function(MaxFunction).unwrap();
            context.add_function(MinFunction).unwrap();
            context.add_function(CbrtFunction).unwrap();
            context.add_function(SqrtFunction).unwrap();
            context.add_function(LnFunction).unwrap();
            context.add_function(LogFunction).unwrap();
            context.add_function(RandFunction).unwrap();
            context.add_function(CeilFunction).unwrap();
            context.add_function(FloorFunction).unwrap();
            context.add_function(TruncateFunction).unwrap();
            context.add_function(RoundFunction).unwrap();
            context.add_function(ExpFunction).unwrap();
            context.add_function(ToRadiansFunction).unwrap();
            context.add_function(ToDegreesFunction).unwrap();
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

#[cfg(test)]
mod tests {
    use crate::context::DefaultContext;
    use crate::evaluator::Evaluator;
    use super::*;

    #[test]
    fn compile_test() {
        let context = DefaultContext::new_decimal();
        let evaluator = Evaluator::with_context(context);
        let expr = "(1 + 2) * 3";
        let result = evaluator.eval(expr).unwrap();
        assert_eq!(result, Decimal::from(9));
    }
}