# E-Gen: Leveraging E-Graphs to Improve Continuous Representations of Symbolic Expressions

## Model
The transformer training and inference code can be found
[here](https://github.com/hongbozheng/transformer).

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
The above command will create a binary executable files `egen` under `/target/debug`.

Check command line input help.
```
cargo run --bin egen
```
Provide 1 input expression and the generated equivalent expressions will be
printed to terminal.
```
cargo run --bin egen -e <expression>
```
Provide a file with multiple expressions and a file to store all generated
equivalent expressions.
```
cargo run --bin egen -i <input filepath> -o <output filepath>
```
Other optional hyperparameters.
```
cargo run --bin egen -f <optim ext flag> -n <n equiv exprs> -l <init token limit> -m <max token limit> -t <init time limit>
```
- `<n proc>` - number of processes
- `<thd pct>` - OS thread percentage
- `<optim ext flag>` - optimized (faster) extraction flag
- `<n equiv exprs>` - number of equivalent expressions
- `<init token limit>` - initial token limit
- `<max token limit>` - maximum token limit
- `<init time limit>` - initial time limit

Example command line inputs.
```
cargo run --bin egen -f <optim ext flag> -n <n equiv exprs> -l <init token limit> -m <max token limit> -t <init time limit> -e <expression>
```
```
cargo run --bin egen -f <optim ext flag> -n <n equiv exprs> -l <init token limit> -m <max token limit> -t <init time limit> -i <input filepath> -o <output filepath>
```
