use crate::decimal::consts;
use crate::utils::approx::{Approx, ApproxEq};
use rust_decimal::prelude::{FromPrimitive, One, ToPrimitive, Zero};
use rust_decimal::Decimal;
use rust_decimal_macros::*;
use std::ops::Neg;

/// Extended methods for `Decimal`.
pub trait DecimalExt {
    fn is_integer(&self) -> bool;
    fn to_radians(&self) -> Decimal;
    fn to_degrees(&self) -> Decimal;
    fn inv(self) -> Decimal;
    fn checked_inv(self) -> Option<Decimal>;
    fn checked_powd(self, exponent: Decimal) -> Option<Decimal>;
    fn checked_powi(self, exponent: i64) -> Option<Decimal>;
    fn checked_sqrt(self) -> Option<Decimal>;
    fn checked_cbrt(self) -> Option<Decimal>;
    fn checked_log(self, exponent: Decimal) -> Option<Decimal>;
    fn checked_ln(self) -> Option<Decimal>;
    fn checked_exp(self) -> Option<Decimal>;
    fn checked_factorial(self) -> Option<Decimal>;
    fn checked_sin(self) -> Option<Decimal>;
    fn checked_cos(self) -> Option<Decimal>;
    fn checked_tan(self) -> Option<Decimal>;
    fn asin(self) -> Option<Decimal>;
    fn acos(self) -> Option<Decimal>;
    fn atan(self) -> Decimal;
    fn atan2(self, other: Decimal) -> Decimal;
    fn sinh(self) -> Option<Decimal>;
    fn cosh(self) -> Option<Decimal>;
    fn tanh(self) -> Option<Decimal>;
    fn asinh(self) -> Decimal;
    fn acosh(self) -> Option<Decimal>;
    fn atanh(self) -> Option<Decimal>;
}

impl DecimalExt for Decimal {
    #[inline]
    fn is_integer(&self) -> bool {
        self.fract().is_zero()
    }

    #[inline]
    fn to_radians(&self) -> Decimal {
        (self * (consts::PI / dec!(180))).approx()
    }

    #[inline]
    fn to_degrees(&self) -> Decimal {
        (self * (dec!(180) / consts::PI)).approx()
    }

    #[inline]
    fn inv(self) -> Decimal {
        consts::ONE / self
    }

    #[inline]
    fn checked_inv(self) -> Option<Decimal> {
        if self.is_zero() {
            None
        } else {
            Some(self.inv())
        }
    }

    fn checked_powd(self, exponent: Decimal) -> Option<Decimal> {
        if exponent.is_integer() {
            return self.checked_powi(exponent.to_i64().unwrap());
        }

        if self.is_zero() {
            return Some(Decimal::one());
        }

        if self.is_one() {
            return Some(self);
        }

        // If base is 'e', return e^x
        if self == consts::E {
            return exponent.checked_exp();
        }

        // x ^ n = e^(n * ln(x))
        let b = self.checked_ln()?.checked_mul(exponent)?;
        let result = b.checked_exp()?;
        Some(result)
    }

    fn checked_powi(self, mut exponent: i64) -> Option<Decimal> {
        if exponent == 0 {
            return Some(Decimal::one());
        }

        if exponent == 1 {
            return Some(self);
        }

        if exponent < 0 {
            return Self::checked_powi(Decimal::one() / self, -exponent);
        }

        let mut result = Decimal::one();
        let mut base = self;

        while exponent > 0 {
            // If exponent is odd
            if exponent & 1 != 0 {
                result = result.checked_mul(base)?;
                exponent -= 1
            }

            base = base.checked_mul(base)?;
            exponent >>= 1;
        }

        Some(result)
    }

    fn checked_sqrt(self) -> Option<Decimal> {
        if self.is_sign_negative() {
            return None;
        }

        // Using Babylonian Method
        // See: https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method

        // Initial approximation
        let mut result = Decimal::from_f64(self.to_f64()?.sqrt())?;
        let mut x = Decimal::zero();

        while x != result {
            x = result;

            // result = 0.5 * ( value / x + result )
            // -> where value = self
            let xr = self.checked_div(result)?;
            let xr1 = xr.checked_add(result)?;
            result = consts::HALF.checked_mul(xr1)?;
        }

        Some(result)
    }

