use crate::MathError;
use num::complex::Complex;

pub fn akar_kuadrat(x: f64) -> Result<Complex<f64>, MathError> {
    if x < 0.0 {
        Ok(Complex::new(0.0, (-x).sqrt()))
    } else {
        Ok(Complex::new(x.sqrt(), 0.0))
    }
}
