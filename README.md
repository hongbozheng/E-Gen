# Dataset

## Expressions Preparation

### Prefix Notation to Infix Notation (Rust)
Build the command-line application.
```
cargo clean && cargo build
```
The above command will create a binary executable files `eeg` under `/target/debug`.

Check command line input help.
```
cargo run
```
Convert from prefix notation to infix notation.
```
cargo run -- -f <op flag> -o <op> -i <input filepath> -c <convert filepath>
```
- `<op flag>` - operator flag
- `<op>` - operator
- `<input filepath>` - input filepath
- `<convert filepath>` - convert filepath

Example command line inputs.
- Convert from prefix notation to infix notation.
```
cargo run -- -i "input/filepath" -r "convert/filepath"
```
- Convert from prefix notation to infix notation with an additional operator.
```
cargo run -- -f -o "d x" -i "input/filepath" -r "convert/filepath"
```

### Replace character 'c' with random integer from 0-9
Check command line input help.
```
./const.py -h
```
Replace 'c' & any integer have more than 2 digits with random integer from 0-9.
```
./const.py -i <input filepath> -o <output filepath>
```
- `<input filepath>` - input filepath
- `<output filepath>` - output filepath

### Generate fundamental expressions
Check command line input help.
```
./fund_expr.py -h
```
Generate fundamental expressions.
```
./fund_expr.py -s <seed> -f <op flag> -o <op>
```
- `<seed>` - random seed
  - general: `42`
  - d: `84`
- `<op flag>` - operator flag
- `<op>` - operator

Example command line inputs.
- Generate general fundamental expressions.
```
./fund_expr.py -s 42
```
- Generate derivative fundamental expressions.
```
./fund_expr.py -s 84 -f -o "d x"
```

## Generate Dataset (Python)

### Generated Equivalent Expressions
- Make sure all generated equivalent expressions `.txt` files 
(`poly_1.txt`, `op_2.txt`, ...) have 2 `'\n'` characters at the end of the file.
- Move all generated equivalent expressions `.txt` files in a directory 
`<directory path>`.

### Preprocess
Check command line input help.
```
./preproc.py -h
```
Deduplicate & Convert & Verify.
```
./preproc -d <equiv exprs dir> -c <convert> -v <verify>
```
- `<equiv exprs dir>` - directory `<directory path>` that contains all generated 
equivalent expressions `.txt` files
- `<convert>` - flag to indicate whether to convert from symbolic operators 
('+', '*', ...) to string operators ('add', 'mul', ...)
- `<verify>` - flag to indicate whether to verify the expressions

The script will create the following 4 `.txt` files
(depends on the cli(s) provided)
1. `exprs.txt` - This file contains all the distinct generated original 
expressions.
2. `invalids.txt` - This file contains all the in-equivalent or invalid 
expressions.
3. `duplicates.txt` - This file contains all the repetitive original expressions.
4. `equiv_exprs_proc.txt` - This file contains all the processed equivalent 
expressions.

### Filter
Filter each group of equivalent expressions.
```
./filter.py
```

### Split
Check command line input help.
```
./split.py -h
```
Split the dataset into train set and validation set.
- `<pct>` - validation set percentage
- `<form>` - train set format (pair, triplet)

### Statistics
Check command line input help.
```
./stats.py -h
```
Calculate dataset statistics.
```
./stats.py -f <filepath>
```
- `<filepath>` - dataset filepath (e.g. `equiv_exprs_filter.txt`)
