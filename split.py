#!/usr/bin/env python3


import argparse
import config
import glob
import logger
import os
import random
import tqdm


def append(exprs: list[str], _set: set[str], subset: list[list[str]]) -> list[str]:
    for expr_pairs in subset:
        for expr_pair in expr_pairs:
            expr_pair = expr_pair.strip().split('\t')
            if expr_pair[0] not in _set:
                exprs.append(expr_pair[0])
                _set.add(expr_pair[0])
            if expr_pair[1] not in _set:
                exprs.append(expr_pair[1])
                _set.add(expr_pair[1])

    return exprs


def w_set(exprs: list[str], filepath: str) -> None:
    indices = list(range(len(exprs)))
    random.shuffle(indices)

    file = open(file=filepath, mode='w')

    for i in indices:
        file.write(f"{exprs[i]}\n")

    file.close()

    return


def split(
        data_dir: str,
        seed: int,
        test_pct: float,
        val_pct: float,
        expr_pairs_train_filepath: str,
        expr_pairs_val_filepath: str,
        expr_pairs_test_filepath: str,
) -> None:
    random.seed(a=seed)

    filepath = os.path.join(data_dir, "**", "equiv_exprs.txt")
    filepaths = glob.glob(pathname=filepath, recursive=True)

    exprs_test = []
    _exprs_test = set()
    exprs_val = []
    _exprs_val = set()
    expr_pairs_train = []

    progbar = tqdm.tqdm(iterable=filepaths)

    for filepath in progbar:
        parts = filepath.split(os.path.sep)
        cls = parts[-3]
        category = parts[-2]
        progbar.set_description(desc=f"[INFO]: Processing class '{cls}', category '{category}'", refresh=True)

        file = open(file=filepath, mode='r')

        expr_pairs_list = []
        expr_pairs = []

        for line in file:
            if line.strip():
                expr_pairs.append(line)
            else:
                expr_pairs_list.append(expr_pairs)

                expr_pairs = []

        n_exprs = len(expr_pairs_list)
        test_size = int(n_exprs*test_pct)
        val_size = int(n_exprs*val_pct)
        indices = list(range(n_exprs))
        random.shuffle(indices)

        subset_test = [expr_pairs_list[i] for i in indices[:test_size]]
        subset_val = [expr_pairs_list[i] for i in indices[test_size:test_size+val_size]]

        exprs_test = append(exprs=exprs_test, _set=_exprs_test, subset=subset_test)
        exprs_val = append(exprs=exprs_val, _set=_exprs_val, subset=subset_val)
        expr_pairs_train.extend(expr_pairs_list[test_size+val_size:])

        file.close()

    assert len(exprs_test) == len(_exprs_test)
    assert len(exprs_val) == len(_exprs_val)
    expr_pairs_train = [expr_pair for expr_pairs in expr_pairs_train for expr_pair in expr_pairs]

    w_set(exprs=exprs_test, filepath=expr_pairs_test_filepath)
    w_set(exprs=exprs_val, filepath=expr_pairs_val_filepath)
    w_set(exprs=expr_pairs_train, filepath=expr_pairs_train_filepath)

    return


def main() -> None:
    if not os.path.exists(path=config.DATA_VERIFIED_DIR):
        logger.log_error(f"'{config.DATA_VERIFIED_DIR}' directory does not exist!")
        logger.log_error(f"Make sure to run './verify.py' first to create '{config.DATA_VERIFIED_DIR}' directory.")
        logger.log_error("Operation aborted.")
        exit(1)
    if os.path.exists(path=config.EXPR_PAIRS_TRAIN_FILEPATH):
        logger.log_error(f"'{config.EXPR_PAIRS_TRAIN_FILEPATH}' file already exists!")
        logger.log_error(f"Make sure to remove '{config.EXPR_PAIRS_TRAIN_FILEPATH}' file first.")
        logger.log_error("Operation aborted.")
        exit(1)
    if os.path.exists(path=config.EXPR_PAIRS_VAL_FILEPATH):
        logger.log_error(f"'{config.EXPR_PAIRS_VAL_FILEPATH}' file already exists!")
        logger.log_error(f"Make sure to remove '{config.EXPR_PAIRS_VAL_FILEPATH}' file first.")
        logger.log_error("Operation aborted.")
        exit(1)
    if os.path.exists(path=config.EXPR_PAIRS_TEST_FILEPATH):
        logger.log_error(f"'{config.EXPR_PAIRS_TEST_FILEPATH}' file already exists!")
        logger.log_error(f"Make sure to remove '{config.EXPR_PAIRS_TEST_FILEPATH}' file first.")
        logger.log_error("Operation aborted.")
        exit(1)

    parser = argparse.ArgumentParser(prog="split", description="Create train, val and test splits")
    parser.add_argument("--test_pct", "-t", type=float, required=True, help="test set percentage")
    parser.add_argument("--val_pct", "-v", type=float, required=True, help="validation set percentage")

    args = parser.parse_args()
    test_pct = args.test_pct
    val_pct = args.val_pct

    logger.log_info("Creating train, val, and test sets...")
    split(data_dir=config.DATA_VERIFIED_DIR, seed=config.SEED, test_pct=test_pct, val_pct=val_pct,
          expr_pairs_train_filepath=config.EXPR_PAIRS_TRAIN_FILEPATH,
          expr_pairs_test_filepath=config.EXPR_PAIRS_TEST_FILEPATH,
          expr_pairs_val_filepath=config.EXPR_PAIRS_VAL_FILEPATH)
    logger.log_info("Finish creating train, val, and test sets.")

    return


if __name__ == "__main__":
    main()