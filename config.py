import logger
import random


# -----------------------------------------------------------------------------
# Functions
# -----------------------------------------------------------------------------
ARITH_OPS = ["add", "sub", "mul", "div", "pow", "sqrt", "abs",]
UNARY_ARITH_OPS = ["sqrt", "abs",]
BINARY_ARITH_OPS = ["add", "sub", "mul", "div", "pow",]
FUNC_OPS = [
    "ln",
    "sin", "cos", "tan",
    "csc", "sec", "cot",
    "asin", "acos", "atan",
    "acsc", "asec", "acot",
    "sinh", "cosh", "tanh",
    "csch", "sech", "coth",
    "asinh", "acosh", "atanh",
    "acsch", "asech", "acoth",
]
CATEGORIES = ["general", "d",]


# -----------------------------------------------------------------------------
# Sympy Verify
# -----------------------------------------------------------------------------
MATH_OPERATORS = {
    # Elementary functions
    "add": 2,
    "sub": 2,
    "mul": 2,
    "div": 2,
    "pow": 2,
    "rac": 2,
    "inv": 1,
    "pow2": 1,
    "pow3": 1,
    "pow4": 1,
    "pow5": 1,
    "sqrt": 1,
    "exp": 1,
    "ln": 1,
    "abs": 1,
    "sign": 1,
    "d": 2,
    # Trigonometric Functions
    "sin": 1,
    "cos": 1,
    "tan": 1,
    "csc": 1,
    "sec": 1,
    "cot": 1,
    # Trigonometric Inverses
    "asin": 1,
    "acos": 1,
    "atan": 1,
    "acsc": 1,
    "asec": 1,
    "acot": 1,
    # Hyperbolic Functions
    "sinh": 1,
    "cosh": 1,
    "tanh": 1,
    "csch": 1,
    "sech": 1,
    "coth": 1,
    # Hyperbolic Inverses
    "asinh": 1,
    "acosh": 1,
    "atanh": 1,
    "acsch": 1,
    "asech": 1,
    "acoth": 1,
}
CONSTANTS = ["pi", "e"]
SYMBOLS = ["I", "INT+", "INT-", "INT", "FLOAT", "-", ".", "10^", "Y"]
START = 25.0
END = 75.0
N = 3
TOL = 1e-10
SECS = 10


# -----------------------------------------------------------------------------
# Refactor
# -----------------------------------------------------------------------------
N_DENOMINATOR_DIGITS = 17


# -----------------------------------------------------------------------------
# Filter
# -----------------------------------------------------------------------------
N_EXPRS = {
    "ln":    {'general': 30000000, 'd': 3000000},
    "sin":   {'general': 15000000, 'd': 1500000},
    "cos":   {'general': 15000000, 'd': 1500000},
    "tan":   {'general': 20000000, 'd': 2000000},
    "csc":   {'general': 30000000, 'd': 3000000},
    "sec":   {'general': 30000000, 'd': 3000000},
    "cot":   {'general': 30000000, 'd': 3000000},
    "asin":  {'general': 15000000, 'd': 1500000},
    "acos":  {'general': 10000000, 'd': 1000000},
    "atan":  {'general': 20000000, 'd': 2000000},
    "acsc":  {'general': 30000000, 'd': 3000000},
    "asec":  {'general': 30000000, 'd': 3000000},
    "acot":  {'general': 30000000, 'd': 3000000},
    "sinh":  {'general': 25000000, 'd': 2500000},
    "cosh":  {'general': 25000000, 'd': 2500000},
    "tanh":  {'general': 30000000, 'd': 3000000},
    "csch":  {'general': 30000000, 'd': 3000000},
    "sech":  {'general': 20000000, 'd': 2000000},
    "coth":  {'general': 30000000, 'd': 3000000},
    "asinh": {'general': 10000000, 'd': 1000000},
    "acosh": {'general': 30000000, 'd': 3000000},
    "atanh": {'general': 10000000, 'd': 1000000},
    "acsch": {'general': 15000000, 'd': 1500000},
    "asech": {'general': 30000000, 'd': 3000000},
    "acoth": {'general': 15000000, 'd': 1500000},
    "poly":  {'general': 30000000, 'd': 3000000},
}
N_OPS_PER_EXPR = 3
DX_PCT = 0.1


# -----------------------------------------------------------------------------
# Split
# -----------------------------------------------------------------------------
N_EXPRS_MIN = 20
N_EXPRS_MAX = 40
TRAIN_SIZE = 50000000


# -----------------------------------------------------------------------------
# Data
# -----------------------------------------------------------------------------
DATA_PATH = "data"

# fund exprs
FUND_EXPRS_DIR = DATA_PATH + "/fund_exprs"

# preprocess
EXPRS_FILEPATH = DATA_PATH + "/exprs.txt"
INVALIDS_FILEPATH = DATA_PATH + "/invalids.txt"
DUPLICATES_FILEPATH = DATA_PATH + "/duplicates.txt"
EQUIV_EXPRS_PROC_FILEPATH = DATA_PATH + "/equiv_exprs_proc.txt"

# filter
EQUIV_EXPRS_FILTER_FILEPATH = DATA_PATH + "/equiv_exprs_filter.txt"

# split
EXPR_PAIRS_FILEPATH = DATA_PATH + "/expr_pairs.txt"
EXPRS_VAL_FILEPATH = DATA_PATH + "/exprs_val.txt"
EXPR_TRIPLETS_FILEPATH = DATA_PATH + "/expr_triplets.txt"
EXPR_CL_FILEPATH = DATA_PATH + "/exprs_cl.txt"
# EXPRS_VAL_ML_FILEPATH = DATA_PATH + "/exprs_val_ml.txt"
DERI_FILEPATH = DATA_PATH + "/derivations.txt"
EMB_ALGEBRA_FILEPATH = DATA_PATH + "/emb_algebra.txt"
POOL_FILEPATH = DATA_PATH + "/pool.txt"


# -----------------------------------------------------------------------------
# Hyperparams
# -----------------------------------------------------------------------------
SEED = 42
random.seed(a=SEED)
LOG_LEVEL = logger.LogLevel.INFO
