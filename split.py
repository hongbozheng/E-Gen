#!/usr/bin/env python3


from typing import List

import argparse
import config as cfg
import logger
import os
import random
from itertools import permutations


def w_train_file(
        blks: List[List[str]],
        indices: List[int],
        filepath: str,
) -> None:
    expr_pairs = []

    for i in indices:
        exprs = blks[i]
        blk_pairs = list(permutations(iterable=exprs, r=2))
        expr_pairs.extend(blk_pairs)

    indices = list(range(len(expr_pairs)))
    random.shuffle(x=indices)

    file = open(file=filepath, mode='w', encoding='utf-8')
    for i in indices:
        file.write(f"{expr_pairs[i][0]}\t{expr_pairs[i][1]}\n")
    file.write("\n")
    file.close()

    return


def w_val_file(
        blks: List[List[str]],
        indices: List[int],
        filepath: str,
) -> None:
    exprs = []

    for i in indices:
        blk = blks[i]
        exprs.extend(blk)

    indices = list(range(len(exprs)))
    random.shuffle(x=indices)

    file = open(file=filepath, mode='w', encoding='utf-8')
    for i in indices:
        file.write(f"{exprs[i]}\n")
    file.write("\n")
    file.close()

    return


def w_file(
        blks: List[List[str]],
        indices: List[int],
        filepath: str,
) -> None:
    file = open(file=filepath, mode='w', encoding='utf-8')
    for i in indices:
        for expr in blks[i]:
            file.write(f"{expr}\n")
        file.write("\n")
    file.close()

    return


def split(
        pct: float,
        form: str,
        filepath: str,
        train_filepath: str,
        val_filepath: str,
        val_ml_filepath: str,
) -> None:
    blks = []

    file = open(file=filepath, mode='r', encoding='utf-8')
    exprs = []
    for line in file:
        expr = line.strip()
        if expr:
            exprs.append(expr)
        else:
            blks.append(exprs)
            exprs = []
    file.close()

    size = len(blks) 
    indices = list(range(size))
    random.shuffle(x=indices)

    val_indices = indices[:int(size*pct)]
    train_indices = indices[int(size*pct):]

    if form == "pair":
        w_train_file(blks=blks, indices=train_indices, filepath=train_filepath)
        w_val_file(blks=blks, indices=val_indices, filepath=val_filepath)
        w_file(blks=blks, indices=val_indices, filepath=val_ml_filepath)
    else:
        raise NotImplemented

    return


def main() -> None:
    if not os.path.exists(path=cfg.EQUIV_EXPRS_FILTER_FILEPATH):
        logger.log_info(
            f"File '{cfg.EQUIV_EXPRS_FILTER_FILEPATH}' does not exist!"
            "Run './filter' first to create file "
            f"'{cfg.EQUIV_EXPRS_FILTER_FILEPATH}'"
        )
        exit(1)

    parser = argparse.ArgumentParser(
        prog="split",
        description="split the filtered equivalent expressions into "
                    "train & val set "
    )
    parser.add_argument(
        "--pct",
        "-p",
        type=float,
        required=True,
        help="Validation set percentage",
    )
    parser.add_argument(
        "--form",
        "-f",
        type=str,
        required=True,
        choices=["pair", "triplet"],
        help="Whether to write in form of expression pairs or triplets",
    )

    args = parser.parse_args()
    pct = args.pct
    form = args.form

    if form == "pair":
        if os.path.exists(path=cfg.EXPR_PAIRS_TRAIN_FILEPATH):
            logger.log_info(
                f"File '{cfg.EXPR_PAIRS_TRAIN_FILEPATH}' already exists!"
            )
            exit(1)
        if os.path.exists(path=cfg.EXPRS_VAL_FILEPATH):
            logger.log_info(
                f"File '{cfg.EXPRS_VAL_FILEPATH}' already exists!"
            )
            exit(1)
        if os.path.exists(path=cfg.EXPRS_VAL_ML_FILEPATH):
            logger.log_info(
                f"File '{cfg.EXPRS_VAL_ML_FILEPATH}' already exists!"
            )
            exit(1)
        logger.log_info(
            f"Creating files '{cfg.EXPR_PAIRS_TRAIN_FILEPATH}', "
            f"'{cfg.EXPRS_VAL_FILEPATH}', and "
            f"'{cfg.EXPRS_VAL_ML_FILEPATH}'..."
        )
    else:
        if os.path.exists(path=cfg.EXPR_TRIPLETS_FILEPATH):
            logger.log_info(
                f"File '{cfg.EXPR_TRIPLETS_FILEPATH}' already exists!"
            )
            exit(1)
        if os.path.exists(path=cfg.EXPRS_ML_FILEPATH):
            logger.log_info(
                f"File '{cfg.EXPRS_ML_FILEPATH}' already exists!"
            )
            exit(1)
        logger.log_info(
            f"Creating files '{cfg.EXPR_TRIPLETS_FILEPATH}' and "
            f"'{cfg.EXPRS_ML_FILEPATH}'..."
        )

    split(
        pct=pct,
        form=form,
        filepath=cfg.EQUIV_EXPRS_FILTER_FILEPATH,
        train_filepath=cfg.EXPR_PAIRS_TRAIN_FILEPATH,
        val_filepath=cfg.EXPRS_VAL_FILEPATH,
        val_ml_filepath=cfg.EXPRS_VAL_ML_FILEPATH,
    )

    if form == "pair":
        logger.log_info(
            f"Finish creating files '{cfg.EXPR_PAIRS_TRAIN_FILEPATH}', "
            f"'{cfg.EXPRS_VAL_FILEPATH}', and "
            f"'{cfg.EXPRS_VAL_ML_FILEPATH}'."
        )
    else:
        raise NotImplementedError
        logger.log_info(
            f"Finish creating files '{cfg.EXPR_TRIPLETS_FILEPATH}' and "
            f"'{cfg.EXPRS_ML_FILEPATH}'."
        )

    return


if __name__ == "__main__":
    main()
