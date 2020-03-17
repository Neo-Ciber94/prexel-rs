use std::f64::consts;
use num_traits::ToPrimitive;

pub fn gamma(mut x: f64) -> f64 {
    //Using Coefficients from: https://mrob.com/pub/ries/lanczos-gamma.html
    const G : f64 = 4.7421875;
    const P: [f64; 15] = [
        0.99999999999999709182,
        57.156235665862923517,
        -59.597960355475491248,
        14.136097974741747174,
        -0.49191381609762019978,
        0.000033994649984811888699,
        0.000046523628927048575665,
        -0.000098374475304879564677,
        0.00015808870322491248884,
        -0.00021026444172410488319,
        0.0002174396181152126432,
        -0.00016431810653676389022,
        0.000084418223983852743293,
        -0.00002619083840158140867,
        0.0000036899182659531622704,
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
        for n in 1..P.len(){
            factor += P[n] / (x + n.to_f64().unwrap());
        }

        let t: f64 = x + G + 0.5;

        let result = f64::sqrt(2.0 * consts::PI)
            * f64::powf(t, x + 0.5)
            * f64::exp(-t)
            * factor;

        result
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