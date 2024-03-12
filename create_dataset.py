#!/usr/bin/env python3


import argparse
import config
import editdistance
import itertools
import logger
import os


def filter(equiv_exprs: list[str], n_exprs: int) -> list[str]:
    edit_dists = []
    for i, expr in enumerate(equiv_exprs[1:]):
        dist = editdistance.eval(a=equiv_exprs[0], b=expr)
        edit_dists.append((i, dist))

    indices_dists = sorted(edit_dists, key=lambda x: x[1], reverse=True)
    indices_dists = indices_dists[:n_exprs-1]

    equiv_exprs_filtered = []
    equiv_exprs_filtered.append(equiv_exprs[0])
    for index, _ in indices_dists:
        equiv_exprs_filtered.append(equiv_exprs[index+1])

    assert len(equiv_exprs_filtered) == n_exprs

    return equiv_exprs_filtered


def create_pairs(equiv_exprs: list) -> list:
    expr_pairs = []

    for expr_pair in itertools.permutations(iterable=equiv_exprs, r=2):
        expr_pairs.append(expr_pair)

    return expr_pairs


def classify(expr: str, classes: list[str], categories: list[str]) -> tuple[str, str]:
    ops = {op: 0 for op in classes}
    category = ""

    expr = expr.split(sep=' ')
    for token in expr:
        if token in classes:
            ops[token] += 1
        if token in categories:
            category = token

    expr_ops = [op for op, cnt in ops.items() if cnt > 0]

    if expr_ops:
        cls = '_'.join(expr_ops)
    else:
        cls = classes[0]
    if category == "":
        category = categories[0]

    return cls, category


def w_data(expr_pair: str, data_dir: str, cls: str, category: str) -> None:
    path = os.path.join(data_dir, cls, category)

    if not os.path.exists(path=path):
        os.makedirs(name=path, exist_ok=True)

    filepath = os.path.join(path, "equiv_exprs.txt")
    file = open(file=filepath, mode='a')
    file.write(f"{expr_pair[0]}\t{expr_pair[1]}\n")
    file.close()

    return


def create_dataset(
        equiv_exprs_filepath: str,
        classes: list[str],
        categories: list[str],
        filter_: bool,
        n_exprs: int,
        data_dir: str,
) -> None:
    file = open(file=equiv_exprs_filepath, mode='r')

    equiv_exprs = []

    for line in file:
        if line.strip():
            equiv_exprs.append(line.strip())
        else:
            if filter_:
                if len(equiv_exprs) == 1:
                    equiv_exprs = []
                    continue
                elif len(equiv_exprs) > n_exprs:
                    equiv_exprs = filter(equiv_exprs=equiv_exprs, n_exprs=n_exprs)

            expr_pairs = create_pairs(equiv_exprs=equiv_exprs)

            for expr_pair in expr_pairs:
                cls, category = classify(expr=expr_pair[0], classes=classes, categories=categories)
                w_data(expr_pair=expr_pair, data_dir=data_dir, cls=cls, category=category)

            equiv_exprs = []

    file.close()

    return


def main() -> None:
    if not os.path.exists(path=config.EQUIV_EXPRS_FILEPATH):
        logger.log_error(f"'{config.EQUIV_EXPRS_FILEPATH}' file does not exist!")
        logger.log_error(f"Make sure to run './deduplicate.py' first to create '{config.EQUIV_EXPRS_FILEPATH}' file.")
        logger.log_error("Operation aborted.")
        exit(1)

    parser = argparse.ArgumentParser(prog="create_dataset",
                                     description="Create raw dataset by splitting all equivalent expressions into "
                                                 "different classes & categories or "
                                                 "Create filtered dataset by removing expressions with `0` equivalent "
                                                 "expressions, filtering the ones with more than `<n_exprs>` "
                                                 "equivalent expressions, and splitting all equivalent expressions "
                                                 "into different classes & categories")
    parser.add_argument("--filter", "-f", action="store_true", default=False, required=False,
                        help="Whether to filter equivalent expressions")
    parser.add_argument("--n_exprs", "-n", type=int, required=False,
                        help="Number of equivalent expressions to keep")

    args = parser.parse_args()
    filter_ = args.filter
    n_exprs = args.n_exprs

    if filter_ and n_exprs is None:
        logger.log_error_raw("[USAGE]: create_dataset [-h] [--filter] --n_exprs N_EXPRS")
        logger.log_error("The following argument is  required: --n_exprs/-n")
        exit(1)
    if filter_ and os.path.exists(path=config.DATA_FILTERED_DIR):
        logger.log_error(f"'{config.DATA_FILTERED_DIR}' directory already exists!")
        logger.log_error(f"Make sure to delete '{config.DATA_FILTERED_DIR}' directory first.")
        logger.log_error("Operation aborted.")
        exit(1)
    elif not filter_ and os.path.exists(path=config.DATA_RAW_DIR):
        logger.log_error(f"'{config.DATA_RAW_DIR}' directory already exists!")
        logger.log_error(f"Make sure to delete '{config.DATA_RAW_DIR}' directory first.")
        logger.log_error("Operation aborted.")
        exit(1)

    if filter_:
        logger.log_info("Creating filtered dataset...")
        create_dataset(equiv_exprs_filepath=config.EQUIV_EXPRS_FILEPATH, classes=config.CLASSES,
                       categories=config.CATEGORIES, filter_=filter_, n_exprs=n_exprs,
                       data_dir=config.DATA_FILTERED_DIR)
        logger.log_info("Finish creating dataset.")
    else:
        logger.log_info("Creating raw dataset...")
        create_dataset(equiv_exprs_filepath=config.EQUIV_EXPRS_FILEPATH, classes=config.CLASSES,
                       categories=config.CATEGORIES, filter_=filter_, n_exprs=n_exprs,
                       data_dir=config.DATA_RAW_DIR)
        logger.log_info("Finish raw dataset.")

    return


if __name__ == "__main__":
    main()