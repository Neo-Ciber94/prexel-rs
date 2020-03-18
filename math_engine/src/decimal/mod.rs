pub use rust_decimal_macros::*;
pub use rust_decimal;
pub mod decimal_ex;

/// A set of Decimal constants.
pub mod consts{
    use rust_decimal_macros::*;
    use rust_decimal::Decimal;

    pub(crate) const TAYLOR_SERIES_ITERATIONS : u32 = 100;
    pub(crate) const PRECISION : u32 = 20;
    pub(crate) const EPSILON : Decimal = dec!(0.00000000000000000001);
    
    //////////////////////// Constants ////////////////////////
    /// Euler's number (e)
    pub const E : Decimal = dec!(2.7182818284590452353602874714);
    /// 1/e
    pub const E_INV : Decimal = dec!(0.3678794411714423215955237702);
    /// e²
    pub const E_POW_2: Decimal = dec!(7.3890560989306502272304274605);
    /// Archimedes' constant π
    pub const PI : Decimal = dec!(3.1415926535897932384626433833);
    /// 2π
    pub const PI_2: Decimal = dec!(6.2831853071795864769252867666);
    /// -π
    pub const PI_MINUS : Decimal = dec!(-3.1415926535897932384626433833);
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
    pub const HALF : Decimal = dec!(0.5);
    /// 1/3
    pub const ONE_FRACT_3: Decimal = dec!(0.33333333333333333333333333333);
    /// -1
    pub const ONE_MINUS : Decimal = dec!(-1);
    /// 0
    pub const ZERO : Decimal = dec!(0);
    /// 1
    pub const ONE : Decimal = dec!(1);
    /// 2
    pub const TWO : Decimal = dec!(2);
    /// 10
    pub const TEN : Decimal = dec!(10);
}