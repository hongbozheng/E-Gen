#!/usr/bin/env python3


from typing import List, Tuple

import argparse
import config as cfg
import logger
import os
import random
from itertools import combinations, permutations
from logger import timestamp
from preproc import get_n_lines
from tqdm import tqdm


def w_file(clusters: List[List[str]], filepath: str) -> None:
    file = open(file=filepath, mode='w', encoding='utf-8')
    for cluster in clusters:
        for expr in cluster:
            file.write(f"{expr}\n")
        file.write('\n')
    file.close()

    return


def w_val_file(clusters: List[List[str]], filepath: str) -> None:
    exprs = [s for cluster in clusters for s in cluster]

    ids = list(range(len(exprs)))
    random.shuffle(x=ids)

    file = open(file=filepath, mode='w', encoding='utf-8')
    for i in ids:
        file.write(f"{exprs[i]}\n")
    file.write("\n")
    file.close()

    return


def val(
        filepath: str,
        n_min: int,
        n_max: int,
        n_exprs: int,
) -> Tuple[List[List[str]], List[int]]:
    n_lines = get_n_lines(filepath=filepath)

    exprs = []
    val_ids = []
    val_set = []
    val_len = []

    file = open(file=filepath, mode='r', encoding='utf-8')
    for i, line in enumerate(tqdm(
            iterable=file,
            desc=f"[{timestamp()}] [INFO]: Reading file '{filepath}'",
            total=n_lines,
            leave=True,
            position=0,
    )):
        expr = line.strip()
        if expr:
            exprs.append(expr)
        else:
            if n_min <= len(exprs) <= n_max:
                val_ids.append(i)
                val_set.append(exprs)
                val_len.append(len(exprs))
            exprs = []
    file.close()

    n = n_exprs // max(val_len)
    while True:
        ids = random.sample(population=range(len(val_len)), k=n)
        lens = [val_len[i] for i in ids]
        if sum(lens) >= n_exprs:
            break
        n += 1

    val_set = [val_set[i] for i in ids]
    val_ids = [val_ids[i] for i in ids]

    return val_set, val_ids


def w_cl_file(clusters: List[List[str]], min_neg: int, filepath: str) -> None:
    n_clusters = len(clusters)

    triplets = []
    for i in range(n_clusters):
        pos = list(permutations(iterable=clusters[i], r=2))

        ids = list(set(range(n_clusters)) - {i})
        if len(ids) > min_neg:
            ids = random.sample(population=ids, k=min_neg)
        elif len(ids) < min_neg:
            quotient, remainder = divmod(min_neg, len(ids))
            samples = random.sample(population=ids, k=remainder)
            ids = ids * quotient + samples

        neg = [random.sample(population=clusters[i], k=1)[0] for i in ids]

        if len(pos) > min_neg:
            pos = random.sample(population=pos, k=min_neg)
        elif len(pos) < min_neg:
            quotient, remainder = divmod(min_neg, len(pos))
            samples = random.sample(population=pos, k=remainder)
            pos = pos * quotient + samples

        assert min_neg == len(pos) == len(neg)

        for p, n in zip(pos, neg):
            triplets.append((p[0], p[1], n))

    ids = list(range(len(triplets)))
    random.shuffle(x=ids)

    file = open(file=filepath, mode='w', encoding="utf-8")
    for i in ids:
        file.write(f"{triplets[i][0]}\t{triplets[i][1]}\t{triplets[i][2]}\n")
    file.close()

    return


def w_train_file(clusters: List[List[str]], filepath: str) -> None:
    expr_pairs = []

    for cluster in clusters:
        cluster_pairs = list(permutations(iterable=cluster, r=2))
        expr_pairs.extend(cluster_pairs)

    ids = list(range(len(expr_pairs)))
    random.shuffle(x=ids)

    file = open(file=filepath, mode='w', encoding='utf-8')
    for i in ids:
        file.write(f"{expr_pairs[i][0]}\t{expr_pairs[i][1]}\n")
    file.write("\n")
    file.close()

    return


