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
START = 25.0
END = 75.0
N = 3
TOL = 1e-10
SECS = 10


# ===================================
# refactor
# ===================================
N_DENOMINATOR_DIGITS = 17


# ===================================
# filter
# ===================================
N_EXPRS = {
    "ln":    {'general': 50, 'd': 50},
    "sin":   {'general': 10, 'd': 10},
    "cos":   {'general': 10, 'd': 10},
    "tan":   {'general': 10, 'd': 10},
    "csc":   {'general': 15, 'd': 15},
    "sec":   {'general': 15, 'd': 15},
    "cot":   {'general': 15, 'd': 15},
    "asin":  {'general': 10, 'd': 10},
    "acos":  {'general': 10, 'd': 10},
    "atan":  {'general': 10, 'd': 10},
    "acsc":  {'general': 15, 'd': 15},
    "asec":  {'general': 15, 'd': 15},
    "acot":  {'general': 25, 'd': 25},
    "sinh":  {'general': 15, 'd': 15},
    "cosh":  {'general': 30, 'd': 30},
    "tanh":  {'general': 30, 'd': 30},
    "csch":  {'general': 45, 'd': 45},
    "sech":  {'general': 45, 'd': 45},
    "coth":  {'general': 40, 'd': 40},
    "asinh": {'general': 30, 'd': 30},
    "acosh": {'general': 45, 'd': 45},
    "atanh": {'general': 35, 'd': 35},
    "acsch": {'general': 40, 'd': 40},
    "asech": {'general': 45, 'd': 45},
    "acoth": {'general': 35, 'd': 35},
    "poly":  {'general': 40, 'd': 40},
}
N_OPS_PER_EXPR = 3
DX_PCT = 0.1


# ===================================
# balance
# ===================================
N_EXPRS_PER_OPS = 250000


# ===================================
# filepath
# ===================================
DATA_DIR = "data"

# fund exprs
FUND_EXPRS_DIR = DATA_DIR + "/fund_exprs"

# preprocess
INVALIDS_FILEPATH = DATA_DIR + "/invalids.txt"
EXPRS_FILEPATH = DATA_DIR + "/exprs.txt"
EQUIV_EXPRS_VERIFIED_FILEPATH = DATA_DIR + "/equiv_exprs_verified.txt"
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
