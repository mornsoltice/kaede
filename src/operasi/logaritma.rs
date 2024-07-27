use crate::MathError;

pub fn logaritma(x: f64, basis: f64) -> Result<f64, MathError> {
    if x <= 0.0 || basis <= 0.0 || basis == 1.0 {
        Err(MathError::TipeError("bilangan positif dan basis tidak sama dengan 1".to_string()))
    } else {
        Ok(x.log(basis))
    }
}

pub fn log2(x: f64) -> Result<f64, MathError> {
    logaritma(x, 2.0)
}

pub fn log10(x: f64) -> Result<f64, MathError> {
    logaritma(x, 10.0)
}
