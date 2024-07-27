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
fn test_limit() {
    let f = |x: f64| x * x;
    let epsilon = 1e-6; 
    let result = limit(f, 2.0, epsilon).unwrap();
    assert!((result - 4.0).abs() < 1e-6, "Expected 4.0 but got {}", result);
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
