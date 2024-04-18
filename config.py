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
    "poly_1": 50,
    "poly_3": 50,
    "poly_5": 50,
    "poly_7": 50,
    "poly_9": 50,
    "poly_11": 50,
    "poly_d_1": 50,
    "poly_d_3": 50,
    "poly_d_5": 50,
    "poly_d_7": 50,
    "poly_d_9": 50,
    "poly_d_11": 50,
    "op_2": 50,
    "op_4": 50,
    "op_6": 50,
    "op_8": 50,
    "op_d_2": 50,
    "op_d_4": 50,
    "op_d_6": 50,
    "op_d_8": 50,
}
N_OPS_PER_EXPR = 3


# ===================================
# filter
# ===================================
N_OPS = 10000


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