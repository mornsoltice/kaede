# Kaede
Rust library untuk operasi matematika sederhana, dan akan terus di update mungkin!.

## Operasi Yang Didukung
- **Operasi Dasar(Pertambahan, Pengurangan, Perkalian, Pembagian)**
- **Integral**
- **Modus**
- **Deret Geometri**
- **Faktorial**
- **Fungsi PDF Normal**
- **Modulo**
- **Logaritma & Logaritma Natural**
- **Trigonometri (Sin, Cos, Tan)**
- **Eksponen**


## Instalasi
Tambahkan dependensi ini ke file `Cargo.toml` Anda:

```toml
[dependencies]
kaede = "0.1"
```
atau bisa melakukan command
```sh
cargo add kaede
```
## Contoh Kode Pertambahan
```rust
let result = tambah(5, 3).unwrap();
println!("Hasil penjumlahan: {}", result); // Output: 8
```
## Contoh Kode Pengurangan
```rust
let result = kurang(5, 3).unwrap();
println!("Hasil pengurangan: {}", result); // Output: 2
```
## Contoh Kode Perkalian
```rust
let result = kali(5, 3).unwrap();
println!("Hasil perkalian: {}", result); // Output: 15
```
## Contoh Kode Pembagian
```rust
let result = bagi(6, 3).unwrap();
println!("Hasil pembagian: {}", result); // Output: 2.0
```
## Contoh Kode Integral
```rust
let f = |x: f64| x * x;
let result = integral(f, 0.0, 1.0, 1000).unwrap();
println!("Integral: {}", result); // Output: ~0.3333
```
## Contoh Kode Limit
```rust
let f = |x: f64| x * x;
let result = limit(f, 2.0, 1e-6).unwrap();
println!("Limit: {}", result); // Output: 4.0
```
## Jumlah Deret Geometri
```rust
let result = jumlah_deret_geometri(2, 2, 3).unwrap();
println!("Jumlah deret geometri: {}", result); // Output: 14.0
```
## Modus
```rust
let result = modus(vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0]).unwrap();
println!("Modus: {}", result); // Output: 3.0
```
## Fungsi PDF Normal
```rust
let result = normal_pdf(0.0, 0.0, 1.0).unwrap();
println!("PDF Normal: {}", result); // Output: ~0.398942
```
## Faktorial
```rust
let result = faktorial(5).unwrap();
println!("Hasil faktorial: {}", result); // Output: 120
```