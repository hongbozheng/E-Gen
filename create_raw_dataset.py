#!/usr/bin/env python3


import config
import logger
import os


def classify(expr: str, classes: list[str], categories: list[str]) -> tuple[str, str]:
    expr = expr.split(sep=' ')

    cls = ""
    category = ""

    for token in expr:
        if token in classes:
            cls = token
        if token in categories:
            category = token

    if cls == "":
        cls = classes[0]
    if category == "":
        category = categories[0]

    return cls, category


def w_raw_data(data_raw_dir: str, equiv_exprs: list[str], cls: str, category: str) -> None:
    path = os.path.join(data_raw_dir, cls, category)
    if not os.path.exists(path=path):
        os.makedirs(name=path, exist_ok=True)

    filepath = os.path.join(path, "equiv_exprs.txt")
    with open(file=filepath, mode='a') as file:
        for expr in equiv_exprs:
            file.write(expr)

    return


def create_raw_dataset(equiv_exprs_filepath: str, classes: list[str], categories: list[str], data_raw_dir: str) -> None:
    equiv_exprs_file = open(file=equiv_exprs_filepath, mode='r')

    equiv_exprs = []

    for line in equiv_exprs_file:
        if line.strip() and line not in equiv_exprs:
            equiv_exprs.append(line)
        elif not line.strip():
            equiv_exprs.append(line)

            cls, category = classify(expr=equiv_exprs[0], classes=classes, categories=categories)
            w_raw_data(data_raw_dir=data_raw_dir, equiv_exprs=equiv_exprs, cls=cls, category=category)

            equiv_exprs = []

    equiv_exprs_file.close()

    return


def main() -> None:
    if not os.path.exists(path=config.EQUIV_EXPRS_FILEPATH):
        logger.log_error(f"'{config.EQUIV_EXPRS_FILEPATH}' file does not exist!")
        logger.log_error(f"Make sure to run './deduplicate.py' first to create {config.EQUIV_EXPRS_FILEPATH} file.")
        logger.log_error("Operation aborted.")
        exit(1)
    if os.path.exists(path=config.DATA_RAW_DIR):
        logger.log_error(f"{config.DATA_RAW_DIR} directory already exists!")
        logger.log_error(f"Make sure to delete {config.DATA_RAW_DIR} directory first.")
        logger.log_error("Operation aborted.")
        exit(1)

    logger.log_info("Creating raw dataset...")
    create_raw_dataset(equiv_exprs_filepath=config.EQUIV_EXPRS_FILEPATH, classes=config.CLASSES,
                       categories=config.CATEGORIES, data_raw_dir=config.DATA_RAW_DIR)
    logger.log_info("Finish creating raw dataset.")

    return


if __name__ == "__main__":
    main()