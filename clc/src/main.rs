use regex::Regex;
use std::env;
use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: clc <expression>");
        return;
    }

    let expression = args.join("");
    let result = evaluate(&expression);

    match result {
        Some(value) => println!("{}", value),
        None => eprintln!("Error in expression"),
    }
}

fn evaluate(expression: &str) -> Option<f64> {
    let tokens = tokenize(expression)?;
    let rpn = shunting_yard(&tokens)?;
    calculate_rpn(&rpn)
}

fn tokenize(expression: &str) -> Option<Vec<String>> {
    let re = Regex::new(r"(sin|cos|ln|log10|exp|sqrt|abs|\d+\.?\d*|[+\-*/%^()])").unwrap();
    let mut tokens = Vec::new();
    for cap in re.captures_iter(expression) {
        tokens.push(cap[0].to_string());
    }
    Some(tokens)
}

fn shunting_yard(tokens: &[String]) -> Option<Vec<String>> {
    let mut output = Vec::new();
    let mut operators = VecDeque::new();

    for token in tokens {
        if token.parse::<f64>().is_ok() {
            output.push(token.clone());
        } else {
            match token.as_str() {
                "sin" | "cos" | "ln" | "log10" | "exp" | "sqrt" | "abs" => {
                    operators.push_back(token.clone());
                }
                "+" | "-" => {
                    while let Some(op) = operators.back() {
                        if op == "(" {
                            break;
                        }
                        if op == "+" || op == "-" || op == "*" || op == "/" || op == "%" || op == "^" {
                            output.push(operators.pop_back().unwrap());
                        } else {
                            break;
                        }
                    }
                    operators.push_back(token.clone());
                }
                "*" | "/" | "%" => {
                    while let Some(op) = operators.back() {
                        if op == "(" {
                            break;
                        }
                        if op == "*" || op == "/" || op == "%" || op == "^" {
                            output.push(operators.pop_back().unwrap());
                        } else {
                            break;
                        }
                    }
                    operators.push_back(token.clone());
                }
                "^" => {
                    operators.push_back(token.clone());
                }
                "(" => {
                    operators.push_back(token.clone());
                }
                ")" => {
                    while let Some(op) = operators.back() {
                        if op == "(" {
                            break;
                        }
                        output.push(operators.pop_back().unwrap());
                    }
                    operators.pop_back(); // Pop the "("
                    if let Some(op) = operators.back() {
                        if op == "sin" || op == "cos" || op == "ln" || op == "log10" || op == "exp" || op == "sqrt" || op == "abs" {
                            output.push(operators.pop_back().unwrap());
                        }
                    }
                }
                _ => return None,
            }
        }
    }

    while let Some(op) = operators.pop_back() {
        output.push(op);
    }

    Some(output)
}

fn calculate_rpn(rpn: &[String]) -> Option<f64> {
    let mut stack = Vec::new();

    for token in rpn {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            match token.as_str() {
                "sin" => {
                    let a = stack.pop()?;
                    stack.push(taylor_sin(a));
                },
                "cos" => {
                    let a = stack.pop()?;
                    stack.push(taylor_cos(a));
                },
                "ln" => {
                    let a = stack.pop()?;
                    stack.push(a.ln());
                },
                "log10" => {
                    let a = stack.pop()?;
                    stack.push(a.log10());
                },
                "exp" => {
                    let a = stack.pop()?;
                    stack.push(a.exp());
                },
                "sqrt" => {
                    let a = stack.pop()?;
                    stack.push(a.sqrt());
                },
                "abs" => {
                    let a = stack.pop()?;
                    stack.push(a.abs());
                },
                _ => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    let result = match token.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        "%" => (a as i64 % b as i64) as f64,
                        "^" => a.powf(b),
                        _ => return None,
                    };
                    stack.push(result);
                }
            }
        }
    }

    stack.pop()
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
