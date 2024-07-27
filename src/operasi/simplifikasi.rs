use std::collections::HashMap;
use regex::Regex;

pub fn simplifikasi(expression: &str) -> String {
    let re = Regex::new(r"([+-]?\d*\.?\d*)([a-zA-Z]*)").unwrap();
    let mut terms: HashMap<String, f64> = HashMap::new();

    for cap in re.captures_iter(expression) {
        if cap[0].is_empty() { continue; }

        let coeff = match &cap[1] {
            "" | "+" => 1.0,
            "-" => -1.0,
            _ => cap[1].parse::<f64>().unwrap(),
        };
        let var = cap[2].to_string();
        *terms.entry(var).or_insert(0.0) += coeff;
    }

    let mut sorted_terms: Vec<_> = terms.into_iter().collect();
    sorted_terms.sort_by(|a, b| a.0.cmp(&b.0));

    let mut simplified_expression = String::new();
    for (var, coeff) in sorted_terms {
        if coeff == 0.0 {
            continue;
        }
        if coeff > 0.0 && !simplified_expression.is_empty() {
            simplified_expression.push('+');
        }
        if coeff == 1.0 && !var.is_empty() {
            simplified_expression.push_str(&format!("{}", var));
        } else if coeff == -1.0 && !var.is_empty() {
            simplified_expression.push_str(&format!("-{}", var));
        } else {
            simplified_expression.push_str(&format!("{}{}", coeff, var));
        }
    }

    if simplified_expression.is_empty() {
        "0".to_string()
    } else {
        simplified_expression
    }
}
