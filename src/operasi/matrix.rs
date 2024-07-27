pub type Matriks = Vec<Vec<f64>>;

pub fn tambah_matriks(a: &Matriks, b: &Matriks) -> Result<Matriks, String> {
    if a.len() != b.len() || a[0].len() != b[0].len() {
        return Err("Dimensi matriks harus sama".to_string());
    }
    let mut hasil = vec![vec![0.0; a[0].len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            hasil[i][j] = a[i][j] + b[i][j];
        }
    }
    Ok(hasil)
}

pub fn kurang_matriks(a: &Matriks, b: &Matriks) -> Result<Matriks, String> {
    if a.len() != b.len() || a[0].len() != b[0].len() {
        return Err("Dimensi matriks harus sama".to_string());
    }
    let mut hasil = vec![vec![0.0; a[0].len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            hasil[i][j] = a[i][j] - b[i][j];
        }
    }
    Ok(hasil)
}

pub fn kali_matriks(a: &Matriks, b: &Matriks) -> Result<Matriks, String> {
    if a[0].len() != b.len() {
        return Err("Jumlah kolom matriks pertama harus sama dengan jumlah baris matriks kedua".to_string());
    }
    let mut hasil = vec![vec![0.0; b[0].len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..a[0].len() {
                hasil[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    Ok(hasil)
}

pub fn transpose_matriks(matriks: &Matriks) -> Matriks {
    let mut hasil = vec![vec![0.0; matriks.len()]; matriks[0].len()];
    for i in 0..matriks.len() {
        for j in 0..matriks[0].len() {
            hasil[j][i] = matriks[i][j];
        }
    }
    hasil
}
