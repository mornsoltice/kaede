use crate::MathError;

pub fn kurang(a: i32, b: i32) -> Result<i32, MathError> {
    Ok(a - b)
}
