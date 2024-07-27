use crate::MathError;

pub fn pangkat(base: f64, exponent: f64) -> Result<f64, MathError> {
    if base == 0.0 && exponent <= 0.0 {
        Err(MathError::TipeError("Tidak dapat menghitung 0 pangkat nol atau negatif".to_string()))
    } else {
        Ok(base.powf(exponent))
    }
}
