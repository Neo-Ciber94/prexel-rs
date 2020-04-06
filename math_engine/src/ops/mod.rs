pub mod checked;
pub mod unchecked;

pub mod math {
    use std::fmt::Debug;
    use std::ops::{Mul, Sub};
    use num_traits::{FromPrimitive, Inv, One, ToPrimitive, Zero};
    use rand::random;
    use crate::error::*;
    pub use crate::function::{BinaryFunction, Function, UnaryFunction};
    use crate::function::{Associativity, Notation, Precedence};
    use crate::utils::gamma::gamma;
    use crate::Result;
    use crate::utils::approx::Approx;

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
    impl<N> UnaryFunction<N> for Factorial
    where
        N: Clone
            + Debug
            + Zero
            + One
            + Sub<N, Output = N>
            + Mul<N, Output = N>
            + PartialOrd
            + ToPrimitive
            + FromPrimitive,
    {
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

            // 0! = 1 and 1! = 1
            if value.is_zero() || value.is_one() {
                return Ok(N::one());
            }

            // Quick path for: `x < 1`
            if value < N::one() {
                return if let Some(n) = value.to_f64() {
                    let result = gamma(n + 1f64);
                    N::from_f64(result).ok_or(Error::from(ErrorKind::Overflow))
                } else {
                    Err(Error::from(ErrorKind::Overflow))
                };
            }

            let mut total = value;
            let mut next = total.clone() - N::one();

            while next >= N::one() {
                total = total.clone() * next.clone();
                next = next.clone() - N::one();
            }

            // If next value is non-zero, apply `Gamma function`.
            if !next.is_zero() {
                if let (Some(mut total_f64), Some(n)) = (total.to_f64(), next.to_f64()) {
                    total_f64 *= gamma(n + 1f64);
                    N::from_f64(total_f64).ok_or(Error::from(ErrorKind::Overflow))
                } else {
                    Err(Error::from(ErrorKind::Overflow))
                }
            } else {
                Ok(total.clone())
            }
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
            if let (Some(base), Some(exp)) = (left.to_f64(), right.to_f64()) {
                N::from_f64(f64::powf(base, exp)).ok_or(Error::from(ErrorKind::Overflow))
            } else {
                Err(Error::from(ErrorKind::Overflow))
            }
        }
    }

    pub struct MaxFunction;
    impl<N: PartialOrd + Clone> Function<N> for MaxFunction {
        fn name(&self) -> &str {
            "max"
        }

        fn call(&self, args: &[N]) -> Result<N> {
            if args.len() <= 1{
                return Err(Error::from(ErrorKind::InvalidArgumentCount));
            }

            let mut max = None;

            for n in args {
                match max {
                    None => max = Some(n.clone()),
                    Some(ref current_max) => {
                        if n > current_max {
                            max = Some(n.clone())
                        }
                    }
                }
            }

            max.ok_or(Error::from(ErrorKind::InvalidArgumentCount))
        }
    }

    pub struct MinFunction;
    impl<N: PartialOrd + Clone> Function<N> for MinFunction {
        fn name(&self) -> &str {
            "min"
        }

        fn call(&self, args: &[N]) -> Result<N> {
            if args.len() <= 1{
                return Err(Error::from(ErrorKind::InvalidArgumentCount));
            }

            let mut min = None;

            for n in args {
                match min {
                    None => min = Some(n.clone()),
                    Some(ref current_min) => {
                        if n < current_min {
                            min = Some(n.clone());
                        }
                    }
                }
            }

            min.ok_or(Error::from(ErrorKind::InvalidArgumentCount))
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
                        let result = try_to_float(&args[0])
                            .map(|n| n.approx())?
                            .$method_name();

                        N::from_f64(result)
                            .ok_or(Error::from(ErrorKind::Overflow))
                    }
                }
            }
        };
    }

    macro_rules! forward_func_inv_impl {
        ($func_name:ident, $method_name:ident) => {
            forward_func_inv_impl!($func_name, $method_name, $method_name);
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
                        let result = try_to_float(&args[0])
                            .map(|n| n.approx())?
                            .$method_name()
                            .inv();

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
    impl<N: ToPrimitive + FromPrimitive> Function<N> for LogFunction {
        fn name(&self) -> &str {
            "log"
        }

        fn call(&self, args: &[N]) -> Result<N> {
            match args.len() {
                1 => match args[0].to_f64().map(f64::log10) {
                    Some(n) => {
                        if n.is_nan() || n.is_infinite() {
                            Err(Error::from(ErrorKind::NAN))
                        } else {
                            N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow))
                        }
                    }
                    None => Err(Error::from(ErrorKind::Overflow)),
                },
                2 => {
                    let x = args[0].to_f64();
                    let y = args[1].to_f64();

                    match (x, y) {
                        (Some(value), Some(base)) => {
                            let result = value.log(base);
                            if result.is_nan() || result.is_infinite() {
                                Err(Error::from(ErrorKind::NAN))
                            } else {
                                N::from_f64(result).ok_or(Error::from(ErrorKind::Overflow))
                            }
                        }
                        _ => Err(Error::from(ErrorKind::Overflow)),
                    }
                }
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
            }
        }
    }

    pub struct RandFunction;
    impl<N: ToPrimitive + FromPrimitive> Function<N> for RandFunction {
        #[inline]
        fn name(&self) -> &str {
            "random"
        }

        fn call(&self, args: &[N]) -> Result<N> {
            match args.len() {
                0 => N::from_f64(random::<f64>()).ok_or(Error::from(ErrorKind::Overflow)),
                1 => {
                    let max = try_to_float(&args[0])?;
                    if max.is_sign_negative(){
                        return Err(Error::from(ErrorKind::NegativeValue));
                    }

                    N::from_f64(random::<f64>() * max).ok_or(Error::from(ErrorKind::Overflow))
                }
                2 => {
                    let min = try_to_float(&args[0])?;
                    let max = try_to_float(&args[1])?;

                    if min > max {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid range for `Random`: min > max, {} > {}", min, max))
                        )
                    }

                    let value = min + ((max - min) * random::<f64>());
                    N::from_f64(value).ok_or(Error::from(ErrorKind::Overflow))
                }
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
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
                        match args[0].to_f64()
                            .map(f64::to_radians)
                            .map(f64::$method_name)
                            .map(|n| n.approx()){
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
                        match args[0]
                            .to_f64()
                            .map(f64::to_radians)
                            .map(f64::$method_name)
                            .map(|n| n.approx())
                            .map(f64::inv)
                        {
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
    forward_func_impl!(SinhFunction, sinh);

    pub struct CoshFunction;
    forward_func_impl!(CoshFunction, cosh);

    pub struct TanhFunction;
    forward_func_impl!(TanhFunction, tanh);

    pub struct CschFunction;
    forward_func_inv_impl!(CschFunction, sinh, csch);

    pub struct SechFunction;
    forward_func_inv_impl!(SechFunction, cosh, sech);

    pub struct CothFunction;
    forward_func_inv_impl!(CothFunction, tanh, coth);

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
                        match args[0].to_f64()
                            .map(f64::$method_name)
                            .map(f64::to_degrees)
                            .map(|n| n.approx()){
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
                        match args[0].to_f64()
                            .map(f64::inv)
                            .map(f64::$method_name)
                            .map(f64::to_degrees)
                            .map(|n| n.approx()){
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
            stringify!(atan)
        }

        fn call(&self, args: &[N]) -> Result<N> {
            match args.len() {
                1 => match args[0].to_f64().map(f64::atan).map(f64::to_degrees) {
                    Some(n) => {
                        if n.is_nan() || n.is_infinite() {
                            Err(Error::from(ErrorKind::NAN))
                        } else {
                            N::from_f64(n).ok_or(Error::from(ErrorKind::Overflow))
                        }
                    }
                    None => Err(Error::from(ErrorKind::Overflow)),
                },
                2 => {
                    if let (Some(y), Some(x)) = (args[0].to_f64(), args[1].to_f64()){
                        if y.is_zero() && x.is_zero(){
                            return Err(Error::from(ErrorKind::NAN));
                        }

                        let result = y.atan2(x).to_degrees();
                        if result.is_nan() || result.is_infinite() {
                            Err(Error::from(ErrorKind::NAN))
                        } else {
                            N::from_f64(result).ok_or(Error::from(ErrorKind::Overflow))
                        }
                    } else{
                        Err(Error::from(ErrorKind::Overflow))
                    }
                }
                _ => Err(Error::from(ErrorKind::InvalidArgumentCount)),
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
    pub(crate) fn try_to_float<N: ToPrimitive>(n: &N) -> Result<f64> {
        match n.to_f64() {
            Some(n) => {
                if n.is_nan() || n.is_infinite() {
                    Err(Error::from(ErrorKind::NAN))
                } else {
                    Ok(n)
                }
            }
            None => Err(Error::from(ErrorKind::Overflow)),
        }
    }

    /// For reduce errors as: `0.1 + 0.2 ≠ 0.3`.
    impl Approx for f64{
        #[inline]
        fn approx(&self) -> Self {
            const ERROR : f64 = 0.000_000_000_1_f64;
            self.approx_by(&ERROR)
        }

        fn approx_by(&self, delta: &Self) -> Self {
            let r = self.round();

            if (self - r).abs() < *delta{
                r
            }
            else{
                *self
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::math::*;
    use num_traits::Inv;
    use crate::utils::approx::Approx;

    const ERROR : f64 = 0.000_000_000_01;

    fn almost_eq(x: f64, y: f64, delta: f64) -> bool{
        (y - x).abs() <= delta
    }

    fn empty_array<T>() -> Box<[T]>{
        vec![].into_boxed_slice()
    }

    #[test]
    fn unary_plus_test(){
        let instance = UnaryPlus;

        assert_eq!(instance.call(-10_f64), Ok(-10_f64));
        assert_eq!(instance.call(5), Ok(5));
    }

    #[test]
    fn factorial_test(){
        let instance = Factorial;

        assert_eq!(instance.call(5), Ok(120));
        assert_eq!(instance.call(1), Ok(1));
        assert_eq!(instance.call(0), Ok(1));

        assert_eq!(instance.call(10_f64), Ok(3628800_f64));

        //assert_eq!(instance.call(0.2_f64), Ok(0.91816874239976061_f64));
        //assert_eq!(instance.call(3.2_f64), Ok(7.75668953579317763_f64));
        assert!(almost_eq(instance.call(0.2_f64).unwrap(), 0.91816874239976061_f64, ERROR));
        assert!(almost_eq(instance.call(3.2_f64).unwrap(), 7.75668953579317763_f64, ERROR));
    }

    #[test]
    fn pow_test(){
        let instance = PowOperator;

        assert_eq!(instance.call(2, 3), Ok(8));
        assert_eq!(instance.call(4, 0), Ok(1));
        assert_eq!(instance.call(0, 2), Ok(0));

        assert!(almost_eq(instance.call(3_f64, -3_f64).unwrap(), 0.03703703703703703_f64, ERROR));
        assert!(almost_eq(instance.call(3_f64, -3_f64).unwrap(), 0.03703703703703703_f64, ERROR));

        assert!(almost_eq(instance.call(3_f64, 0.2_f64).unwrap(), 1.245730939615517325966_f64, ERROR));
        assert!(almost_eq(instance.call(3_f64, -0.2_f64).unwrap(), 0.802741561760230682095_f64, ERROR));
    }

    #[test]
    fn min_test(){
        let instance = MinFunction;

        assert_eq!(instance.call(&[-10_f64, 3_f64]), Ok(-10_f64));
        assert_eq!(instance.call(&[5, 3, 1]), Ok(1));

        assert!(instance.call(&[5]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err())
    }

    #[test]
    fn max_test(){
        let instance = MaxFunction;

        assert_eq!(instance.call(&[-10_f64, 3_f64]), Ok(3_f64));
        assert_eq!(instance.call(&[5, 3, 1]), Ok(5));

        assert!(instance.call(&[5]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err())
    }

    #[test]
    fn floor_test(){
        let instance = FloorFunction;

        assert_eq!(instance.call(&[8.2_f64]), Ok(8_f64));

        assert!(instance.call(&[8.2_f64, 3_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err())
    }

    #[test]
    fn ceil_test(){
        let instance = CeilFunction;

        assert_eq!(instance.call(&[8.2_f64]), Ok(9_f64));

        assert!(instance.call(&[8.2_f64, 3_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err())
    }

    #[test]
    fn truncate_test(){
        let instance = TruncateFunction;

        assert_eq!(instance.call(&[8.2_f64]), Ok(8_f64));
        assert_eq!(instance.call(&[8.9_f64]), Ok(8_f64));

        assert!(instance.call(&[8.2_f64, 3_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err())
    }

    #[test]
    fn round_test(){
        let instance = RoundFunction;

        assert_eq!(instance.call(&[8.2_f64]), Ok(8_f64));
        assert_eq!(instance.call(&[8.9_f64]), Ok(9_f64));

        assert!(instance.call(&[8.2_f64, 3_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err())
    }

    #[test]
    fn sign_test(){
        let instance = SignFunction;

        assert_eq!(instance.call(&[6]), Ok(1));
        assert_eq!(instance.call(&[-3]), Ok(-1));
        assert_eq!(instance.call(&[6_f64]), Ok(1_f64));
        assert_eq!(instance.call(&[-4_f64]), Ok(-1_f64));

        assert!(instance.call(&[8.2_f64, 3_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err())
    }

    #[test]
    fn sqrt_test(){
        let instance = SqrtFunction;

        assert_eq!(instance.call(&[25]), Ok(5));
        assert_eq!(instance.call(&[9]), Ok(3));
        assert_eq!(instance.call(&[5.5_f64]), Ok(5.5_f64.sqrt()));
        assert_eq!(instance.call(&[0.2_f64]), Ok(0.2_f64.sqrt()));

        assert!(instance.call(&[8.2_f64, 3_f64]).is_err());
        assert!(instance.call(&[-9]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err())
    }

    #[test]
    fn exp_test(){
        let instance = ExpFunction;

        fn compute_exp(func: &ExpFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.exp()));
        }

        compute_exp(&instance, 0_f64);
        compute_exp(&instance, 5_f64);
        compute_exp(&instance, -3_f64);

        assert!(instance.call(&[8.2_f64, 3_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err())
    }

    #[test]
    fn ln_test(){
        let instance = LnFunction;

        fn compute_ln(func: &LnFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.ln()));
        }

        compute_ln(&instance, 25_f64);
        compute_ln(&instance, 5_f64);

        assert!(instance.call(&[0]).is_err());
        assert!(instance.call(&[-5]).is_err());
        assert!(instance.call(&[8.2_f64, 3_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err())
    }

    #[test]
    fn log_test(){
        let instance = LogFunction;

        fn compute_log10(func: &LogFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.log10()), "Log({})", value);
        }

        fn compute_log(func: &LogFunction, value: f64, base: f64){
            assert_eq!(func.call(&[value, base]), Ok(value.log(base)), "Log({}, base = {})", value, base);
        }

        compute_log10(&instance, 25_f64);
        compute_log(&instance, 5_f64, 20_f64);

        assert!(instance.call(&[0]).is_err());
        assert!(instance.call(&[-25]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err())
    }

    #[test]
    fn rand_test(){
        const SAMPLES : usize = 1000;
        let instance = RandFunction;

        fn compute_random(rand: &RandFunction){
            for _ in 0..SAMPLES{
                let value : f64 = rand.call(&empty_array::<f64>()).unwrap();
                assert!(value >= 0_f64 && value < 1_f64, "value out of range: {0} >= 0 && {0} < 1", value)
            }
        }

        fn compute_random_range1(rand: &RandFunction, max: f64){
            for _ in 0..SAMPLES{
                let value : f64 = rand.call(&[max]).unwrap();
                assert!(value >= 0_f64 && value < max, "value out of range: {0} >= 0 && {0} < {1}", value, max)
            }
        }

        fn compute_random_range2(rand: &RandFunction, min: f64, max: f64){
            for _ in 0..SAMPLES{
                let value : f64 = rand.call(&[min, max]).unwrap();
                assert!(value >= min && value < max, "value out of range: {0} >= {1} && {0} < {2}", value, min, max)
            }
        }

        compute_random(&instance);
        compute_random_range1(&instance, 10_f64);

        compute_random_range2(&instance, 5_f64, 10_f64);
        compute_random_range2(&instance, -10_f64, -5_f64);
        compute_random_range2(&instance, -10_f64, 10_f64);

        assert!(instance.call(&[-12]).is_err());
        assert!(instance.call(&[20, 10]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
    }

    #[test]
    fn sin_test(){
        let instance = SinFunction;

        fn compute_sin(func: &SinFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.to_radians().sin()), "Sin({})", value);
        }

        compute_sin(&instance, 45_f64);
        compute_sin(&instance, 30_f64);
        compute_sin(&instance, -90_f64);
        compute_sin(&instance, 0_f64);

        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn cos_test(){
        let instance = CosFunction;

        fn compute_cos(func: &CosFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.to_radians().cos().approx()), "Cos({})", value);
        }

        compute_cos(&instance, 45_f64);
        compute_cos(&instance, 30_f64);
        compute_cos(&instance, -90_f64);
        compute_cos(&instance, 0_f64);

        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn tan_test(){
        let instance = TanFunction;

        fn compute_tan(func: &TanFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.to_radians().tan().approx()), "Tan({})", value);
        }

        compute_tan(&instance, 45_f64);
        compute_tan(&instance, 30_f64);
        compute_tan(&instance, 0_f64);

        assert!(instance.call(&[90]).is_err());
        assert!(instance.call(&[-90]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn csc_test(){
        let instance = CscFunction;

        fn compute_csc(func: &CscFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.to_radians().sin().inv()), "Csc({})", value);
        }

        compute_csc(&instance, 45_f64);
        compute_csc(&instance, 30_f64);
        compute_csc(&instance, -90_f64);

        assert!(instance.call(&[0_f64]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn sec_test(){
        let instance = SecFunction;

        fn compute_sec(func: &SecFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.to_radians().cos().inv()), "Sec({})", value);
        }

        compute_sec(&instance, 45_f64);
        compute_sec(&instance, 30_f64);

        assert!(instance.call(&[90]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn cot_test(){
        let instance = CotFunction;

        fn compute_cot(func: &CotFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.to_radians().tan().approx().inv()), "Cot({})", value);
        }

        compute_cot(&instance, 45_f64);
        compute_cot(&instance, 30_f64);

        //assert!(instance.call(&[90]).is_err()); error due float precision
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn sinh_test(){
        let instance = SinhFunction;

        fn compute_sinh(func: &SinhFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.sinh()), "Sinh({})", value);
        }

        compute_sinh(&instance, 45_f64);
        compute_sinh(&instance, 30_f64);
        compute_sinh(&instance, -90_f64);
        compute_sinh(&instance, 0_f64);

        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn cosh_test(){
        let instance = CoshFunction;

        fn compute_cosh(func: &CoshFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.cosh()), "Cosh({})", value);
        }

        compute_cosh(&instance, 45_f64);
        compute_cosh(&instance, 30_f64);
        compute_cosh(&instance, -90_f64);
        compute_cosh(&instance, 0_f64);

        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn tanh_test(){
        let instance = TanhFunction;

        fn compute_tanh(func: &TanhFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.tanh()), "Tanh({})", value);
        }

        compute_tanh(&instance, 45_f64);
        compute_tanh(&instance, 30_f64);
        compute_tanh(&instance, -90_f64);
        compute_tanh(&instance, 0_f64);

        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn csch_test(){
        let instance = CschFunction;

        fn compute_csch(func: &CschFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.sinh().inv()), "Csch({})", value);
        }

        compute_csch(&instance, 45_f64);
        compute_csch(&instance, 30_f64);
        compute_csch(&instance, -90_f64);

        // assert!(instance.call(&[0_f64]).is_err()); float precision error
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn sech_test(){
        let instance = SechFunction;

        fn compute_sech(func: &SechFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.cosh().inv()), "Sech({})", value);
        }

        compute_sech(&instance, 45_f64);
        compute_sech(&instance, 30_f64);
        compute_sech(&instance, -90_f64);
        compute_sech(&instance, 0_f64);

        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn coth_test(){
        let instance = CothFunction;

        fn compute_coth(func: &CothFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.tanh().inv()), "Coth({})", value);
        }

        compute_coth(&instance, 45_f64);
        compute_coth(&instance, 30_f64);
        compute_coth(&instance, -90_f64);

        // assert!(instance.call(&[0_f64]).is_err()); float precision error
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn asin_test(){
        let instance = ASinFunction;

        fn compute_asin(func: &ASinFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.asin().to_degrees()), "ASin({})", value);
        }

        compute_asin(&instance, -1_f64);
        compute_asin(&instance, 1_f64);
        compute_asin(&instance, 0.3_f64);

        assert!(instance.call(&[10]).is_err());
        assert!(instance.call(&[-20]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn acos_test(){
        let instance = ACosFunction;

        fn compute_acos(func: &ACosFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.acos().to_degrees()), "Acos({})", value);
        }

        compute_acos(&instance, -1_f64);
        compute_acos(&instance, 1_f64);
        compute_acos(&instance, 0.3_f64);

        assert!(instance.call(&[10]).is_err());
        assert!(instance.call(&[-20]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn atan_test(){
        let instance = ATanFunction;

        fn compute_atan(func: &ATanFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.atan().to_degrees()), "Atan({})", value);
        }

        fn compute_atan2(func: &ATanFunction, y: f64, x: f64){
            assert_eq!(func.call(&[y, x]), Ok(y.atan2(x).to_degrees()), "Atan({}, {})", y, x);
        }

        compute_atan(&instance, -1_f64);
        compute_atan(&instance, 1_f64);
        compute_atan(&instance, 0.3_f64);
        compute_atan(&instance, 19_f64);
        compute_atan(&instance, -21_f64);

        compute_atan2(&instance, -1_f64, 2_f64);
        compute_atan2(&instance, 1_f64, -3_f64);
        compute_atan2(&instance, 0.3_f64, 0.4_f64);

        assert!(instance.call(&[0_f64, 0_f64]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn acsc_test(){
        let instance = ACscFunction;

        fn compute_acsc(func: &ACscFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.inv().asin().to_degrees()), "ACsc({})", value);
        }

        compute_acsc(&instance, -1_f64);
        compute_acsc(&instance, 1_f64);
        compute_acsc(&instance, -22_f64);
        compute_acsc(&instance, 13_f64);

        assert!(instance.call(&[0]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn asec_test(){
        let instance = ASecFunction;

        fn compute_asec(func: &ASecFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.inv().acos().to_degrees()), "ASec({})", value);
        }

        compute_asec(&instance, -1_f64);
        compute_asec(&instance, 1_f64);
        compute_asec(&instance, -22_f64);
        compute_asec(&instance, 13_f64);

        assert!(instance.call(&[0]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn acot_test(){
        let instance = ACotFunction;

        fn compute_atan(func: &ACotFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.inv().atan().to_degrees()), "ACot({})", value);
        }

        compute_atan(&instance, -1_f64);
        compute_atan(&instance, 1_f64);
        compute_atan(&instance, 0.3_f64);
        compute_atan(&instance, 19_f64);
        compute_atan(&instance, -21_f64);

        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn asinh_test(){
        let instance = ASinhFunction;

        fn compute_asinh(func: &ASinhFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.asinh().to_degrees()), "ASinh({})", value);
        }

        compute_asinh(&instance, -1_f64);
        compute_asinh(&instance, 1_f64);
        compute_asinh(&instance, 0.3_f64);
        compute_asinh(&instance, 20_f64);

        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn acosh_test(){
        let instance = ACoshFunction;

        fn compute_acosh(func: &ACoshFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.acosh().to_degrees()), "ACosh({})", value);
        }

        compute_acosh(&instance, 1_f64);
        compute_acosh(&instance, 20_f64);

        assert!(instance.call(&[0_f64]).is_err());
        assert!(instance.call(&[-7_f64]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn atanh_test(){
        let instance = ATanhFunction;

        fn compute_atanh(func: &ATanhFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.atanh().to_degrees()), "ATanh({})", value);
        }

        compute_atanh(&instance, 0_f64);
        compute_atanh(&instance, 0.9_f64);

        assert!(instance.call(&[1]).is_err());
        assert!(instance.call(&[-1]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn acsch_test(){
        let instance = ACschFunction;

        fn compute_acsch(func: &ACschFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.inv().asinh().to_degrees()), "ACsch({})", value);
        }

        compute_acsch(&instance, -1_f64);
        compute_acsch(&instance, 1_f64);
        compute_acsch(&instance, -30_f64);
        compute_acsch(&instance, 20_f64);

        assert!(instance.call(&[0]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn asech_test(){
        let instance = ASechFunction;

        fn compute_asech(func: &ASechFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.inv().acosh().to_degrees()), "ASech({})", value);
        }

        compute_asech(&instance, 0.1_f64);
        compute_asech(&instance, 1_f64);

        assert!(instance.call(&[0]).is_err());
        assert!(instance.call(&[2]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }

    #[test]
    fn acoth_test(){
        let instance = ACothFunction;

        fn compute_acoth(func: &ACothFunction, value: f64){
            assert_eq!(func.call(&[value]), Ok(value.inv().atanh().to_degrees()), "ACoth({})", value);
        }

        compute_acoth(&instance, -23_f64);
        compute_acoth(&instance, 50_f64);

        assert!(instance.call(&[0]).is_err());
        assert!(instance.call(&[1]).is_err());
        assert!(instance.call(&[-1]).is_err());
        assert!(instance.call(&[10_f64, 3_f64, 7_f64]).is_err());
        assert!(instance.call(&empty_array::<f64>()).is_err());
    }
}
