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
    for line in reader.lines() {
        let line = line?;

        // Replace spaces between digits with no space
        let mut words = line.split_whitespace().peekable();
        let mut new_line = String::new();
        let mut prev_is_digit = false;
        while let Some(word) = words.next() {
            if let Some(next_word) = words.peek() {
                if word.len() == 1 && next_word.len() == 1 &&
                    word.chars().all(|c| c.is_ascii_digit()) &&
                    next_word.chars().all(|c| c.is_ascii_digit()) {
                    new_line.push_str(word);
                } else {
                    new_line.push_str(word);
                    new_line.push(' ');
                }
            } else { new_line.push_str(word); }
        }

        // Replace alphabetical operator to mathematical operator
        let new_line = new_line
            .replace("add", "+")
            .replace("mul", "*")
            .replace("div", "/")
            .replace("INT+ ", "")
            .replace("INT- ", "-");

        // Write the updated line to the output file
        writeln!(writer, "{}", new_line)?;
    }

    // Flush the writer to ensure that all data is written to the output file
    writer.flush()?;

    Ok(())
}