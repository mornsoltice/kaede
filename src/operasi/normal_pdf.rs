use crate::MathError;

pub fn normal_pdf(x: f64, mean: f64, sigma: f64) -> Result<f64, MathError> {
    if sigma <= 0.0 {
        Err(MathError::TipeError("positif sigma".to_string()))
    } else {
        let coefficient = 1.0 / (sigma * (2.0 * std::f64::consts::PI).sqrt());
        let exponent = -((x - mean).powi(2)) / (2.0 * sigma.powi(2));
        Ok(coefficient * exponent.exp())
    }
}
