#!/usr/bin/env python3


from typing import Dict, List

import config as cfg
import logger
import mmap
import os
import random
from logger import timestamp
from tqdm import tqdm
from write import write


def get_n_lines(filepath: str) -> int:
    fp = open(file=filepath, mode='r+')
    buf = mmap.mmap(fileno=fp.fileno(), length=0)
    n_lines = 0
    while buf.readline():
        n_lines += 1
    fp.close()

    return n_lines


def get_n_exprs(expr: str, n_exprs: Dict[str, Dict[str, int]]) -> int:
    tokens = expr.replace("INT+ ", "").replace("INT- ", "").split(sep=' ')

    if tokens[0] != 'd':
        for token in tokens:
            if token in n_exprs:
                return n_exprs[token]['general']
        return n_exprs['poly']['general']
    else:
        for token in tokens[2:]:
            if token in n_exprs:
                return n_exprs[token]['d']
        return n_exprs['poly']['d']


def filter(
        equiv_exprs: List[str],
        operators: List[str],
        n_ops: int,
        n: int,
        dx: bool,
) -> List[str]:
    exprs = [equiv_exprs[0]]

    for expr in equiv_exprs[1:]:
        tokens = expr.split(sep=' ')
        op_cnt = sum(1 for token in tokens if token in operators)
        if not dx:
            if op_cnt <= n_ops:
                exprs.append(expr)
        else:
            if op_cnt <= n_ops and "d x" not in expr:
                exprs.append(expr)

    if len(exprs) < n and dx:
        exprs_op = list(set(equiv_exprs[1:])-set(exprs))
        for expr in exprs_op:
            tokens = expr.split(sep=' ')
            op_cnt = sum(1 for token in tokens if token in operators)
            if op_cnt <= n_ops:
                exprs.append(expr)

    if len(exprs) == n:
        return exprs
    elif len(exprs) > n:
        exprs = random.sample(population=exprs[1:], k=n-1)
        exprs.insert(0, equiv_exprs[0])
        return exprs
    else:
        exprs_op = list(set(equiv_exprs[1:])-set(exprs))
        exprs_op = random.sample(population=exprs_op, k=n-len(exprs))
        exprs.extend(exprs_op)
        return exprs


def main() -> None:
    if os.path.exists(path=cfg.EQUIV_EXPRS_FILTER_FILEPATH):
        logger.log_info(
            f"File '{cfg.EQUIV_EXPRS_FILTER_FILEPATH}' already exists!"
        )
        exit(1)
    if not os.path.exists(path=cfg.EQUIV_EXPRS_PROC_FILEPATH):
        logger.log_info(
            f"File '{cfg.EQUIV_EXPRS_PROC_FILEPATH}' does not exist!"
            "Run './preprocess' first to create file "
            f"'{cfg.EQUIV_EXPRS_PROC_FILEPATH}'"
        )
        exit(1)

    logger.log_info(
        f"Creating file '{cfg.EQUIV_EXPRS_FILTER_FILEPATH}'..."
    )

    file = open(file=cfg.EQUIV_EXPRS_PROC_FILEPATH, mode='r', encoding='utf-8')
    n_lines = get_n_lines(filepath=cfg.EQUIV_EXPRS_PROC_FILEPATH)

    equiv_exprs = []

    for line in tqdm(
        iterable=file,
        desc=f"[{timestamp()}] [INFO]: Reading file "
             f"'{cfg.EQUIV_EXPRS_PROC_FILEPATH}'",
        total=n_lines,
        leave=True,
        position=0,
    ):
        expr = line.strip()

        if expr:
            equiv_exprs.append(expr)
        else:
            if len(equiv_exprs) == 1:
                equiv_exprs = []
                continue

            n = get_n_exprs(expr=equiv_exprs[0], n_exprs=cfg.N_EXPRS)

            if len(equiv_exprs) > n:
                if 'd x' not in equiv_exprs[0]:
                    equiv_exprs = filter(
                        equiv_exprs=equiv_exprs,
                        operators=cfg.OPERATORS,
                        n_ops=cfg.N_OPS_PER_EXPR,
                        n=n,
                        dx=False,
                    )
                else:
                    equiv_exprs = filter(
                        equiv_exprs=equiv_exprs,
                        operators=cfg.OPERATORS,
                        n_ops=cfg.N_OPS_PER_EXPR,
                        n=n,
                        dx=True,
                    )

            write(
                filepath=cfg.EQUIV_EXPRS_FILTER_FILEPATH,
                mode='a',
                encoding='utf-8',
                exprs=equiv_exprs,
                newline=True,
            )

            equiv_exprs = []

    file.close()

    logger.log_info(
        f"Finish creating file '{cfg.EQUIV_EXPRS_FILTER_FILEPATH}'."
    )
        
    return


if __name__ == '__main__':
    main()
