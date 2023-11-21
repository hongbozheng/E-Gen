use crate::*;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::exit;

/// public function to add parentheses recursively
/// for each expression
/// ## Argument
/// * `tokens` - tokens from expression
/// ## Return
/// * `String` - expression or sub-expression
///              in Polish notation with parentheses
pub fn add_paren_recursive(tokens: &mut Vec<&str>) -> String {
    if tokens.is_empty() { return String::new(); }

    let token = tokens.remove(0);

    if token.chars().all(char::is_numeric) { return token.to_string(); }

    if token == "+" || token == "-" || token == "*" || token == "/" ||
        token == "ln" || token == "exp" || token == "pow" || token == "sqrt" ||
        token == "sin" || token == "cos" || token == "tan" ||
        token == "csc" || token == "sec" || token == "cot" ||
        token == "sinh" || token == "cosh" || token == "tanh" ||
        token == "asin" || token == "acos" || token == "atan" ||
        token == "asinh" || token == "acosh" || token == "atanh" {

        let operator = token;

        if operator == "+" || operator == "-" || operator == "*" ||
            operator == "/" || operator == "pow" {

            let operand_1 = add_paren_recursive(tokens);
            let operand_2 = add_paren_recursive(tokens);

            if operand_1.contains('-') {
                if let Some(index) = operand_1.find('-') {
                    let (first, second) = operand_1.split_at(index);
                    if first.chars().all(char::is_numeric) && second[1..].chars().all(char::is_numeric) {
                        if operand_2.contains('-') {
                            if let Some(index) = operand_2.find('-') {
                                let (first, second) = operand_2.split_at(index);
                                if first.chars().all(char::is_numeric) && second[1..].chars().all(char::is_numeric) {
                                    return "c".to_string();
                                }
                            }
                        } else if operand_2.chars().all(char::is_numeric) {
                            return "c".to_string();
                        }
                    }
                }
            } else if operand_1.chars().all(char::is_numeric) {
                if operand_2.contains('-') {
                    if let Some(index) = operand_2.find('-') {
                        let (first, second) = operand_2.split_at(index);
                        if first.chars().all(char::is_numeric) && second[1..].chars().all(char::is_numeric) {
                            return "c".to_string();
                        }
                    }
                } else if operand_2.chars().all(char::is_numeric) {
                    return "c".to_string();
                }
            }

            if (operand_1 == "c" && operand_2.chars().all(char::is_numeric)) ||
                (operand_1.chars().all(char::is_numeric) && operand_2 == "c") ||
                (operand_1 == "c" && operand_2 == "c") {
                return "c".to_string();
            }

            return format!("({} {} {})", operator, operand_1, operand_2);
        }

        let operand = add_paren_recursive(tokens);

        if operand.contains('-') {
            if let Some(index) = operand.find('-') {
                let (first, second) = operand.split_at(index);
                if first.chars().all(char::is_numeric) && second[1..].chars().all(char::is_numeric) {
                    return "c".to_string();
                }
            }
        } else if operand.chars().all(char::is_numeric) {
            return "c".to_string();
        }

        if operand == "c" {
            return "c".to_string();
        }
        return format!("({} {})", operator, operand);
    }

    token.to_string()
}

/// public function to add parenthese to expressions
/// ## Argument
/// * `expr` - expression to add parenthese
/// * `string` - expression in Polish notation with parenthese
pub fn add_paren(expr: &str) -> String {
    let mut tokens: Vec<&str> = expr.split_whitespace().collect();
    add_paren_recursive(&mut tokens)
}

/// public function to translate alphabetical operator
/// to mathematical operator
/// ## Argument
/// * `input_filename` - input filename
/// * `output_filename` - output filename
/// ## Return
/// * `std::io::Result<()>`
pub fn refactor() {
    let cli = parse_args();
    let add_op = &cli.flag;
    let op = match &cli.op {
        Some(op) => { op },
        None => { "" },
    };
    let input_filepath = &cli.input_filepath;
    let ref_filepath = &cli.ref_filepath;

    log_info(&format!("Start refactoring expressions from file '{}'.\n", input_filepath));

    // Open the input file and create output file
    let input_file = match File::open(input_filepath.clone()) {
        Ok(input_file) => { input_file },
        Err(e) => {
            log_error(&format!("Failed to open file '{}'.\n", input_filepath));
            log_error(&format!("{}\n", e));
            exit(1);
        },
    };

    let ref_file = match File::create(ref_filepath.clone()) {
        Ok(ref_file) => { ref_file },
        Err(e) => {
            log_error(&format!("Failed to create refactor file '{}'.\n", ref_filepath));
            log_error(&format!("{}\n", e));
            exit(1);
        },
    };

    // Create buffered reader and writer for the input and output files
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(ref_file);

    let mut exprs = HashSet::default();

    // Iterate over each line in the input file
    for expr in reader.lines() {
        let expr = match expr {
            Ok(expr) => { expr },
            Err(e) => {
                log_error("Failed to read expression from reader.\n");
                log_error(&format!("{}\n", e));
                exit(1);
            },
        };

        // Replace spaces between digits with no space
        let mut tokens = expr.split_whitespace().peekable();
        let mut new_expr = String::new();

        while let Some(token) = tokens.next() {
            if let Some(next_token) = tokens.peek() {
                if token.len() == 1 && next_token.len() == 1 &&
                    token.chars().all(|c| c.is_ascii_digit()) &&
                    next_token.chars().all(|c| c.is_ascii_digit()) {
                    new_expr.push_str(token);
                } else {
                    new_expr.push_str(token);
                    new_expr.push(' ');
                }
            } else { new_expr.push_str(token); }
        }

        // Replace alphabetical operator to mathematical operator
        let mut new_expr = new_expr
            .replace("add", "+")
            .replace("mul", "*")
            .replace("div", "/")
            .replace("INT+ ", "")
            .replace("INT- ", "-")
            .replace("pi", "3.14")
            .replace("abs ", "");

        new_expr = add_paren(&new_expr);

        // Write the updated line to the output file
        if *add_op {
            new_expr = format!("({} {})", op, new_expr);
        } else {
            new_expr = format!("{}", new_expr);
        }

        if exprs.contains(&new_expr) {
            continue;
        }
        exprs.insert(new_expr.clone());

        match writeln!(writer, "{}", new_expr) {
            Ok(_) => {},
            Err(e) => {
                log_error("Failed to write new expression into buffer.\n");
                log_error(&format!("{}\n", e));
                exit(1);
            },
        };
    }

    // Flush the writer to ensure that all data is written to the output file
    match writer.flush() {
        Ok(_) => {},
        Err(e) => {
            log_error(&format!("Failed to flush buffer to refactor file '{}'.\n", ref_filepath));
            log_error(&format!("{}\n", e));
            exit(1);
        },
    };

    log_info(&format!("Finish refactoring expressions, results saved to '{}'.\n", ref_filepath));
}