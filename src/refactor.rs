use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

/// public function to translate alphabetical operator
/// to mathematical operator
/// ## Argument
/// * `input_filename` - input filename
/// * `output_filename` - output filename
/// ## Return
/// * `std::io::Result<()>`
pub fn refactor(input_filename: &str, output_filename: &str) -> std::io::Result<()> {
    // Open the input and output files
    let input_file = File::open(input_filename)?;
    let output_file = File::create(output_filename)?;

    // Create buffered reader and writer for the input and output files
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    // Iterate over each line in the input file
    for expr in reader.lines() {
        let expr = expr?;

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
            .replace("INT- ", "-");

        new_expr = add_paren(&new_expr);

        // Write the updated line to the output file
        writeln!(writer, "{}", new_expr)?;
    }

    // Flush the writer to ensure that all data is written to the output file
    writer.flush()?;

    Ok(())
}

/// public function to add parentheses recursively
/// for each expression
/// ## Argument
/// * `tokens` - tokens from expression
/// ## Return
/// * `String` - expression or sub-expression
///              in Polish notation with parentheses
pub fn add_paren_recursive(tokens: &mut Vec<&str>) -> String {
    if tokens.is_empty() {
        return String::new();
    }

    let token = tokens.remove(0);

    if token.chars().all(char::is_numeric) {
        return token.to_string();
    }

    if token == "+" || token == "-" || token == "*" || token == "/" ||
        token == "ln" || token == "exp" || token == "pow" || token == "sqrt" ||
        token == "sin" || token == "cos" || token == "tan" ||
        token == "sinh" || token == "cosh" || token == "tanh" ||
        token == "asin" || token == "acos" || token == "atan" ||
        token == "asinh" || token == "acosh" || token == "atanh" {

        let operator = token;

        if operator == "+" || operator == "-" || operator == "*" ||
            operator == "/" || operator == "pow" {

            let operand_1 = add_paren_recursive(tokens);
            let operand_2 = add_paren_recursive(tokens);
            return format!("({} {} {})", operator, operand_1, operand_2);
        }

        let operand = add_paren_recursive(tokens);
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