    fn checked_cbrt(self) -> Option<Decimal> {
        if self.is_one() {
            return Some(self);
        }

        let mut x = Decimal::one();

        // Using Newton's Method
        // See: https://en.wikipedia.org/wiki/Cube_root#Numerical_methods
        for _ in 0..consts::TAYLOR_SERIES_ITERATIONS {
            let xx = x.checked_mul(x)?;
            let x2 = consts::TWO.checked_mul(x)?;
            x = self
                .checked_div(xx)?
                .checked_add(x2)?
                .checked_mul(consts::ONE_FRACT_3)?;
        }

        Some(x)
    }

    fn checked_log(self, base: Decimal) -> Option<Decimal> {
        let a = Self::checked_ln(self)?;
        if base == consts::TEN {
            return a.checked_div(consts::LN_10);
        }

        let b = Self::checked_ln(base)?;
        let result = a.checked_div(b)?;
        Some(result)
    }

    fn checked_ln(self) -> Option<Decimal> {
        if self <= Decimal::zero() {
            return None;
        }

        if self.is_one() {
            return Some(Decimal::zero());
        }

        // See: https://en.wikipedia.org/wiki/Natural_logarithm#Numerical_value
        if self >= Decimal::one() {
            let mut n = 0u32;
            let mut a = self;

            while a > Decimal::one() {
                a = a.checked_div(consts::TEN)?;
                n += 1;
            }

            // ln(x) = log(a * 10^n) = ln(a) + n * log(10)
            // B = n * log(10)
            let lna = Self::checked_ln(a)?;
            let b = consts::LN_10.checked_mul(n.into())?;
            let result = lna.checked_add(b)?;
            return Some(result);
        }

        // See: https://en.wikipedia.org/wiki/Logarithm#Power_series
        // Error: ~0.0000000000000000000000000007
        const ITERATIONS: u32 = consts::TAYLOR_SERIES_ITERATIONS * 10;
        let mut result = Decimal::zero();

        for n in 1..ITERATIONS {
            let sign = Self::checked_powi(consts::ONE_MINUS, (n + 1).into())?;
            let x = self.checked_sub(Decimal::one())?;
            let xn = Self::checked_powi(x, n.into())?;
            let div = xn.checked_div(n.into())?;
            let y = div.checked_mul(sign)?;

            // result += [(-1)^(n + 1)] * (value - 1)^n / n
            result = result.checked_add(y)?;
        }

        Some(result.approx())
    }

    #[allow(clippy::assertions_on_constants)]
    fn checked_exp(self) -> Option<Decimal> {
        if self.is_zero() {
            return Some(Decimal::one());
        }

        if self.is_one() {
            return Some(consts::E);
        }

        debug_assert!(consts::TAYLOR_SERIES_ITERATIONS > 0);

        // Using Continued fraction
        // https://en.wikipedia.org/wiki/Exponential_function#Continued_fractions_for_ex
        let a0 = self.checked_mul(self)?;
        let mut b0 = 4 * consts::TAYLOR_SERIES_ITERATIONS - 2;
        let mut result = (4 * consts::TAYLOR_SERIES_ITERATIONS + 2).into();

        while b0 > 2 {
            let div = a0.checked_div(result)?;
            result = Decimal::checked_add(b0.into(), div)?;
            b0 -= 4;
        }

        if b0 == 2 {
            let sum = consts::TWO.checked_sub(self)?;
            let div = a0.checked_div(result)?;
            result = sum.checked_add(div)?;
        }

        let xx = consts::TWO.checked_mul(self)?;
        let div = xx.checked_div(result)?;
        result = consts::ONE.checked_add(div)?;

        Some(result.approx())
    }

