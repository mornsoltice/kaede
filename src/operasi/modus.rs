use crate::MathError;
use std::collections::HashMap;

pub fn modus(arr: Vec<f64>) -> Result<f64, MathError> {
    let mut count = HashMap::new();
    for &value in &arr {
        let rounded_value = (value * 1e6).round() / 1e6; 
        let key = rounded_value.to_string();
        *count.entry(key).or_insert(0) += 1;
    }
    let max_count = count.values().cloned().max().unwrap_or(0);
    let mode = count.into_iter()
                    .filter(|&(_, count)| count == max_count)
                    .map(|(key, _)| key.parse().unwrap()) 
                    .next()
                    .ok_or(MathError::TipeError("non-empty list".to_string()))?;
    if max_count == 1 {
        Err(MathError::TipeError("non-single mode".to_string()))
    } else {
        Ok(mode)
    }
}
