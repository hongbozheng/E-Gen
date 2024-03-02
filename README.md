## Equivalent Expressions Generation (Rust)
##### Build the command-line application
```
cargo clean && cargo build
```
The above command will create a binary executable files `egg` under `/target/debug`

##### Check command line input help
```
cargo run
```
##### Provide 1 input expression and equivalent expressions will be printed to terminal
```
cargo run -- -e <expression>
```
##### Provide a file with multiple expressions and a file to store all equivalent expressions
```
cargo run -- -i <input filepath> -o <output filepath>
```
##### Other optional hyperparameters
```
cargo run -- -p <n proc> -d <thd pct> -f <optim ext flag> -n <n equiv exprs> -l <token limit> -m <max token limit> -t <time limit>
```
- `<n proc>` - number of process
- `<thd pct>` - thread percentage
- `<optim ext flag>` - optimized extraction flag
- `<n equiv exprs>` - number of equivalent expressions
- `<token limit>` - token limit
- `<max token limit>` - maximum token limit
- `<time limit>` - time limit

##### Example command line inputs
```
cargo run -- -p 12 -d 0.8 -f -n 20 -l 10 -m 12 -t 300 -e "(cos x)"
```
```
cargo run -- -p 12 -d 0.8 -f -n 20 -l 10 -m 12 -t 300 -i "input/filepath" -o "output/filepath"
```

## Context-Grammar (Python)

#### Test Context-Sensitive Grammar
```
./main -c <csg>
```
`<csg>` context-sensitive grammar flag, use `1`

#### Test Context-Free Grammar
```
./main -c <csg>
```
`<csg>` context-sensitive grammar flag, use `0`

## Log Level Flag in `config.rs` or `config.py`
change `log_level = logger.LogLevel.XXXXX`
- `all = 6`
- `trace = 5`
- `debug = 4`
- `info = 3`
- `warn = 2`
- `error = 1`
- `fatal = 0`
- `off = -1`