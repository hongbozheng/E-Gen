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
./preprocess.py -h
```
##### Preprocess (deduplicate & refactor & verify)
1. Make sure all generated equivalent expressions `.txt` files have 2 `'\n'` characters at the end of the file
2. Move all generated equivalent expressions `.txt` files in a folder `<folder_name>`
3. Then rename them to be `equiv_exprs_<index>.txt`. For example, `equiv_exprs_0.txt`, `equiv_exprs_1.txt`,
`equiv_exprs_2.txt`, ...
4. Run the following command-line application
```
./preprocess -d <equiv_exprs_dir> -r <refactor> -v <verify>
```
- `<equiv_exprs_dir>` - folder `<folder_name>` that contains all generated equivalent expressions `.txt` files
- `<refactor>` - flag to indicate whether to refactor the expressions
- `<verify>` - flag to indicate whether to verify the expressions

The script will create the following three `.txt` files
1. `exprs.txt` - This file contains all the distinct generated original expressions
2. `equiv_exprs.txt` - This file contains all the distinct generated equivalent expressions
3. `duplicates.txt` - This file contains all the repetitive original expressions
4. `invalids.txt` - This file contains all the expressions with invalid domain

#### Create Dataset
##### Check command line input help
```
./create_dataset.py -h
```
##### Create raw dataset
Splitting all equivalent expression pairs into different classes & categories
```
./create_dataset.py
```
##### Create filtered dataset
1. Remove expressions with `0` equivalent expressions
2. Filter the ones with more than `<n_exprs>` equivalent
expressions
3. Create equivalent expression pairs
4. Classify all equivalent expression pairs into different classes & categories
```
./create_dataset.py -f <filter> -n <n_exprs>
```
- `<filter>` - flag to indicate whether to filter all generated equivalent expressions
- `<n_exprs>` - number of expressions to keep

#### Verify Dataset
##### Check command line input help
```
./verify.py -h
```
##### Verify dataset & Create verified dataset
1. Verify if an expression pair is equivalent **(Note: Verification is time consuming)**
2. Create verified dataset
3. Create incorrect dataset
```
./verify.py
```

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

#### Create Train, Validation, and Test Sets
##### Check command line input help
```
./split.py -h
```
##### Split dataset into train, validation, and test sets
```
./split.py -t <test_pct> -v <val_pct>
```
- `<test_pct>` - test set percentage
- `<val_pct>` - validation set percentage