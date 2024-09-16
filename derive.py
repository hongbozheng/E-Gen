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
from sympy import Expr
from tqdm import tqdm


def hack(expr: str) -> Expr:
    tokens = expr.split(sep=' ')
    # print("tokens", tokens)
    id_op = {i: op for i, op in enumerate(tokens) if op in cfg.FUNC_OPS}
    if id_op:
        src_id = random.choice(seq=list(id_op.keys()))
        tgt_ops = [op for op in cfg.FUNC_OPS if op != id_op[src_id]]
        tgt_op = random.choice(seq=tgt_ops)
        tokens[src_id] = tgt_op
    else:
        print("IN ELSE SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS")
        id_op = {
            i: op for i, op in enumerate(tokens) if op in cfg.ARITH_OPS
        }
        src_id = random.choice(seq=list(id_op.keys()))
        tgt_ops = [op for op in cfg.ARITH_OPS if op != id_op[src_id]]
        tgt_op = random.choice(seq=tgt_ops)
        tokens[src_id] = tgt_op

    expr_hack = ' '.join(tokens)
    if expr_hack == expr:
        print("SAME SAME SAME !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!")

    return expr_hack


def general(expr: str, err: bool) -> List[str]:
    expr_sp = prefix_to_sympy(expr=expr, evaluate=False)
    ts = TraverseSolver(expr=expr_sp)
    steps = ts.solve(display_graph=False, nshape=(2, 3))

    # print(steps)
    # print(len(steps))

    for i in range(len(steps)):
        steps[i] = sympy_to_prefix(expr=steps[i])

    if len(steps) != len(set(steps)):
        print("DUPLICATED STEPS 1")

    # i = 0
    # while i < len(steps)-1:
    #     if steps[i] == steps[i+1]:
    #         del steps[i+1]
    #     else:
    #         steps[i] = sympy_to_prefix(expr=steps[i])
    #         i += 1
    # steps[-1] = sympy_to_prefix(expr=steps[-1])

    if len(steps) <= 1:
        return [steps[0] + "\t-1"]

    if err:
        gt = random.randint(a=1, b=len(steps)-1)
        # gt=4
        # print("gt", gt)
        steps = steps[:gt+1]
        # print("keep", steps)
        # print("original", steps[gt])
        steps[gt] = hack(expr=steps[gt])
        # print("hack    ", steps[gt])

        step_sp = prefix_to_sympy(expr=steps[gt], evaluate=False)
        # print(step_sp)

        ts = TraverseSolver(expr=step_sp)
        err_steps = ts.solve(display_graph=False, nshape=(2, 3))
        # print(len(err_steps))

        for i in range(len(err_steps)):
            err_steps[i] = sympy_to_prefix(expr=err_steps[i])

        if len(err_steps) != len(set(err_steps)):
            print("DUPLICATED STEPS 2")

        steps.extend(err_steps)

        # i = gt
        # while i < len(steps)-1:
        #     if steps[i] == steps[i+1]:
        #         del steps[i+1]
        #     else:
        #         steps[i] = sympy_to_prefix(expr=steps[i])
        #         i += 1
        # steps[-1] = sympy_to_prefix(expr=steps[-1])

    if not err:
        steps[0] += "\t-1"
    else:
        steps[0] += f"\t{gt-1}"

    # print(steps)
    # print(len(steps))
    return steps


def derivative(expr: str, err: bool) -> List[str]:
    return


def main() -> None:
    if not os.path.exists(path=cfg.EXPRS_ML_FILEPATH):
        logger.log_info(
            f"File '{cfg.EXPRS_ML_FILEPATH}' does not exist! "
            f"Run './split' first to create file '{cfg.EXPRS_ML_FILEPATH}'."
        )
        exit(1)
    if os.path.exists(path=cfg.EXPRS_DERI_FILEPATH):
        logger.log_info(
            f"File '{cfg.EXPRS_DERI_FILEPATH}' already exists!"
        )
        exit(1)

    logger.log_info("Start generating derivations...")

    n_lines = get_n_lines(filepath=cfg.EXPRS_ML_FILEPATH)

    file = open(file=cfg.EXPRS_ML_FILEPATH, mode='r', encoding='utf-8')
    deri_file = open(file=cfg.EXPRS_DERI_FILEPATH, mode='a', encoding='utf-8')

    for line in tqdm(
        iterable=file,
        desc=f"[{timestamp()}] [INFO]: Reading file "
             f"'{cfg.EXPRS_ML_FILEPATH}'",
        total=n_lines,
    ):
        expr = line.strip()
        if expr:
            # print("expr", expr)
            err = random.random() < 1.0
            if "d x " not in expr and "d " not in expr:
                steps = general(expr=expr, err=err)
                # print(steps[0])
                # print("after", len(steps))
            else:
                steps = []
            # else:
            #     print("herere")
            #     pass
                # steps = derivative(expr=expr, err=err)

            if len(steps) >= 4:
                for step in steps:
                    deri_file.write(f"{step}\n")
                deri_file.write('\n')
        # exit()

    deri_file.close()
    file.close()

    logger.log_info("Finish generating derivations.")

    return


if __name__ == "__main__":
    main()
