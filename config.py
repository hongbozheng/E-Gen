import logger


# ===================================
# fund expr
# ===================================
OPERATORS = ["ln",
             "sin", "cos", "tan",
             "csc", "sec", "cot",
             "asin", "acos", "atan",
             "acsc", "asec", "acot",
             "sinh", "cosh", "tanh",
             "csch", "sech", "coth",
             "asinh", "acosh", "atanh",
             "acsch", "asech", "acoth",]
CATEGORIES = ["general", "d",]


# ===================================
# sympy verify
# ===================================
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
START = 0.0
END = 10.0
N = 3
TOL = 1e-5
SECS = 2


# ===================================
# refactor
# ===================================
N_DENOMINATOR_DIGITS = 17


# ===================================
# filter
# ===================================
N_EXPRS = {
    "ln":    {'general': 110, 'd': 110},
    "sin":   {'general': 35, 'd': 35},
    "cos":   {'general': 35, 'd': 35},
    "tan":   {'general': 35, 'd': 35},
    "csc":   {'general': 40, 'd': 40},
    "sec":   {'general': 40, 'd': 40},
    "cot":   {'general': 40, 'd': 40},
    "asin":  {'general': 35, 'd': 35},
    "acos":  {'general': 35, 'd': 35},
    "atan":  {'general': 40, 'd': 40},
    "acsc":  {'general': 35, 'd': 35},
    "asec":  {'general': 35, 'd': 35},
    "acot":  {'general': 50, 'd': 50},
    "sinh":  {'general': 35, 'd': 35},
    "cosh":  {'general': 60, 'd': 60},
    "tanh":  {'general': 60, 'd': 60},
    "csch":  {'general': 90, 'd': 90},
    "sech":  {'general': 90, 'd': 90},
    "coth":  {'general': 80, 'd': 80},
    "asinh": {'general': 60, 'd': 60},
    "acosh": {'general': 90, 'd': 90},
    "atanh": {'general': 70, 'd': 70},
    "acsch": {'general': 80, 'd': 80},
    "asech": {'general': 100, 'd': 100},
    "acoth": {'general': 80, 'd': 80},
    "poly":  {'general': 50, 'd': 50},
}
N_OPS_PER_EXPR = 3
DX_PCT = 0.1


# ===================================
# balance
# ===================================
N_EXPRS_PER_OPS = 500000


# ===================================
# filepath
# ===================================
DATA_DIR = "data"

# fund exprs
FUND_EXPRS_DIR = DATA_DIR + "/fund_exprs"

# preprocess
INVALIDS_FILEPATH = DATA_DIR + "/invalids.txt"
EXPRS_FILEPATH = DATA_DIR + "/exprs.txt"
EQUIV_EXPRS_RAW_FILEPATH = DATA_DIR + "/equiv_exprs_raw.txt"
DUPLICATES_FILEPATH = DATA_DIR + "/duplicates.txt"

# filter
EQUIV_EXPRS_FILTERED_FILEPATH = DATA_DIR + "/equiv_exprs_filtered.txt"

# expr pairs
EXPR_PAIRS_FILEPATH = DATA_DIR + "/expr_pairs.txt"
INCORRECTS_FILEPATH = DATA_DIR + "/incorrects.txt"

# split
EXPR_PAIRS_TRAIN_FILEPATH = DATA_DIR + "/expr_pairs_train.txt"
EXPRS_VAL_FILEPATH = DATA_DIR + "/exprs_val.txt"
EXPRS_TEST_FILEPATH = DATA_DIR + "/exprs_test.txt"


# ===================================
# hyperparameter
# ===================================
SEED = 42
LOG_LEVEL = logger.LogLevel.INFO