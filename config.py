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

LOG_LEVEL = logger.LogLevel.INFO