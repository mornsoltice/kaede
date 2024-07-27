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
fn test_limit_polynomial() {
        let f = |x: f64| x * x; // Fungsi polinomial f(x) = x^2
        let result = limit(f, 2.0, 1e-6).unwrap();
        assert!((result - 4.0).abs() < 1e-6, "Expected 4.0 but got {}", result); // Limit f(x) saat x mendekati 2 adalah 4
    }

#[test]
fn test_limit_trigonometric() {
        let f = |x: f64| (x.sin() - x) / (x * x); // Fungsi trigonometri f(x) = (sin(x) - x) / x^2
        let result = limit(f, 0.0, 1e-6).unwrap();
        assert!((result - (-1.0 / 6.0)).abs() < 1e-6, "Expected -1/6 but got {}", result); // Limit f(x) saat x mendekati 0 adalah -1/6
    }

#[test]
fn test_limit_logarithmic() {
    let f = |x: f64| (x.ln() - 1.0) / x; // Fungsi logaritma f(x) = (ln(x) - 1) / x
    let result = limit(f, 1.0, 1e-6).unwrap();
    assert!((result - 0.0).abs() < 1e-6, "Expected 0 but got {}", result); // Limit f(x) saat x mendekati 1 adalah 0
}
#[test]
fn test_limit_exponential() {
    let f = |x: f64| (x.exp() - x - 1.0) / (x * x); // Fungsi eksponensial f(x) = (exp(x) - x - 1) / x^2
    let result = limit(f, 0.0, 1e-6).unwrap();
    assert!((result - 0.5).abs() < 1e-6, "Expected 0.5 but got {}", result); // Limit f(x) saat x mendekati 0 adalah 0.5
}
#[test]
fn test_limit_nonexistent() {
    let f = |x: f64| if x < 1.0 { x } else { -x }; // Fungsi yang tidak memiliki limit terdefinisi di x = 1
    let result = limit(f, 1.0, 1e-6);
    assert!(matches!(result, Err(MathError::TipeError(_))));
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