def train(filepath: str, val_ids: List[int]) -> List[List[str]]:
    n_lines = get_n_lines(filepath=filepath)

    exprs = []
    train_set = []

    file = open(file=filepath, mode='r', encoding='utf-8')
    for i, line in enumerate(tqdm(
            iterable=file,
            desc=f"[{timestamp()}] [INFO]: Reading file '{filepath}'",
            total=n_lines,
            leave=True,
            position=0,
    )):
        expr = line.strip()
        if expr:
            exprs.append(expr)
        else:
            if i not in val_ids:
                train_set.append(exprs)
            exprs = []
    file.close()

    return train_set


def split(
        form: str,
        n_min: int,
        n_max: int,
        n_exprs: int,
        train_size: int,
        filepath: str,
        triplets_filepath: str,
        cl_filepath: str,
        pairs_filepath: str,
        val_filepath: str,
) -> None:
    if form == "triplet":
        logger.log_info(f"Creating file '{cl_filepath}'...")
    elif form == "pair":
        logger.log_info(f"Creating file '{val_filepath}'...")

    val_set, val_ids = val(
        filepath=filepath,
        n_min=n_min,
        n_max=n_max,
        n_exprs=n_exprs
    )

    if form == "pair":
        w_val_file(clusters=val_set, filepath=val_filepath)
        logger.log_info(f"Finish creating file '{val_filepath}'.")
    elif form == "triplet":
        w_file(clusters=val_set, filepath=cl_filepath)
        logger.log_info(f"Finish creating file '{cl_filepath}'.")

    if form == "pair":
        logger.log_info(f"Creating file '{pairs_filepath}'...")
    elif form == "triplet":
        logger.log_info(f"Creating file '{triplets_filepath}'...")

    train_set = train(filepath=filepath, val_ids=val_ids)

    if form == "triplet":
        min_neg = train_size // len(train_set) + 1
        w_cl_file(
            clusters=train_set,
            min_neg=min_neg,
            filepath=triplets_filepath
        )
        logger.log_info(f"Finish creating file '{triplets_filepath}'.")
    elif form == "pair":
        w_train_file(clusters=train_set, filepath=pairs_filepath)
        logger.log_info(f"Finish creating file '{pairs_filepath}'.")

    return


def main() -> None:
    if not os.path.exists(path=cfg.EQUIV_EXPRS_FILTER_FILEPATH):
        logger.log_info(
            f"File '{cfg.EQUIV_EXPRS_FILTER_FILEPATH}' does not exist! "
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
        "--val",
        "-v",
        type=int,
        required=True,
        help="Number of expressions for validation",
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
    n_exprs = args.val
    form = args.form

    if form == "triplet":
        if os.path.exists(path=cfg.EXPR_TRIPLETS_FILEPATH):
            logger.log_info(
                f"File '{cfg.EXPR_TRIPLETS_FILEPATH}' already exists!"
            )
            exit(1)
        if os.path.exists(path=cfg.EXPR_CL_FILEPATH):
            logger.log_info(
                f"File '{cfg.EXPR_CL_FILEPATH}' already exists!"
            )
            exit(1)
    else:
        if os.path.exists(path=cfg.EXPR_PAIRS_FILEPATH):
            logger.log_info(
                f"File '{cfg.EXPR_PAIRS_FILEPATH}' already exists!"
            )
            exit(1)
        if os.path.exists(path=cfg.EXPRS_VAL_FILEPATH):
            logger.log_info(
                f"File '{cfg.EXPRS_VAL_FILEPATH}' already exists!"
            )
            exit(1)
        # if os.path.exists(path=cfg.EXPRS_VAL_ML_FILEPATH):
        #     logger.log_info(
        #         f"File '{cfg.EXPRS_VAL_ML_FILEPATH}' already exists!"
        #     )
        #     exit(1)

    split(
        form=form,
        n_min=cfg.N_EXPRS_MIN,
        n_max=cfg.N_EXPRS_MAX,
        n_exprs=n_exprs,
        train_size=cfg.TRAIN_SIZE,
        filepath=cfg.EQUIV_EXPRS_FILTER_FILEPATH,
        triplets_filepath=cfg.EXPR_TRIPLETS_FILEPATH,
        cl_filepath=cfg.EXPR_CL_FILEPATH,
        pairs_filepath=cfg.EXPR_PAIRS_FILEPATH,
        val_filepath=cfg.EXPRS_VAL_FILEPATH,
    )

    return


if __name__ == "__main__":
    main()
