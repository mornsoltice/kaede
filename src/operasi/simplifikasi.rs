use std::collections::HashMap;
use regex::Regex;

pub fn simplifikasi(expression: &str) -> String {
    let re = Regex::new(r"([+-]?\d*)([a-zA-Z]+)").unwrap();
    let mut terms: HashMap<String, i32> = HashMap::new();

    for cap in re.captures_iter(expression) {
        let coeff = match &cap[1] {
            "" | "+" => 1,
            "-" => -1,
            _ => cap[1].parse::<i32>().unwrap(),
        };
        let var = cap[2].to_string();
        *terms.entry(var).or_insert(0) += coeff;
    }

    let mut simplified_expression = String::new();
    for (var, coeff) in terms {
        if coeff > 0 && !simplified_expression.is_empty() {
            simplified_expression.push('+');
        }
        if coeff == 1 {
            simplified_expression.push_str(&format!("{}", var));
        } else if coeff == -1 {
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
