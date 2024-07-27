use kaede::*; 

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
fn test_integral() {
    let f = |x: f64| x * x;
    assert!((integral(f, 0.0, 1.0, 1000).unwrap() - 1.0 / 3.0).abs() < 1e-4);
}

#[test]
fn test_akar_kuadrat() {
    assert_eq!(akar_kuadrat(9.0).unwrap(), 3.0);
    assert!(matches!(akar_kuadrat(-1.0).unwrap_err(), MathError::TipeError(_)));
}

#[test]
fn test_logaritma_natural() {
    assert!((logaritma_natural(1.0).unwrap() - 0.0).abs() < 1e-6);
    assert!(matches!(logaritma_natural(0.0).unwrap_err(), MathError::TipeError(_)));
}

#[test]
fn test_logaritma() {
    assert!((logaritma(100.0, 10.0).unwrap() - 2.0).abs() < 1e-6);
    assert!(matches!(logaritma(100.0, 1.0).unwrap_err(), MathError::TipeError(_)));
}

#[test]
fn test_sinus() {
    assert!((sinus(0.0) - 0.0).abs() < 1e-6);
    assert!((sinus(std::f64::consts::PI / 2.0) - 1.0).abs() < 1e-6);
}

#[test]
fn test_kosinus() {
    assert!((kosinus(0.0) - 1.0).abs() < 1e-6);
    assert!((kosinus(std::f64::consts::PI) + 1.0).abs() < 1e-6);
}

#[test]
fn test_tangen() {
    assert!((tangen(0.0) - 0.0).abs() < 1e-6);
    assert!((tangen(std::f64::consts::PI / 4.0) - 1.0).abs() < 1e-6);
}

#[test]
fn test_modulo() {
    assert_eq!(modulo(10, 3).unwrap(), 1);
    assert_eq!(modulo(10, 5).unwrap(), 0);
    assert!(matches!(modulo(10, 0), Err(MathError::ErrorDibagiNol)));
}

#[test]
fn test_pangkat_positive_exponent() {
    assert_eq!(pangkat(2.0, 3.0).unwrap(), 8.0);
    assert_eq!(pangkat(5.0, 2.0).unwrap(), 25.0);
}

#[test]
fn test_pangkat_negative_exponent() {
    assert!((pangkat(2.0, -3.0).unwrap() - 0.125).abs() < 1e-6);
    assert!((pangkat(5.0, -2.0).unwrap() - 0.04).abs() < 1e-6);
}

#[test]
fn test_pangkat_zero_exponent() {
    assert_eq!(pangkat(2.0, 0.0).unwrap(), 1.0);
    assert_eq!(pangkat(5.0, 0.0).unwrap(), 1.0);
}

#[test]
fn test_pangkat_zero_base() {
    assert_eq!(pangkat(0.0, 3.0).unwrap(), 0.0);
    assert_eq!(pangkat(0.0, 1.0).unwrap(), 0.0);
}

#[test]
fn test_pangkat_zero_base_negative_exponent() {
    assert!(matches!(pangkat(0.0, -1.0), Err(MathError::TipeError(_))));
    assert!(matches!(pangkat(0.0, -3.0), Err(MathError::TipeError(_))));
}

#[test]
fn test_pangkat_negative_base_positive_exponent() {
    assert_eq!(pangkat(-2.0, 2.0).unwrap(), 4.0);
    assert_eq!(pangkat(-3.0, 2.0).unwrap(), 9.0);
}

#[test]
fn test_pangkat_negative_base_negative_exponent() {
    assert!((pangkat(-2.0, -2.0).unwrap() - 0.25).abs() < 1e-6);
    assert!((pangkat(-3.0, -2.0).unwrap() - 0.111111).abs() < 1e-6);
}

#[test]
fn test_pangkat_fractional_exponent() {
    assert!((pangkat(9.0, 0.5).unwrap() - 3.0).abs() < 1e-6);
    assert!((pangkat(27.0, 1.0/3.0).unwrap() - 3.0).abs() < 1e-6);
}

fn test_simplifikasi_simple() {
    assert_eq!(simplifikasi("2x + 3x"), "5x");
    assert_eq!(simplifikasi("x + x"), "2x");
}

#[test]
fn test_simplifikasi_negative() {
    assert_eq!(simplifikasi("x - x"), "0");
    assert_eq!(simplifikasi("2x - 3x"), "-x");
}

#[test]
fn test_simplifikasi_multiple_variables() {
    assert_eq!(simplifikasi("2x + 3y + x + y"), "3x+4y");
    assert_eq!(simplifikasi("2a + 3b - a - b"), "a+2b");
}

#[test]
fn test_simplifikasi_with_constants() {
    assert_eq!(simplifikasi("2x + 3"), "2x");
    assert_eq!(simplifikasi("3 + 4x - x - 3"), "3x");
}