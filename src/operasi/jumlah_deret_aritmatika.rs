use crate::MathError;

pub fn jumlah_deret_aritmatika(awal: i32, rasio: i32, jumlah: i32) -> Result<i32, MathError> {
    if jumlah <= 0 {
        return Err(MathError::TipeError("jumlah anggota harus positif".to_string()));
    }
    let akhir = awal + (jumlah - 1) * rasio;
    let total = jumlah * (awal + akhir) / 2;
    Ok(total)
}
