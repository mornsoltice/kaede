use std::collections::HashMap;
use regex::Regex;

pub fn simplifikasi(expression: &str) -> String {
    let re = Regex::new(r"([+-]?\d*)([a-zA-Z]*)").unwrap();
    let mut terms: HashMap<String, i32> = HashMap::new();
    let mut constant = 0;

    for cap in re.captures_iter(expression) {
        if cap[0].is_empty() {
            continue;
        }

        let coeff = match &cap[1] {
            "" | "+" => 1,
            "-" => -1,
            _ => cap[1].parse::<i32>().unwrap_or(0),
        };

        if let Some(var) = cap.get(2) {
            if var.as_str().is_empty() {
                constant += coeff;
            } else {
                let var = var.as_str().to_string();
                *terms.entry(var).or_insert(0) += coeff;
            }
        } else {
            constant += coeff;
        }
    }

    let mut simplified_expression = terms.iter()
        .filter(|&(_, &v)| v != 0)
        .map(|(k, &v)| {
            if v == 1 {
                k.clone()
            } else if v == -1 {
                format!("-{}", k)
            } else {
                format!("{}{}", v, k)
            }
        })
        .collect::<Vec<String>>();

    if constant != 0 {
        simplified_expression.push(constant.to_string());
    }

    simplified_expression.sort_by(|a, b| {
        if a.chars().all(char::is_numeric) {
            std::cmp::Ordering::Greater
        } else if b.chars().all(char::is_numeric) {
            std::cmp::Ordering::Less
        } else {
            a.cmp(b)
        }
    });

    let result = simplified_expression.join("+").replace("+-", "-");
    if result.is_empty() {
        "0".to_string()
    } else {
        result
    }
}