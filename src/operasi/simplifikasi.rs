use std::collections::HashMap;
use regex::Regex;

pub fn simplifikasi(expression: &str) -> String {
    let re = Regex::new(r"([+-]?\d*)([a-zA-Z]+)?").unwrap();
    let mut terms: HashMap<String, i32> = HashMap::new();
    let mut constant = 0;

    for cap in re.captures_iter(expression) {
        let coeff = match &cap[1] {
            "" | "+" => 1,
            "-" => -1,
            _ => cap[1].parse::<i32>().unwrap(),
        };

        if let Some(var) = cap.get(2) {
            let var = var.as_str().to_string();
            *terms.entry(var).or_insert(0) += coeff;
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

    simplified_expression.sort();
    let result = simplified_expression.join("+").replace("+-", "-");
    if result.is_empty() {
        "0".to_string()
    } else {
        result
    }
}
