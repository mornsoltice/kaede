use crate::MathError;

pub fn akar_kuadrat(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::TipeError("bilangan non-negatif".to_string()))
    } else {
        Ok(x.sqrt())
    }
}
