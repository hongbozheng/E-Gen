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
    "ln":    {'general': 50, 'd': 50},
    "sin":   {'general': 50, 'd': 50},
    "cos":   {'general': 50, 'd': 50},
    "tan":   {'general': 50, 'd': 50},
    "csc":   {'general': 50, 'd': 50},
    "sec":   {'general': 50, 'd': 50},
    "cot":   {'general': 50, 'd': 50},
    "asin":  {'general': 50, 'd': 50},
    "acos":  {'general': 50, 'd': 50},
    "atan":  {'general': 50, 'd': 50},
    "acsc":  {'general': 50, 'd': 50},
    "asec":  {'general': 50, 'd': 50},
    "acot":  {'general': 50, 'd': 50},
    "sinh":  {'general': 50, 'd': 50},
    "cosh":  {'general': 50, 'd': 50},
    "tanh":  {'general': 50, 'd': 50},
    "csch":  {'general': 50, 'd': 50},
    "sech":  {'general': 50, 'd': 50},
    "coth":  {'general': 50, 'd': 50},
    "asinh": {'general': 50, 'd': 50},
    "acosh": {'general': 50, 'd': 50},
    "atnah": {'general': 50, 'd': 50},
    "acsch": {'general': 50, 'd': 50},
    "asech": {'general': 50, 'd': 50},
    "acoth": {'general': 50, 'd': 50},
    "poly":  {'general': 50, 'd': 50},
}
N_OPS_PER_EXPR = 3


# ===================================
# filter
# ===================================
N_OPS = 100


# ===================================
# filepath
# ===================================
DATA_DIR = "data"

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