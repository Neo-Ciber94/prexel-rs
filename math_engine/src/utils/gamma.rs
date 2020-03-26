use std::f64::consts;
use num_traits::ToPrimitive;

pub fn gamma(mut x: f64) -> f64 {
    //Using Coefficients from: https://mrob.com/pub/ries/lanczos-gamma.html
    const G : f64 = 4.742_187_5;
    const P: [f64; 15] = [
        0.999_999_999_999_997_091_82,
        57.156_235_665_862_923_517,
        -59.597_960_355_475_491_248,
        14.136_097_974_741_747_174,
        -0.491_913_816_097_620_199_78,
        0.000_033_994_649_984_811_888_699,
        0.000_046_523_628_927_048_575_665,
        -0.000_098_374_475_304_879_564_677,
        0.000_158_088_703_224_912_488_84,
        -0.000_210_264_441_724_104_883_19,
        0.000_217_439_618_115_212_643_2,
        -0.000_164_318_106_536_763_890_22,
        0.000_084_418_223_983_852_743_293,
        -0.000_026_190_838_401_581_408_67,
        0.000_003_689_918_265_953_162_270_4,
    ];

    // Using Lanczos approximation
    // ~10^-13 precision
    // See: https://en.wikipedia.org/wiki/Lanczos_approximation
    if x < 0.5 {
        consts::PI / (f64::sin(consts::PI * x) * gamma(1.0 - x))
    } else {
        // Lanczos solve gamma for (x + 1)
        x -= 1.0;

        let mut factor: f64 = P[0];
        for (n, coefficient) in P.iter().enumerate().skip(1){
            factor += coefficient / (x + n.to_f64().unwrap());
        }

        let t: f64 = x + G + 0.5;

        // Result = sqrt( 2 * PI) * t ^ (x + 1/2) * e^(-t) * factor
        f64::sqrt(2.0 * consts::PI)
            * f64::powf(t, x + 0.5)
            * f64::exp(-t)
            * factor
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn gamma_fact(n: f64) -> f64{
        gamma(n + 1.0)
    }

    fn f64_eq(a: f64, b: f64, delta: f64) -> bool{
        a.abs() - b.abs() < delta
    }

    #[test]
    fn gamma_test(){
        const ERROR : f64 = 0.00000000001;
        assert!(f64_eq(gamma_fact(0.9), 0.96176583190738741940757480212503, ERROR));
        assert!(f64_eq(gamma_fact(0.5), 0.88622692545275801364908374167057, ERROR));
        assert!(f64_eq(gamma_fact(0.1), 0.95135076986687318362924871772654, ERROR));

        //assert_eq!(gamma_fact(0.9), 0.96176583190738741940757480212503);
        //assert_eq!(gamma_fact(0.5), 0.88622692545275801364908374167057);
        //assert_eq!(gamma_fact(0.1), 0.95135076986687318362924871772654);
    }
}