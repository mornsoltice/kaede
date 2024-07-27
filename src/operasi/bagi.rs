use crate::MathError;

pub fn bagi(a: i32, b: i32) -> Result<f64, MathError> {
    if b == 0 {
        Err(MathError::ErrorDibagiNol)
    } else {
        Ok(a as f64 / b as f64)
    }
}
