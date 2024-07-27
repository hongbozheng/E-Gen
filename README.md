# Dataset

## Expressions Preparation
#### Refactor Expressions from Expembtx Syntax to Egg Syntax (Rust)
##### Build the command-line application
```
cargo clean && cargo build
```
The above command will create a binary executable files `egg` under `/target/debug`

##### Check command line input help
```
cargo run
```
##### Provide operator flag, operator, input filepath and refactor filepath
```
cargo run -- -f <operator flag> -o <operator> -i <input filepath> -r <refactor filepath>
```
- `<operator flag>` - operator flag
- `<operator>` - operator
- `<input filepath>` - input filepath
- `<refactor filepath>` - refactor filepath

##### Example command line inputs
- Refactor from `expembtx` syntax to `egg` syntax
```
cargo run -- -i "input/filepath" -r "refactor/filepath"
```
- Refactor from `expembtx` syntax to `egg` syntax with an additional operator
```
cargo run -- -f -o "d x" -i "input/filepath" -r "refactor/filepath"
```

#### Replace character 'c' with random integer from 0-9
##### Check command line input help
```
./const.py -h
```
##### Replace 'c' & any integer have more than 2 digits with random integer from 0-9
```
./const.py -i <input_filepath> -o <output_filepath>
```
- `<input_filepath>` - input filepath
- `<output_filepath>` - output filepath

#### Generate fundamental expressions
##### Check command line input help
```
./fund_expr.py -h
```
##### Generate fundamental expressions
```
./fund_expr.py -s <seed> -f <op_flag> -o <operator>
```
- `<seed>` - random seed
  - general: `42`
  - d: `84`
- `op_flag` - operator flag
- `operator` - operator str

##### Example command line inputs
- Generate general fundamental expressions
```
./fund_expr.py -s 42
```
- Generate derivative fundamental expressions
```
./fund_expr.py -s 84 -f -o "d x"
```

## Generate Dataset (Python)
#### Preprocess
##### Check command line input help
```
./preproc.py -h
```
##### Deduplicate & Refactor & Verify
1. Make sure all generated equivalent expressions `.txt` files (`poly_1.txt`, `op_2.txt`, ...) have 2 `'\n'` characters at the end of the file
2. Move all generated equivalent expressions `.txt` files in a folder `<folder_name>`
3. Run the following command-line application
```
./preproc -d <equiv_exprs_dir> -r <refactor> -v <verify>
```
- `<equiv_exprs_dir>` - folder `<folder_name>` that contains all generated equivalent expressions `.txt` files
- `<refactor>` - flag to indicate whether to refactor the expressions
- `<verify>` - flag to indicate whether to verify the expressions

The script will create the following 5 `.txt` files (depends on the cli(s) provided)
1. `exprs.txt` - This file contains all the distinct generated original expressions
2. `invalids.txt` - This file contains all the expressions with invalid domain
3. `duplicates.txt` - This file contains all the repetitive original expressions
4. `equiv_exprs_proc.txt` - This file contains all the processed equivalent expressions

#### Filter
##### Filter each block of equivalent expressions
```
./filter.py
```

#### Split into Train Set and Val Set
##### Check command line input help
```
./split.py -h
```
- `<pct>` - validation set percentage
- `<form>` - train set format

#### Statistics
##### Check command line input help
```
./stats.py -h
```
##### Calculate dataset statistics
```
./stats.py -d <dataset_dir>
```
- `<dataset_dir>` - dataset directory
