use crate::decimal::consts;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, One, ToPrimitive, Zero};
use rust_decimal_macros::*;

/// Extended math operations for `Decimal`.
pub trait DecimalExt {
    fn is_integer(&self) -> bool;
    fn checked_pow(self, exponent: Decimal) -> Option<Decimal>;
    fn checked_pow_n(self, exponent: i64) -> Option<Decimal>;
    fn checked_sqrt(self) -> Option<Decimal>;
    fn checked_cbrt(self) -> Option<Decimal>;
    fn checked_log(self, exponent: Decimal) -> Option<Decimal>;
    fn checked_ln(self) -> Option<Decimal>;
    fn checked_exp(self) -> Option<Decimal>;
    fn checked_factorial(self) -> Option<Decimal>;
    fn sin(self) -> Decimal;
    fn cos(self) -> Decimal;
    fn tan(self) -> Option<Decimal>;
}

trait ApproxEq{
    fn approx_eq(&self, other: &Self, delta: &Self) -> bool;
}

impl ApproxEq for Decimal{
    #[inline]
    fn approx_eq(&self, other: &Self, delta: &Self) -> bool {
        (self - other).abs() < *delta
    }
}

impl DecimalExt for Decimal {
    #[inline]
    fn is_integer(&self) -> bool {
        self.fract().is_zero()
    }