    fn checked_factorial(self) -> Option<Decimal> {
        // To reduce errors
        const MAX_DECIMAL_PLACES: u32 = 15;

        if self.is_sign_negative() {
            return None;
        }

        if self == consts::TWO || self == consts::ONE || self == consts::ZERO {
            return Some(Decimal::one());
        }

        if self < Decimal::one() {
            return gamma(self + consts::ONE).map(|d| d.round_dp(MAX_DECIMAL_PLACES));
        }

        let mut result = self;
        let mut n = self - Decimal::one();

        while n > Decimal::zero() {
            result = result.checked_mul(n)?;
            n -= Decimal::one();
        }

        if !n.is_zero() {
            result *= gamma(n + consts::ONE)?;
        }

        Some(result.round_dp(MAX_DECIMAL_PLACES))
    }

    fn checked_sin(self) -> Option<Decimal> {
        let radians: Decimal = self % consts::PI_2;

        if radians == Decimal::zero()
            || ApproxEq::approx_eq(&radians.abs(), &consts::PI, &consts::PRECISION)
            || ApproxEq::approx_eq(&radians.abs(), &consts::PI_2, &consts::PRECISION)
        {
            return Some(Decimal::zero());
        }

        if ApproxEq::approx_eq(&radians, &consts::PI_FRACT_2, &consts::PRECISION) {
            return Some(consts::ONE);
        }

        if ApproxEq::approx_eq(&radians, &consts::PI_3_FRACT_2, &consts::PRECISION)
            || ApproxEq::approx_eq(&radians.abs(), &consts::PI_FRACT_2, &consts::PRECISION)
        {
            return Some(consts::ONE_MINUS);
        }

        // Using Taylor Series
        // See: https://en.wikipedia.org/wiki/Taylor_series#Trigonometric_functions

        let xx: Decimal = radians * radians;
        let mut factor = radians;
        let mut result = radians;

        for n in 1..consts::TAYLOR_SERIES_ITERATIONS {
            factor *= -xx / Decimal::from_u32((2 * n + 1) * (2 * n))?;
            result += factor;
        }

        Some(result.approx())
    }

    fn checked_cos(self) -> Option<Decimal> {
        let radians: Decimal = self % consts::PI_2;

        if radians.is_zero() {
            return Some(Decimal::one());
        }

        if ApproxEq::approx_eq(&radians, &consts::PI_FRACT_2, &consts::PRECISION) {
            return Some(Decimal::zero());
        }

        // Using Taylor Series
        // See: https://en.wikipedia.org/wiki/Taylor_series#Trigonometric_functions
        let xx: Decimal = radians * radians;
        let mut factor: Decimal = -xx / consts::TWO;
        let mut result = Decimal::one() + factor;

        for n in 2..consts::TAYLOR_SERIES_ITERATIONS {
            factor *= -xx / Decimal::from_u32(2 * n * (2 * n - 1))?;
            result += factor;
        }

        Some(result.approx())
    }

    fn checked_tan(self) -> Option<Decimal> {
        let cos = self.checked_cos()?;
        if cos.is_zero() {
            None
        } else {
            let result = self.checked_sin()? / cos;
            Some(result.approx())
        }
    }

    fn asin(self) -> Option<Decimal> {
        if self < consts::ONE_MINUS || self > consts::ONE {
            None
        } else {
            if self.is_zero() {
                return Some(consts::ZERO);
            }

            if self.is_one() {
                return Some(consts::PI_FRACT_2);
            }

            if self == consts::ONE_MINUS {
                return Some(-consts::PI_FRACT_2);
            }

            let xx = self * self;
            let a0 = Decimal::checked_sqrt(consts::ONE - xx)?;
            let b0 = a0 + consts::ONE;
            let result: Decimal = consts::TWO * Decimal::atan(self / b0);
            Some(result.approx())
        }
    }

    fn acos(self) -> Option<Decimal> {
        if self < consts::ONE_MINUS || self > consts::ONE {
            None
        } else {
            if self.is_zero() {
                return Some(consts::PI_FRACT_2);
            }

            if self.is_one() {
                return Some(consts::ZERO);
            }

            if self == consts::ONE_MINUS {
                return Some(consts::PI);
            }

            let xx = self * self;
            let a0 = Decimal::checked_sqrt(consts::ONE - xx)?;
            let b0 = consts::ONE + self;
            let result = consts::TWO * Decimal::atan(a0 / b0);
            Some(result.approx())
        }
    }

