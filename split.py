#!/usr/bin/env python3


import argparse
import config
import glob
import itertools
import logger
import os
import random
import tqdm


def w_set(equiv_exprs_list: list[list[str]], expr_pairs_filepath: str) -> None:
    expr_pairs_file = open(file=expr_pairs_filepath, mode='a')

    for equiv_exprs in equiv_exprs_list:
        for pair in itertools.permutations(iterable=equiv_exprs, r=2):
            expr_pairs_file.write(f"{pair[0]}\t{pair[1]}\n")

    expr_pairs_file.close()

    return


def split(
        seed: int,
        data_dir: str,
        test_pct: float,
        val_pct: float,
        expr_pairs_train_filepath: str,
        expr_pairs_val_filepath: str,
        expr_pairs_test_filepath: str,
) -> None:
    filepath = os.path.join(data_dir, "**", "equiv_exprs.txt")
    filepaths = glob.glob(pathname=filepath, recursive=True)

    progbar = tqdm.tqdm(iterable=filepaths)

    for filepath in progbar:
        parts = filepath.split(os.path.sep)
        cls = parts[-3]
        category = parts[-2]
        progbar.set_description(desc=f"[INFO]: Processing class '{cls}', category '{category}'", refresh=True)

        file = open(file=filepath, mode='r')

        equiv_exprs_list = []
        equiv_exprs = []

        for line in file:
            if line.strip():
                equiv_exprs.append(line.strip())
            else:
                equiv_exprs_list.append(equiv_exprs)
                equiv_exprs = []

        file.close()

        random.seed(a=seed)
        random.shuffle(x=equiv_exprs_list)

        n_exprs = len(equiv_exprs_list)
        test_size = int(n_exprs*test_pct)
        val_size = int(n_exprs*val_pct)

        test_set = equiv_exprs_list[:test_size]
        val_set = equiv_exprs_list[test_size:test_size+val_size]
        train_set = equiv_exprs_list[test_size+val_size:]

        w_set(equiv_exprs_list=train_set, expr_pairs_filepath=expr_pairs_train_filepath)
        w_set(equiv_exprs_list=val_set, expr_pairs_filepath=expr_pairs_val_filepath)
        w_set(equiv_exprs_list=test_set, expr_pairs_filepath=expr_pairs_test_filepath)

        equiv_exprs_list = []

    return


def main() -> None:
    if os.path.exists(path=config.EXPR_PAIRS_TRAIN_FILEPATH):
        logger.log_error(f"{config.EXPR_PAIRS_TRAIN_FILEPATH} file already exists!")
        logger.log_error(f"Make sure to remove {config.EXPR_PAIRS_TRAIN_FILEPATH} file first.")
        logger.log_error("Operation aborted.")
        exit(1)
    if os.path.exists(path=config.EXPR_PAIRS_VAL_FILEPATH):
        logger.log_error(f"{config.EXPR_PAIRS_VAL_FILEPATH} file already exists!")
        logger.log_error(f"Make sure to remove {config.EXPR_PAIRS_VAL_FILEPATH} file first.")
        logger.log_error("Operation aborted.")
        exit(1)
    if os.path.exists(path=config.EXPR_PAIRS_TEST_FILEPATH):
        logger.log_error(f"{config.EXPR_PAIRS_TEST_FILEPATH} file already exists!")
        logger.log_error(f"Make sure to remove {config.EXPR_PAIRS_TEST_FILEPATH} file first.")
        logger.log_error("Operation aborted.")
        exit(1)

    parser = argparse.ArgumentParser(prog="split", description="Create train, val and test splits")
    parser.add_argument("--seed", "-s", type=int, default=42, required=False, help="random seed")
    parser.add_argument("--dataset_dir", "-d", required=True, help="dataset directory")
    parser.add_argument("--test_pct", "-t", type=float, required=True, help="test set percentage")
    parser.add_argument("--val_pct", "-v", type=float, required=True, help="validation set percentage")

    args = parser.parse_args()
    seed = args.seed
    dataset_dir = args.dataset_dir
    test_pct = args.test_pct
    val_pct = args.val_pct

    split(seed=seed, data_dir=dataset_dir, test_pct=test_pct, val_pct=val_pct,
          expr_pairs_train_filepath=config.EXPR_PAIRS_TRAIN_FILEPATH,
          expr_pairs_test_filepath=config.EXPR_PAIRS_TEST_FILEPATH,
          expr_pairs_val_filepath=config.EXPR_PAIRS_VAL_FILEPATH)

    return


if __name__ == "__main__":
    main()