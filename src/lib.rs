mod operasi;

pub use operasi::tambah::tambah;
pub use operasi::kurang::kurang;
pub use operasi::kali::kali;
pub use operasi::bagi::bagi;
pub use operasi::faktorial::faktorial;
pub use operasi::jumlah_deret_geometri::jumlah_deret_geometri;
pub use operasi::modus::modus;
pub use operasi::normal_pdf::normal_pdf;
pub use operasi::integral::integral;
pub use operasi::jumlah_deret_aritmatika::jumlah_deret_aritmatika;
pub use operasi::akar_kuadrat::akar_kuadrat;
pub use operasi::logaritma_natural::logaritma_natural;
pub use operasi::logaritma::{logaritma, log10, log2};
pub use operasi::sinus::sinus;
pub use operasi::kosinus::kosinus;
pub use operasi::tangen::tangen;
pub use operasi::modulo::modulo;
pub use operasi::pangkat::pangkat;
pub use operasi::simplifikasi::simplifikasi;
pub use operasi::roots::{akar_empat, akar_kubik};
pub use operasi::matrix::{tambah_matriks, kurang_matriks, kali_matriks, transpose_matriks};
pub use operasi::trigonometry::{cot, sec, csc};

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