    fn atan(self) -> Decimal {
        if self.is_zero() {
            return Decimal::zero();
        }

        if self.is_one() {
            return consts::PI_FRACT_4;
        }

        if self == consts::ONE_MINUS {
            return -consts::PI_FRACT_4;
        }

        if self < consts::ONE_MINUS {
            return -consts::PI_FRACT_2 - Decimal::atan(consts::ONE / self);
        }

        if self > consts::ONE {
            return consts::PI_FRACT_2 - Decimal::atan(consts::ONE / self);
        }

        // Using continued fractions
        // https://en.wikipedia.org/wiki/Inverse_trigonometric_functions#Continued_fractions_for_arctangent
        let mut i = consts::TAYLOR_SERIES_ITERATIONS;
        let mut result: Decimal = consts::TWO * Decimal::from_u32(i).unwrap() - consts::ONE;

        while i > 1 {
            let n = Decimal::from_u32(i).unwrap();
            let z = self * Decimal::from_u32(i - 1).unwrap();
            let z2 = z * z;
            let div = z2 / result;
            result = div + (consts::TWO * n - consts::THREE);
            i -= 1;
        }

        result = self / result;
        result.approx()
    }

    fn atan2(self, x: Decimal) -> Decimal {
        if self.is_zero() && x.is_zero() {
            return Decimal::zero();
        }

        if x.is_zero() {
            return if self > consts::ZERO {
                consts::PI_2
            } else {
                consts::PI_2_MINUS
            };
        }

        // Θ = arctan(y / x)
        let atan2 = Decimal::atan(self / x);

        if x > consts::ZERO {
            atan2
        } else if self >= consts::ZERO {
            atan2 + consts::PI
        } else {
            atan2 - consts::PI
        }
    }

    fn sinh(self) -> Option<Decimal> {
        // formula: sinh(x) = (e^x - e^-x)/2
        let e0 = self.checked_exp()?;
        let e1 = self.neg().checked_exp()?;
        let result = (e0 - e1) / consts::TWO;
        Some(result)
    }

    fn cosh(self) -> Option<Decimal> {
        // formula: cosh(x) = (e^x + e^-x)/2
        let e0 = self.checked_exp()?;
        let e1 = self.neg().checked_exp()?;
        let result = (e0 + e1) / consts::TWO;
        Some(result)
    }

    fn tanh(self) -> Option<Decimal> {
        let result = self.sinh()? / self.cosh()?;
        Some(result)
    }

    fn asinh(self) -> Decimal {
        // formula: asinh(x) = ln(x + sqrt(x^2 + 1))
        let x2 = self * self;
        Decimal::checked_ln(self + Decimal::checked_sqrt(x2 + consts::ONE).unwrap()).unwrap()
    }

    fn acosh(self) -> Option<Decimal> {
        if self < consts::ONE {
            return None;
        }

        // formula: acosh(x) = ln(x + sqrt(x^2 - 1))
        let x2 = self * self;
        Decimal::checked_ln(self + Decimal::checked_sqrt(x2 - consts::ONE)?)
    }

    fn atanh(self) -> Option<Decimal> {
        if self < consts::ONE_MINUS || self > consts::ONE {
            return None;
        }

        // formula: atanh(x) = 0.5 * ln((1 + x)/(1 - x))
        let x0 = consts::ONE + self;
        let x1 = consts::ONE - self;
        Decimal::checked_ln(x0 / x1)?.checked_mul(consts::HALF)
    }
}

impl ApproxEq for Decimal {
    #[inline]
    fn approx_eq(&self, other: &Self, delta: &Self) -> bool {
        (self - other).abs() < *delta
    }
}

impl Approx for Decimal {
    #[inline]
    fn approx(&self) -> Self {
        Self::approx_by(self, &consts::PRECISION)
    }

