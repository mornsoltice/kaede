use crate::MathError;
use std::f64::consts::PI;

pub enum Function {
    Polynomial(Vec<f64>), // Koefisien untuk fungsi polinomial
    Trigonometric(TrigonometricFunction),
}

pub enum TrigonometricFunction {
    Sin,
    Cos,
}

pub struct Limit {
    func: Function,
    point: f64,
    epsilon: f64,
}

impl Limit {
    pub fn new(func: Function, point: f64, epsilon: f64) -> Self {
        Limit {
            func,
            point,
            epsilon,
        }
    }

    pub fn evaluate(&self) -> Result<f64, MathError> {
        // Mengevaluasi fungsi di sekitar titik x
        let limit_left = self.evaluate_at(self.point - self.epsilon);
        let limit_right = self.evaluate_at(self.point + self.epsilon);

        let tolerance = 1e-6;

        // Memeriksa apakah limit ada
        if (limit_left - limit_right).abs() < tolerance {
            Ok((limit_left + limit_right) / 2.0)
        } else {
            Err(MathError::TipeError("Limit tidak terdefinisi".to_string()))
        }
    }

    fn evaluate_at(&self, x: f64) -> f64 {
        match &self.func {
            Function::Polynomial(coeffs) => self.evaluate_polynomial(coeffs, x),
            Function::Trigonometric(trig_func) => self.evaluate_trigonometric(trig_func, x),
        }
    }

    fn evaluate_polynomial(&self, coeffs: &[f64], x: f64) -> f64 {
        coeffs.iter().rev().fold(0.0, |acc, &coeff| acc * x + coeff)
    }

    fn evaluate_trigonometric(&self, trig_func: &TrigonometricFunction, x: f64) -> f64 {
        match trig_func {
            TrigonometricFunction::Sin => x.sin(),
            TrigonometricFunction::Cos => x.cos(),
        }
    }
}
