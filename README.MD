# Kaede
Rust library untuk operasi matematika sederhana, dan akan terus di update mungkin!.

## Operasi Yang Didukung
- **Operasi Dasar(Pertambahan, Pengurangan, Perkalian, Pembagian)**
- **Integral**
- **Limit(Polinomial)**
- **Modus**
- **Deret Geometri**
- **Faktorial**
- **Fungsi PDF Normal**


## Instalasi
Tambahkan dependensi ini ke file `Cargo.toml` Anda:

```toml
[dependencies]
kaede = "0.1"
```

## Contoh Kode
```rust
use kaede::*;

fn main() {
    let a = 5;
    let b = 3;

    match tambah(a, b) {
        Ok(result) => println!("Hasil penjumlahan: {}", result),
        Err(e) => eprintln!("Terjadi kesalahan: {}", e),
    }

    match bagi(a, b) {
        Ok(result) => println!("Hasil pembagian: {}", result),
        Err(e) => eprintln!("Terjadi kesalahan: {}", e),
    }
}
```