    fn approx_by(&self, delta: &Self) -> Self {
        let r = self.round();
        if self.approx_eq(&r, delta) {
            r
        } else {
            *self
        }
    }
}

fn gamma(mut x: Decimal) -> Option<Decimal> {
    //Using Coefficients from: https://mrob.com/pub/ries/lanczos-gamma.html
    const G: Decimal = dec!(4.7421875);
    const P: [Decimal; 15] = [
        dec!(0.99999999999999709182),
        dec!(57.156235665862923517),
        dec!(-59.597960355475491248),
        dec!(14.136097974741747174),
        dec!(-0.49191381609762019978),
        dec!(0.000033994649984811888699),
        dec!(0.000046523628927048575665),
        dec!(-0.000098374475304879564677),
        dec!(0.00015808870322491248884),
        dec!(-0.00021026444172410488319),
        dec!(0.0002174396181152126432),
        dec!(-0.00016431810653676389022),
        dec!(0.000084418223983852743293),
        dec!(-0.00002619083840158140867),
        dec!(0.0000036899182659531622704),
    ];

    // Using Lanczos approximation
    // ~10^-13 precision
    // See: https://en.wikipedia.org/wiki/Lanczos_approximation
    if x < consts::HALF {
        Some(consts::PI / Decimal::checked_sin(consts::PI * x)? * gamma(Decimal::one() - x)?)
    } else {
        // Lanczos solve gamma for (x + 1)
        x -= Decimal::one();

        let mut factor: Decimal = P[0];
        for (n, coefficient) in P.iter().enumerate().skip(1) {
            factor += coefficient / (x + Decimal::from_usize(n).unwrap());
        }

        let t: Decimal = x + G + consts::HALF;
        let result = Decimal::checked_sqrt(consts::TWO * consts::PI)?
            * Decimal::checked_powd(t, x + consts::HALF)?
            * Decimal::checked_exp(-t)?
            * factor;

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Equality using an epsilon value
    #[allow(unused)]
    macro_rules! assert_approx_eq {
        ($left:expr, $right:expr, $delta:expr) => {{
            match (&$left, &$right, &$delta) {
                (left_val, right_val, delta_val) => {
                    if !((left_val - right_val).abs() < *delta_val) {
                        panic!(
                            r#"assertion failed: `(left == right) with error of: +/-{}`
  left: `{:?}`,
 right: `{:?}`"#,
                            &*delta_val, &*left_val, &*right_val
                        )
                    }
                }
            }
        }};

        ($left:expr, $right:expr) => {{
            assert_approx_eq2!($left, $right, consts::PRECISION)
        }};
    }

    macro_rules! assert_almost_eq {
        ($left:expr, $right:expr $(,)?) => {{
            assert_almost_eq_by!($left, $right, 20);
        }};
    }

    macro_rules! assert_almost_eq_by {
        ($left:expr, $right:expr, $dp:expr $(,)?) => {{
            match (&$left, &$right, $dp) {
                (left_val, right_val, dp) => {
                    let a = left_val.round_dp(dp);
                    let b = right_val.round_dp(dp);
                    assert_eq!(a, b);
                }
            }
        }};
    }

    macro_rules! decimal {
        ($value:literal) => {
            dec!($value) as Decimal
        };
    }

    #[test]
    fn is_integer_test() {
        assert!(decimal!(10).is_integer());
        assert!(decimal!(-20).is_integer());
        assert!(decimal!(5.0).is_integer());

        assert!(!decimal!(15.5).is_integer());
        assert!(!decimal!(-20.2).is_integer());
        assert!(!decimal!(5.2).is_integer());
    }

    #[test]
    fn checked_sqrt_test() {
        assert_almost_eq!(decimal!(25).checked_sqrt().unwrap(), decimal!(5));
        assert_almost_eq!(
            decimal!(2).checked_sqrt().unwrap(),
            decimal!(1.41421356237309504880),
        );
    }

    #[test]
    fn checked_cbrt_test() {
        assert_almost_eq!(
            decimal!(10).checked_cbrt().unwrap(),
            decimal!(2.1544346900318837217592935665),
        );
        assert_almost_eq!(
            decimal!(-5).checked_cbrt().unwrap(),
            decimal!(-1.7099759466766969893531088725),
        );
    }

    #[test]
    fn checked_pow_n_test() {
        assert_almost_eq!(decimal!(5).checked_powi(0).unwrap(), decimal!(1));
        assert_almost_eq!(decimal!(8).checked_powi(1).unwrap(), decimal!(8));
        assert_almost_eq!(decimal!(0).checked_powi(0).unwrap(), decimal!(1));
        assert_almost_eq!(decimal!(2).checked_powi(3).unwrap(), decimal!(8));
        assert_almost_eq!(decimal!(5).checked_powi(-3).unwrap(), decimal!(0.008));
    }

    #[test]
    fn checked_pow_test() {
        assert_almost_eq!(
            decimal!(9).checked_powd(decimal!(0.5)).unwrap(),
            decimal!(3)
        );
        assert_almost_eq!(
            decimal!(4).checked_powd(decimal!(-0.25)).unwrap(),
            decimal!(0.70710678118654752440),
        );
    }

    #[test]
    fn checked_ln_test() {
        assert_eq!(decimal!(0).checked_ln(), None);
        assert_eq!(decimal!(-2).checked_ln(), None);

        assert_almost_eq!(
            decimal!(10).checked_ln().unwrap(),
            decimal!(2.3025850929940456840179914546),
        );
        assert_almost_eq!(
            decimal!(9).checked_ln().unwrap(),
            decimal!(2.1972245773362193827904904738),
        );
        assert_almost_eq!(
            decimal!(25).checked_ln().unwrap(),
            decimal!(3.2188758248682007492015186664),
        );
    }

    #[test]
    fn checked_exp_test() {
        assert_almost_eq!(decimal!(0).checked_exp().unwrap(), decimal!(1));
        assert_eq!(decimal!(1).checked_exp().unwrap(), consts::E);

        assert_almost_eq!(
            decimal!(3).checked_exp().unwrap(),
            decimal!(20.085536923187667740928529654),
        );
        assert_almost_eq!(
            decimal!(-4).checked_exp().unwrap(),
            decimal!(0.0183156388887341802937180212),
        );
        assert_almost_eq!(
            decimal!(0.5).checked_exp().unwrap(),
            decimal!(1.6487212707001281468486507878),
        );
    }

    #[test]
    fn checked_factorial_test() {
        assert_almost_eq!(decimal!(10).checked_factorial().unwrap(), decimal!(3628800));
        assert_almost_eq_by!(
            decimal!(0.3).checked_factorial().unwrap(),
            decimal!(0.8974706963062771884937549547),
            10,
        );
        assert_almost_eq_by!(
            decimal!(6.5).checked_factorial().unwrap(),
            decimal!(1871.254305797788346476077053),
            10,
        );
    }

    #[test]
    fn sin_test() {
        assert_almost_eq!(
            decimal!(180).to_radians().checked_sin().unwrap(),
            Decimal::zero(),
        );
        assert_almost_eq!(
            decimal!(-180).to_radians().checked_sin().unwrap(),
            Decimal::zero(),
        );
        assert_almost_eq!(
            decimal!(90).to_radians().checked_sin().unwrap(),
            decimal!(1)
        );
        assert_almost_eq!(
            decimal!(-90).to_radians().checked_sin().unwrap(),
            decimal!(-1)
        );
        assert_almost_eq!(
            decimal!(45).to_radians().checked_sin().unwrap(),
            decimal!(0.7071067811865475244008443621),
        );
    }

    #[test]
    fn cos_test() {
        assert_almost_eq!(
            decimal!(90).to_radians().checked_cos().unwrap(),
            Decimal::zero(),
        );
        assert_almost_eq!(
            decimal!(0).to_radians().checked_cos().unwrap(),
            Decimal::one(),
        );
        assert_almost_eq!(
            decimal!(45).to_radians().checked_cos().unwrap(),
            decimal!(0.7071067811865475244008443621),
        );
        assert_almost_eq!(
            decimal!(30).to_radians().checked_cos().unwrap(),
            decimal!(0.8660254037844386467637231707),
        );
    }

    #[test]
    fn tan_test() {
        assert_almost_eq!(
            decimal!(45).to_radians().checked_tan().unwrap(),
            Decimal::one(),
        );
        assert_almost_eq!(
            decimal!(30).to_radians().checked_tan().unwrap(),
            decimal!(0.57735026918962576451),
        );
    }

    #[test]
    fn asin_test() {
        assert!(decimal!(1).asin().is_some());
        assert!(decimal!(-1).asin().is_some());

        assert!(decimal!(2).asin().is_none());
        assert!(decimal!(-2).asin().is_none());

        assert_almost_eq!(decimal!(1).asin().unwrap().to_degrees(), decimal!(90));
        assert_almost_eq!(
            decimal!(0.707106781186547524400)
                .asin()
                .unwrap()
                .to_degrees(),
            decimal!(45),
        );
    }

    #[test]
    fn acos_test() {
        assert!(decimal!(1).acos().is_some());
        assert!(decimal!(-1).acos().is_some());

        assert!(decimal!(2).acos().is_none());
        assert!(decimal!(-2).acos().is_none());

        assert_almost_eq!(decimal!(0).acos().unwrap().to_degrees(), decimal!(90));
        assert_almost_eq!(
            decimal!(0.707106781186547524400)
                .acos()
                .unwrap()
                .to_degrees(),
            decimal!(45),
        );
    }

    #[test]
    fn atan_test() {
        assert_almost_eq!(decimal!(0).atan().to_degrees(), decimal!(0));
        assert_almost_eq!(decimal!(1).atan().to_degrees(), decimal!(45));
        assert_almost_eq!(decimal!(-1).atan().to_degrees(), decimal!(-45));
    }

    #[test]
    fn sinh_test() {
        assert_almost_eq!(
            decimal!(2).sinh().unwrap(),
            decimal!(3.62686040784701876766821)
        );
        assert_almost_eq!(decimal!(0).sinh().unwrap(), decimal!(0));
        assert_almost_eq!(
            decimal!(-1).sinh().unwrap(),
            decimal!(-1.175201193643801456882)
        )
    }

    #[test]
    fn cosh_test() {
        assert_almost_eq!(
            decimal!(2).cosh().unwrap(),
            decimal!(3.7621956910836314595622134)
        );
        assert_almost_eq!(decimal!(0).cosh().unwrap(), decimal!(1));
        assert_almost_eq!(
            decimal!(-1).cosh().unwrap(),
            decimal!(1.543080634815243778477905)
        )
    }

    #[test]
    fn tanh_test() {
        assert_almost_eq!(
            decimal!(2).tanh().unwrap(),
            decimal!(0.9640275800758168839464137)
        );
        assert_almost_eq!(decimal!(0).tanh().unwrap(), decimal!(0));
        assert_almost_eq!(
            decimal!(-1).tanh().unwrap(),
            decimal!(-0.76159415595576488811945)
        )
    }

    #[test]
    fn asinh_test() {
        assert_almost_eq!(decimal!(1).asinh(), decimal!(0.88137358701954302523260932));
        assert_almost_eq!(decimal!(0).asinh(), decimal!(0));
        assert_almost_eq!(decimal!(-1).asinh(), decimal!(-0.881373587019543025232609))
    }

    #[test]
    fn acosh_test() {
        assert!(decimal!(0).acosh().is_none());
        assert_almost_eq!(decimal!(1).acosh().unwrap(), decimal!(0));
        assert_almost_eq_by!(
            decimal!(2).acosh().unwrap(),
            decimal!(1.316957896924816708),
            10
        );
    }

    #[test]
    fn atanh_test() {
        assert!(decimal!(-2).atanh().is_none());
        assert!(decimal!(2).atanh().is_none());
        assert_almost_eq!(decimal!(0).atanh().unwrap(), decimal!(0));
        assert_almost_eq!(
            decimal!(0.25).atanh().unwrap(),
            decimal!(0.25541281188299534160275)
        );
    }
}
