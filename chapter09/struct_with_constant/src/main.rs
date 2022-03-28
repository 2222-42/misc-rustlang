use std::f64::consts::FRAC_PI_2;

struct Polynominal<const N: usize> {
    coefficients: [f64; N],
}

impl<const N: usize> Polynominal<N> {
    fn new(coefficients: [f64; N]) -> Self {
        Self { coefficients }
    }

    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in 0..N {
            sum += self.coefficients[i] * x.powi(i as i32);
        }
        sum
    }
}

fn main() {
    let sine_poly = Polynominal::new([0.0, 1.0, 0.0, -1.0 / 6.0, 0.0, 1.0 / 120.0]);
    assert_eq!(sine_poly.eval(0.0), 0.0);
    assert!((sine_poly.eval(FRAC_PI_2) - 1.0).abs() < 0.005);
}
