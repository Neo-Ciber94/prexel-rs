pub use rust_decimal_macros::*;
pub use rust_decimal;
pub mod decimal_math;

#[macro_export]
macro_rules! decimal {
    ($val:expr) => {{
        dec!($val) as Decimal
    }};
}

pub mod consts{
    //! A set of Decimal constants.
    use rust_decimal_macros::*;
    use rust_decimal::Decimal;

    //////////////////////// Math Constants ////////////////////////
    pub const E : Decimal = dec!(2.7182818284590452353602874714);
    pub const PI : Decimal = dec!(3.1415926535897932384626433833);
    pub const PI2 : Decimal = dec!(6.2831853071795864769252867666);
    pub const PI_MINUS : Decimal = dec!(-3.1415926535897932384626433833);
    pub const PI2_MINUS : Decimal = dec!(-6.2831853071795864769252867666);
    pub const PI_HALF : Decimal = dec!(1.5707963267948966192313216916);
    pub const PI_3HALF : Decimal = dec!(4.7123889803846898576939650750);

    //////////////////////// Utils Constants ////////////////////////
    pub const TAYLOR_SERIES_ITERATIONS : u32 = 100;
    pub const EPSILON : Decimal = dec!(0.0000000000000000001);
    pub const E_INV : Decimal = dec!(0.3678794411714423215955237702);
    pub const LN2 : Decimal = dec!(0.6931471805599453094172321215);
    pub const LN10 : Decimal = dec!(2.3025850929940456840179914546844);
    pub const LN10_INV: Decimal = dec!(0.4342944819032518276511289189);
    pub const ZERO : Decimal = dec!(0);
    pub const HALF : Decimal = dec!(0.5);
    pub const THIRD : Decimal = dec!(0.33333333333333333333333333333);
    pub const ONE_MINUS : Decimal = dec!(-1);
    pub const ONE : Decimal = dec!(1);
    pub const TWO : Decimal = dec!(2);
    pub const TEN : Decimal = dec!(10);
}