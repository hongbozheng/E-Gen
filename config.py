import logger

CLASSES = ["poly", "ln",
           "sin", "cos", "tan",
           "csc", "sec", "cot",
           "asin", "acos", "atan",
           "acsc", "asec", "acot",
           "sinh", "cosh", "tanh",
           "csch", "sech", "coth",
           "asinh", "acosh", "atanh",
           "acsch", "asech", "acoth",]
CATEGORIES = ["general", "d",]

DATA_DIR = "data"
DATA_RAW_DIR = DATA_DIR + "/raw"
DATA_FILTERED_DIR = DATA_DIR + "/filtered"
DATA_VERIFIED_DIR = DATA_DIR + "/verified"
DATA_INCORRECT_DIR = DATA_DIR + "/incorrect"

EXPRS_FILEPATH = DATA_DIR + "/exprs.txt"
EQUIV_EXPRS_FILEPATH = DATA_DIR + "/equiv_exprs.txt"
DUPLICATES_FILEPATH = DATA_DIR + "/duplicates.txt"

EXPR_PAIRS_TRAIN_FILEPATH = DATA_DIR + "/expr_pairs_train.txt"
EXPRS_VAL_FILEPATH = DATA_DIR + "/exprs_val.txt"
EXPRS_TEST_FILEPATH = DATA_DIR + "/exprs_test.txt"

FUND_EXPRS_FILEPATH = DATA_DIR + "/fund_exprs.txt"

SEED = 42
LOG_LEVEL = logger.LogLevel.INFO