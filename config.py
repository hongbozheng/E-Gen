import logger

CLASSES = ["poly", "ln",
           "sin", "cos", "tan",
           "csc", "sec", "cot",
           "asin", "acos", "atan",
           "acsc", "asec", "acot",
           "sinh", "cosh", "tanh",
           "csch", "sech", "coth",
           "asinh", "acosh", "atanh",
           "acsch", "asech", "acosh",]
CATEGORIES = ["general", "d",]

DATA_DIR = "data"
DATA_RAW_DIR = DATA_DIR + "/raw"
DATA_PROCESSED_DIR = DATA_DIR + "/processed"
DATA_REFACTORED_DIR = DATA_DIR + "/refactored"

EXPRS_FILEPATH = DATA_DIR + "/exprs.txt"
EQUIV_EXPRS_FILEPATH = DATA_DIR + "/equiv_exprs.txt"
DUPLICATES_FILEPATH = DATA_DIR + "/duplicates.txt"

EXPR_PAIRS_TRAIN_FILEPATH = DATA_DIR + "/expr_pairs_train.txt"
EXPR_PAIRS_VAL_FILEPATH = DATA_DIR + "/expr_pairs_val.txt"
EXPR_PAIRS_TEST_FILEPATH = DATA_DIR + "/expr_pairs_test.txt"

SEED = 42
LOG_LEVEL = logger.LogLevel.INFO