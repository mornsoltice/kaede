use crate::MathError;

pub fn modulo(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::ErrorDibagiNol)
    } else {
        Ok(a % b)
    }
}
