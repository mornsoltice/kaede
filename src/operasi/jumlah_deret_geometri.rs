use crate::MathError;

pub fn jumlah_deret_geometri(utama: i32, rasio_umum: i32, jumlah: i32) -> Result<f64, MathError> {
    if rasio_umum == 1 {
        Ok((jumlah * utama) as f64)
    } else {
        Ok((utama as f64 / (1.0 - rasio_umum as f64)) * (1.0 - (rasio_umum as f64).powi(jumlah)))
    }
}
