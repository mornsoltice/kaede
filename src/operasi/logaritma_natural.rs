use crate::MathError;

pub fn logaritma_natural(x: f64) -> Result<f64, MathError> {
    if x <= 0.0 {
        Err(MathError::TipeError("bilangan positif".to_string()))
    } else {
        Ok(x.ln())
    }
}
