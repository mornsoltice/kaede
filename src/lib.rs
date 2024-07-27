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
    if (limit_left - limit_right).abs() < epsilon {
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

pub fn jumlah_deret_aritmatika(awal: i32, rasio: i32, jumlah: i32) -> Result<i32, MathError> {
    if jumlah <= 0 {
        return Err(MathError::TipeError("jumlah anggota harus positif".to_string()));
    }
    let akhir = awal + (jumlah - 1) * rasio;
    let total = jumlah * (awal + akhir) / 2;
    Ok(total)
}

pub fn akar_kuadrat(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::TipeError("bilangan non-negatif".to_string()))
    } else {
        Ok(x.sqrt())
    }
}

pub fn logaritma_natural(x: f64) -> Result<f64, MathError> {
    if x <= 0.0 {
        Err(MathError::TipeError("bilangan positif".to_string()))
    } else {
        Ok(x.ln())
    }
}

pub fn logaritma(x: f64, basis: f64) -> Result<f64, MathError> {
    if x <= 0.0 || basis <= 0.0 || basis == 1.0 {
        Err(MathError::TipeError("bilangan positif dan basis tidak sama dengan 1".to_string()))
    } else {
        Ok(x.log(basis))
    }
}

pub fn sinus(x: f64) -> f64 {
    x.sin()
}

pub fn kosinus(x: f64) -> f64 {
    x.cos()
}

pub fn tangen(x: f64) -> f64 {
    x.tan()
}