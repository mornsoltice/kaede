use crate::MathError;

pub fn faktorial(angka: i32) -> Result<i32, MathError> {
    if angka < 0 {
        Err(MathError::TipeError("bilangan non-negatif".to_string()))
    } else {
        Ok((1..=angka).product())
    }
}