    fn checked_pow(self, exponent: Decimal) -> Option<Decimal> {
        if exponent.is_integer() {
            return self.checked_pow_n(exponent.to_i64().unwrap());
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
        Some(b.checked_exp()?)
    }

    fn checked_pow_n(self, mut exponent: i64) -> Option<Decimal> {
        if exponent == 0 {
            return Some(Decimal::one());
        }

        if exponent == 1 {
            return Some(self);
        }

        if exponent < 0 {
            return Self::checked_pow_n(Decimal::one() / self, -exponent);
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
            exponent = exponent >> 1;
        }

        Some(result)
    }

    fn checked_sqrt(self) -> Option<Decimal> {
        //return Some(self.checked_pow(decimal.decimal::HALF)?);

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

    fn checked_cbrt(self) -> Option<Decimal>{
        if self.is_one() {
            return Some(self);
        }

        let mut x = Decimal::one();

        // Using Newton's Method
        // See: https://en.wikipedia.org/wiki/Cube_root#Numerical_methods
        for _ in 0..consts::TAYLOR_SERIES_ITERATIONS{
            let xx = x.checked_mul(x)?;
            let x2 = consts::TWO.checked_mul(x)?;
            x = self.checked_div(xx)?
                .checked_add(x2)?
                .checked_mul(consts::ONE_FRACT_3)?;
        }

        Some(x)
    }

    fn checked_log(self, base: Decimal) -> Option<Decimal> {
        let a = Self::checked_ln(self)?;
        if base == consts::TEN {
            return a.checked_div(consts::LN_10_INV);
        }

        let b = Self::checked_ln(base)?;
        Some(a.checked_div(b)?)
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
            let mut a = self.clone();

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
        // Error: 0.0000000000000000000000000007
        const ITERATIONS: u32 = consts::TAYLOR_SERIES_ITERATIONS * 10;
        let mut result = Decimal::zero();

        for n in 1..ITERATIONS {
            let sign = Self::checked_pow_n(consts::ONE_MINUS, (n + 1).into())?;
            let x = self.checked_sub(Decimal::one())?;
            let xn = Self::checked_pow_n(x, n.into())?;
            let div = xn.checked_div(n.into())?;
            let y = div.checked_mul(sign)?;

            // result += [(-1)^(n + 1)] * (value - 1)^n / n
            result = result.checked_add(y)?;
        }

        Some(result)
    }

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

        Some(result)
    }

    fn checked_factorial(self) -> Option<Decimal> {
        if self.is_sign_negative() {
            return None;
        }

        if self == consts::TWO || self == consts::ONE || self == consts::ZERO {
            return Some(Decimal::one());
        }

        if self < Decimal::one() {
            return gamma(self + consts::ONE);
        }

        let mut result = self.clone();
        let mut n = self - Decimal::one();

        while n > Decimal::zero() {
            result = result.checked_mul(n)?;
            n -= Decimal::one();
        }

        if !n.is_zero() {
            result *= gamma(n + consts::ONE)?;
        }

        Some(result)
    }

    fn sin(self) -> Decimal {
        let radians : Decimal = self % consts::PI_2;

        if radians == Decimal::zero() ||
            ApproxEq::approx_eq(&radians.abs(), &consts::PI, &consts::EPSILON) ||
            ApproxEq::approx_eq(&radians.abs(), &consts::PI_2, &consts::EPSILON) {
            return Decimal::zero();
        }

        if ApproxEq::approx_eq(&radians, &consts::PI_FRACT_2, &consts::EPSILON) {
            return consts::ONE;
        }

        if  ApproxEq::approx_eq(&radians, &consts::PI_3_FRACT_2, &consts::EPSILON) ||
            ApproxEq::approx_eq(&radians.abs(), &consts::PI_FRACT_2, &consts::EPSILON) {
            return consts::ONE_MINUS;
        }

        // Using Taylor Series
        // See: https://en.wikipedia.org/wiki/Taylor_series#Trigonometric_functions

        let xx: Decimal = radians * radians;
        let mut factor = radians;
        let mut result = radians;

        for n in 1..consts::TAYLOR_SERIES_ITERATIONS {
            factor *= -xx / Decimal::from_u32((2 * n + 1) * (2 * n)).unwrap();
            result += factor;
        }

        result
    }

    fn cos(self) -> Decimal {
        let radians : Decimal =  self  % consts::PI_2;

        if radians.is_zero() {
            return Decimal::one();
        }

        if ApproxEq::approx_eq(&radians, &consts::PI_FRACT_2, &consts::EPSILON) {
            return Decimal::zero();
        }

        // Using Taylor Series
        // See: https://en.wikipedia.org/wiki/Taylor_series#Trigonometric_functions
        let xx: Decimal = radians * radians;
        let mut factor: Decimal = -xx / consts::TWO;
        let mut result = Decimal::one() + factor;

        for n in 2..consts::TAYLOR_SERIES_ITERATIONS {
            factor *= -xx / Decimal::from_u32(2 * n * (2 * n - 1)).unwrap();
            result += factor;
        }

        result
    }

    fn tan(self) -> Option<Decimal> {
        let cos = self.cos();
        if cos.is_zero() {
            None
        } else {
            Some(self.sin() / cos)
        }
    }
}

fn gamma(mut x: Decimal) -> Option<Decimal> {
    //Using Coefficients from: https://mrob.com/pub/ries/lanczos-gamma.html
    const G : Decimal = dec!(4.7421875);
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
        Some(consts::PI / Decimal::sin(consts::PI * x) * gamma(Decimal::one() - x)?)
    } else {
        // Lanczos solve gamma for (x + 1)
        x -= Decimal::one();

        let mut factor: Decimal = P[0];
        for n in 1..P.len() {
            factor += P[n] / (x + Decimal::from_usize(n).unwrap());
        }

        let t: Decimal = x + G + consts::HALF;
        let result = Decimal::checked_sqrt(consts::TWO * consts::PI)?
            * Decimal::checked_pow(t, x + consts::HALF)?
            * Decimal::checked_exp(-t)?
            * factor;

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Equality using an epsilon value
    macro_rules! assert_approx_eq {
        ($left:expr, $right:expr, $delta:expr) => ({
        match (&$left, &$right, &$delta) {
            (left_val, right_val, delta_val) => {
                if !((left_val - right_val).abs() < *delta_val) {
                    panic!(r#"assertion failed: `(left == right) with error of: +/-{}`
  left: `{:?}`,
 right: `{:?}`"#, &*delta_val, &*left_val, &*right_val)
                }
            }
        }
    });

        ($left:expr, $right:expr) => ({
            assert_approx_eq!($left, $right, consts::EPSILON)
        });
    }

    macro_rules! decimal{
        ($value:expr) => {
            dec!($value) as Decimal
        }
    }

    #[test]
    fn is_integer_test(){
        assert!(decimal!(10).is_integer());
        assert!(decimal!(-20).is_integer());
        assert!(decimal!(5.0).is_integer());

        assert!(!decimal!(15.5).is_integer());
        assert!(!decimal!(-20.2).is_integer());
        assert!(!decimal!(5.2).is_integer());
    }

    #[test]
    fn checked_sqrt_test() {
        assert_eq!(Decimal::checked_sqrt(dec!(25)).unwrap(), dec!(5));
        assert_eq!(
            Decimal::checked_sqrt(dec!(2)).unwrap(),
            dec!(1.4142135623730950488016887242097)
        );
    }

    #[test]
    fn checked_cbrt_test(){
        assert_approx_eq!(dec!(10).checked_cbrt().unwrap(), dec!(2.1544346900318837217592935665194));
        assert_approx_eq!(dec!(-5).checked_cbrt().unwrap(), dec!(-1.7099759466766969893531088725439));
    }

    #[test]
    fn checked_pow_n_test() {
        assert_eq!(Decimal::checked_pow_n(dec!(5), 0).unwrap(), dec!(1));
        assert_eq!(Decimal::checked_pow_n(dec!(8), 1).unwrap(), dec!(8));
        assert_eq!(Decimal::checked_pow_n(dec!(0), 0).unwrap(), dec!(1));
        assert_eq!(Decimal::checked_pow_n(dec!(2), 3).unwrap(), dec!(8));
        assert_eq!(Decimal::checked_pow_n(dec!(5), -3).unwrap(), dec!(0.008));
    }

    #[test]
    fn checked_pow_test() {
        assert_eq!(Decimal::checked_pow(dec!(9), dec!(0.5)).unwrap(), 3.into());
        assert_eq!(
            Decimal::checked_pow(dec!(4), dec!(-0.25)).unwrap(),
            dec!(0.70710678118654752440084436210485)
        );
    }

    #[test]
    fn checked_ln_test() {
        assert_eq!(Decimal::checked_ln(dec!(0)), None);
        assert_eq!(Decimal::checked_ln(dec!(-2)), None);

        assert_approx_eq!(
            Decimal::checked_ln(dec!(10)).unwrap(),
            dec!(2.3025850929940456840179914546844)
        );
        assert_approx_eq!(
            Decimal::checked_ln(dec!(9)).unwrap(),
            dec!(2.1972245773362193827904904738451)
        );
        assert_approx_eq!(
            Decimal::checked_ln(dec!(25)).unwrap(),
            dec!(3.2188758248682007492015186664524)
        );
    }

    #[test]
    fn checked_exp_test() {
        assert_eq!(Decimal::checked_exp(dec!(0)).unwrap(), dec!(1));
        assert_eq!(Decimal::checked_exp(dec!(1)).unwrap(), consts::E);

        assert_approx_eq!(
            Decimal::checked_exp(dec!(3)).unwrap(),
            dec!(20.085536923187667740928529654582)
        );
        assert_approx_eq!(
            Decimal::checked_exp(dec!(-4)).unwrap(),
            dec!(0.01831563888873418029371802127324)
        );
        assert_approx_eq!(
            Decimal::checked_exp(dec!(0.5)).unwrap(),
            dec!(1.6487212707001281468486507878142)
        );
    }

    #[test]
    fn checked_factorial_test() {
        const ERROR : Decimal = dec!(0.0000000000001);

        assert_eq!(Decimal::checked_factorial(dec!(10)).unwrap(), dec!(3628800));
        assert_approx_eq!(
            Decimal::checked_factorial(dec!(0.3)).unwrap(),
            dec!(0.89747069630627718849375495477148),
            ERROR
        );
        assert_approx_eq!(
            Decimal::checked_factorial(dec!(6.5)).unwrap(),
            dec!(1871.254305797788346476077053604),
            ERROR
        );
    }

    #[test]
    fn sin_test() {
        assert_eq!(Decimal::sin(radians(dec!(180))), Decimal::zero());
        assert_eq!(Decimal::sin(radians(dec!(-180))), Decimal::zero());
        assert_eq!(Decimal::sin(radians(dec!(90))), dec!(1));
        assert_eq!(Decimal::sin(radians(dec!(-90))), dec!(-1));
        assert_approx_eq!(
            Decimal::sin(radians(dec!(45))),
            dec!(0.70710678118654752440084436210485)
        );
    }

    #[test]
    fn cos_test() {
        assert_eq!(Decimal::cos(radians(dec!(90))), Decimal::zero());
        assert_eq!(Decimal::cos(radians(dec!(0))), Decimal::one());
        assert_approx_eq!(
            Decimal::cos(radians(dec!(45))),
            dec!(0.70710678118654752440084436210485)
        );
        assert_approx_eq!(
            Decimal::cos(radians(dec!(30))),
            dec!(0.86602540378443864676372317075294)
        );
    }

    fn radians(n: Decimal) -> Decimal {
        n * (consts::PI / dec!(180))
    }
}