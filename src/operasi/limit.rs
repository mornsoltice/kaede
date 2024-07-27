use crate::MathError;

pub fn limit<F>(f: F, x: f64, epsilon: f64) -> Result<f64, MathError>
where
    F: Fn(f64) -> f64,
{
    let limit_left = f(x - epsilon);
    let limit_right = f(x + epsilon);
    let tolerance = 1e-6;

    println!("Limit left: {}", limit_left);
    println!("Limit right: {}", limit_right);
    println!("Tolerance: {}", tolerance);

    if (limit_left - limit_right).abs() < tolerance {
        Ok((limit_left + limit_right) / 2.0)
    } else {
        Err(MathError::TipeError("Limit tidak terdefinisi".to_string()))
    }
}
