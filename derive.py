#!/usr/bin/env python3


from typing import List

import config as cfg
import logger
import os
import random
from logger import timestamp
from notation import prefix_to_sympy, sympy_to_prefix
from preproc import get_n_lines
from solver import TraverseSolver
from tqdm import tqdm


def general(expr: str, error: bool) -> List[str]:
    expr_sp = prefix_to_sympy(expr=expr, evaluate=False)
    ts = TraverseSolver(expr=expr_sp)
    steps = ts.solve(display_graph=False, nshape=(2, 3))
    # steps.insert(0, expr)

    print(steps)
    print(len(steps))

    i = 0
    while i < len(steps)-1:
        if steps[i] == steps[i+1]:
            del steps[i+1]
        else:
            steps[i] = sympy_to_prefix(expr=steps[i])
            i += 1

    steps[-1] = sympy_to_prefix(expr=steps[-1])

    print(steps)
    print(len(steps))
    return steps


def derivative(expr: str, error: bool) -> List[str]:
    return


def main() -> None:
    if not os.path.exists(path=cfg.EXPRS_VAL_ML_FILEPATH):
        logger.log_info(
            f"File '{cfg.EXPRS_VAL_ML_FILEPATH}' does not exist! "
            f"Run './split' first to create file '{cfg.EXPRS_VAL_ML_FILEPATH}'."
        )
        exit(1)
    if os.path.exists(path=cfg.EXPRS_DERI_FILEPATH):
        logger.log_info(
            f"File '{cfg.EXPRS_DERI_FILEPATH}' already exists!"
        )
        exit(1)

    n_lines = get_n_lines(filepath=cfg.EXPRS_VAL_ML_FILEPATH)

    file = open(file=cfg.EXPRS_VAL_ML_FILEPATH, mode='r', encoding='utf-8')
    # deri_file = open(file=cfg.EXPRS_DERI_FILEPATH, mode='a', encoding='utf-8')

    for line in tqdm(
        iterable=file,
        desc=f"[{timestamp()}] [INFO]: Reading file "
             f"'{cfg.EXPRS_VAL_ML_FILEPATH}'",
        total=n_lines,
    ):
        expr = line.strip()

        if expr:
            error = random.random() < 0.5
            if "d x" not in expr:
                derivation = general(expr=expr, error=error)
            else:
                derivation = derivative(expr=expr, error=error)
        exit()

    # deri_file.close()
    file.close()

    return


if __name__ == "__main__":
    main()
