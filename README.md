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
cargo run
```
Provide 1 input expression and the generated equivalent expressions will be 
printed to terminal.
```
cargo run -- -e <expression>
```
Provide a file with multiple expressions and a file to store all generated 
equivalent expressions.
```
cargo run -- -i <input filepath> -o <output filepath>
```
Other optional hyperparameters.
```
cargo run -- -f <optim ext flag> -n <n equiv exprs> -l <init token limit> -m <max token limit> -t <init time limit>
```
- `<optim ext flag>` - optimized (faster) extraction flag
- `<n equiv exprs>` - number of equivalent expressions
- `<init token limit>` - initial token limit
- `<max token limit>` - maximum token limit
- `<init time limit>` - initial time limit

If the number of generated expressions (`<n equiv exprs>`) is not reached 
during extraction, the token limit (`<init token limit>`) will +1 
and the time limit will +300s to keep generating equivalent expressions. 
When the max token limit (`<max token limit>`) is reached, the generation 
will stop.

Example command line inputs.
```
cargo run -- -f -n 20 -l 10 -m 12 -t 300 -e "(cos x)"
```
```
cargo run -- -f -n 20 -l 10 -m 12 -t 300 -i "input/filepath" -o "output/filepath"
```
