import logger


DATA_DIR = "data"
EMBEDDING_ALGEBRA_FILEPATH = DATA_DIR + "/embedding_algebra.txt"

SEED = 42

labels = [
    "sin",
    "cos",
    "tan",
    "csc",
    "sec",
    "cot",
    "ln",
    "sinh",
    "cosh",
    "tanh",
    "asin",
    "acos",
    "atan",
]

colors = [
    "cyan",
    "orange",
    "magenta",
    "lime",
    "pink",
    "yellow",
    "purple",
    "skyblue",
    "coral",
    "royalblue",
    "violet",
    "orangered",
    "navy",
]

LOG_LEVEL = logger.LogLevel.INFO