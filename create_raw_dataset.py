#!/usr/bin/env python3


import argparse
import config
import glob
import logger
import os
import tqdm


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


def create_raw_dataset(equiv_exprs_dir: str, classes: list[str], categories: list[str], data_raw_dir: str) -> None:
    pathname = os.path.join(equiv_exprs_dir, "equiv_exprs_*.txt")
    filepaths = glob.glob(pathname=pathname)

    progbar = tqdm.tqdm(iterable=filepaths)

    for filepath in progbar:
        progbar.set_description(desc="[INFO]: Processing file '%s'" % filepath, refresh=True)

        with open(file=filepath, mode='r') as file:
            equiv_exprs = []

            for line in file:
                if line.strip() and line not in equiv_exprs:
                    equiv_exprs.append(line)
                elif not line.strip():
                    equiv_exprs.append(line)

                    cls, category = classify(expr=equiv_exprs[0], classes=classes, categories=categories)
                    w_raw_data(data_raw_dir=data_raw_dir, equiv_exprs=equiv_exprs, cls=cls, category=category)

                    equiv_exprs = []

    return


def main() -> None:
    if os.path.exists(path=config.DATA_RAW_DIR):
        logger.log_info(f"Raw dataset directory '{config.DATA_RAW_DIR}' already exists!")
        logger.log_info("Make sure you ONLY include NEWLY generated equivalent expressions .txt files in the directory "
                        "you provided as command line input!")
        logger.log_info("Or make sure you remove the existing raw dataset directory and include all generated "
                        "equivalent expressions .txt files in the directory you provided as command line input!")

        usr_input = input("[INFO]: Do you want to abort the operation? [Y/n]")
        if usr_input == 'Y':
            logger.log_info("Operation aborted.")
            return

    parser = argparse.ArgumentParser(prog="create_raw_dataset",
                                     description="split generated equivalent expressions into categories")
    parser.add_argument("--equiv_exprs_dir", "-d", type=str, required=True, help="Equivalent expressions directory")

    args = parser.parse_args()
    equiv_exprs_dir = args.equiv_exprs_dir

    logger.log_info("Creating raw dataset...")
    create_raw_dataset(equiv_exprs_dir=equiv_exprs_dir, classes=config.CLASSES, categories=config.CATEGORIES,
                       data_raw_dir=config.DATA_RAW_DIR)
    logger.log_info("Finish creating raw dataset.")

    return


if __name__ == "__main__":
    main()