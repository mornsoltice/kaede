use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum MathError {
    TipeError(String),
    ErrorDibagiNol,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MathError::TipeError(ref tipe) => write!(f, "Error: kamu memasukkan tipe data yang salah, seharusnya adalah {}", tipe),
            MathError::ErrorDibagiNol => write!(f, "Error: tidak bisa dibagikan dengan nol!"),
        }
    }
}

impl std::error::Error for MathError {}

pub fn tambah(a: i32, b: i32) -> Result<i32, MathError> {
    Ok(a + b)
}

pub fn kurang(a: i32, b: i32) -> Result<i32, MathError> {
    Ok(a - b)
}

pub fn kali(a: i32, b: i32) -> Result<i32, MathError> {
    Ok(a * b)
}

pub fn bagi(a: i32, b: i32) -> Result<f64, MathError> {
    if b == 0 {
        Err(MathError::ErrorDibagiNol)
    } else {
        Ok(a as f64 / b as f64)
    }
}

pub fn faktorial(angka: i32) -> Result<i32, MathError> {
    if angka < 0 {
        Err(MathError::TipeError("bilangan non-negatif".to_string()))
    } else {
        Ok((1..=angka).product())
    }
}

pub fn jumlah_deret_geometri(utama: i32, rasio_umum: i32, jumlah: i32) -> Result<f64, MathError> {
    if rasio_umum == 1 {
        Ok((jumlah * utama) as f64)
    } else {
        Ok((utama as f64 / (1.0 - rasio_umum as f64)) * (1.0 - (rasio_umum as f64).powi(jumlah)))
    }
}

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
                    .map(|(key, _)| key.parse().unwrap()) // Convert back to f64
                    .next()
                    .ok_or(MathError::TipeError("non-empty list".to_string()))?;
    if max_count == 1 {
        Err(MathError::TipeError("non-single mode".to_string()))
    } else {
        Ok(mode)
    }
}

pub fn normal_pdf(x: f64, mean: f64, sigma: f64) -> Result<f64, MathError> {
    if sigma <= 0.0 {
        Err(MathError::TipeError("positif sigma".to_string()))
    } else {
        let coefficient = 1.0 / (sigma * (2.0 * std::f64::consts::PI).sqrt());
        let exponent = -((x - mean).powi(2)) / (2.0 * sigma.powi(2));
        Ok(coefficient * exponent.exp())
    }
}

pub fn limit<F>(f: F, x: f64, epsilon: f64) -> Result<f64, MathError>
where
    F: Fn(f64) -> f64,
{
    let limit_left = f(x - epsilon);
    let limit_right = f(x + epsilon);
    if (limit_left - limit_right).abs() < std::f64::EPSILON {
        Ok((limit_left + limit_right) / 2.0)
    } else {
        Err(MathError::TipeError("Limit tidak terdefinisi".to_string()))
    }
}

pub fn integral<F>(f: F, a: f64, b: f64, n: usize) -> Result<f64, MathError>
where
    F: Fn(f64) -> f64,
{
    if n == 0 {
        return Err(MathError::TipeError("Jumlah interval tidak boleh nol".to_string()));
    }
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));
    for i in 1..n {
        sum += f(a + i as f64 * h);
    }
    Ok(sum * h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tambah() {
        assert_eq!(tambah(2, 3).unwrap(), 5);
    }

    #[test]
    fn test_kurang() {
        assert_eq!(kurang(5, 3).unwrap(), 2);
    }

    #[test]
    fn test_kali() {
        assert_eq!(kali(2, 3).unwrap(), 6);
    }

    #[test]
    fn test_bagi() {
        assert_eq!(bagi(6, 3).unwrap(), 2.0);
        assert!(matches!(bagi(6, 0).unwrap_err(), MathError::ErrorDibagiNol));
    }

    #[test]
    fn test_faktorial() {
        assert_eq!(faktorial(5).unwrap(), 120);
        assert!(matches!(faktorial(-1).unwrap_err(), MathError::TipeError(_)));
    }

    #[test]
    fn test_jumlah_deret_geometri() {
        assert_eq!(jumlah_deret_geometri(2, 2, 3).unwrap(), 14.0);
        assert_eq!(jumlah_deret_geometri(2, 1, 3).unwrap(), 6.0);
    }

    #[test]
    fn test_modus() {
        assert_eq!(modus(vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0]).unwrap(), 3.0);
        assert!(matches!(modus(vec![1.0]).unwrap_err(), MathError::TipeError(_)));
    }

    #[test]
    fn test_normal_pdf() {
        assert!((normal_pdf(0.0, 0.0, 1.0).unwrap() - 0.398942).abs() < 1e-6);
        assert!(matches!(normal_pdf(0.0, 0.0, 0.0).unwrap_err(), MathError::TipeError(_)));
    }

    #[test]
    fn test_limit() {
        let f = |x: f64| x * x;
        assert!((limit(f, 2.0, 1e-6).unwrap() - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_integral() {
        let f = |x: f64| x * x;
        assert!((integral(f, 0.0, 1.0, 1000).unwrap() - 1.0 / 3.0).abs() < 1e-4);
    }
}
