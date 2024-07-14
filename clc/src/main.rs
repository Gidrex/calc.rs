use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: clc <expression>");
        return;
    }

    let expression = args.join(" ");
    let result = evaluate(&expression);

    match result {
        Some(value) => println!("{}", value),
        None => eprintln!("Error in expression"),
    }
}

fn evaluate(expression: &str) -> Option<f64> {
    let re = Regex::new(r"^\s*(sin|cos|ln|log10|exp|sqrt|abs)?\s*(\d+\.?\d*)\s*([+\-*/%^])?\s*(\d+\.?\d*)?\s*$").unwrap();
    if let Some(caps) = re.captures(expression) {
        let left_operand: f64 = caps.get(2)?.as_str().parse().ok()?;
        let operator = caps.get(3).map_or("", |m| m.as_str());
        let right_operand: f64 = if operator.is_empty() && (caps.get(1)?.as_str() == "sin" || caps.get(1)?.as_str() == "cos" || caps.get(1)?.as_str() == "ln" || caps.get(1)?.as_str() == "log10" || caps.get(1)?.as_str() == "exp" || caps.get(1)?.as_str() == "sqrt" || caps.get(1)?.as_str() == "abs") {
            left_operand
        } else {
            caps.get(4)?.as_str().parse().ok()?
        };

        let result = match caps.get(1).map_or("", |m| m.as_str()) {
            "sin" => taylor_sin(left_operand),
            "cos" => taylor_cos(left_operand),
            "ln" => left_operand.ln(),
            "log10" => left_operand.log10(),
            "exp" => left_operand.exp(),
            "sqrt" => left_operand.sqrt(),
            "abs" => left_operand.abs(),
            _ => match operator {
                "+" => left_operand + right_operand,
                "-" => left_operand - right_operand,
                "*" => left_operand * right_operand,
                "/" => left_operand / right_operand,
                "%" => (left_operand as i64 % right_operand as i64) as f64,
                "^" => left_operand.powf(right_operand),
                _ => return None,
            },
        };

        Some(result)
    } else {
        None
    }
}

fn taylor_sin(x: f64) -> f64 {
    let x = x.to_radians();
    let mut term: f64 = x;
    let mut sum: f64 = 0.0;
    let mut n = 1;

    while term.abs() > 1e-10 {
        sum += term;
        term *= -x * x / ((2 * n) * (2 * n + 1)) as f64;
        n += 1;
    }

    sum
}

fn taylor_cos(x: f64) -> f64 {
    let x = x.to_radians();
    let mut term: f64 = 1.0;
    let mut sum: f64 = 0.0;
    let mut n = 1;

    while term.abs() > 1e-10 {
        sum += term;
        term *= -x * x / ((2 * n - 1) * (2 * n)) as f64;
        n += 1;
    }

    sum
}
