# E-Gen: Leveraging E-Graphs to Improve Continuous Representations of Symbolic Expressions

## Environment Setup
Install Rust using [`rustup`](https://www.rust-lang.org/tools/install).

## Equivalent Expressions Generation

### Generation Configuration
To modify generation configuration, check `config.py` file.

### Generation
Build the command-line application.
```
cargo clean && cargo build
```
The above command will create a binary executable files `eeg` under `/target/debug`.

Check command line input help.
```
cargo run --bin eeg
```
Provide 1 input expression and the generated equivalent expressions will be
printed to terminal.
```
cargo run --bin eeg -e <expression>
```
Provide a file with multiple expressions and a file to store all generated
equivalent expressions.
```
cargo run --bin eeg -i <input filepath> -o <output filepath>
```
Other optional hyperparameters.
```
cargo run --bin eeg -t <thd pct> -l <token limit> -f <exhaustive flag>
```
* `<thd pct>` - OS thread percentage
* `<token limit>` - token limit
* `<exhaustive>` - exhaustive extraction (slow) flag

Example command line inputs.
```
cargo run --bin eeg -t <thd pct> -l <token limit> -f <csg flag> -e <expression>
```
```
cargo run --bin eeg -t <thd pct> -l <token limit> -f <csg flag> -i <input filepath> -o <output filepath>
```
