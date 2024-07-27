use crate::MathError;

pub fn limit<F>(f: F, x: f64, epsilon: f64) -> Result<f64, MathError>
where
    F: Fn(f64) -> f64,
{
    let limit_left = f(x - epsilon);
    let limit_right = f(x + epsilon);

    let tolerance = 1e-6;

    let value_tolerance = 1e-6;

    if (limit_left - limit_right).abs() < tolerance {
        Ok((limit_left + limit_right) / 2.0)
    } else {
        let average_limit = (limit_left + limit_right) / 2.0;
        let left_stable = (limit_left - average_limit).abs() < value_tolerance;
        let right_stable = (limit_right - average_limit).abs() < value_tolerance;

        if left_stable && right_stable {
            Ok(average_limit)
        } else {
            Err(MathError::TipeError("Limit tidak terdefinisi".to_string()))
        }
    }
}
