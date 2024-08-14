#!/usr/bin/env python3


from typing import Dict, List, Tuple

import argparse
import config
import csv
import logger


def pt_stats(stats_: Dict, stats_op: Dict) -> None:
    logger.log_info("Operator | Expression | Expression Pairs")

    file = open(file="stats.csv", mode='w', newline='')
    writer = csv.writer(file)
    writer.writerow(["Operator", "Expression", "Expression Pairs"])

    for op in stats_:
        logger.log_info(f"{op:<8} | {stats_[op][0]:<10} | {stats_[op][1]}")
        writer.writerow([f"{op}", stats_[op][0], stats_[op][1]])
    file.close()

    n_exprs = 0
    n_expr_pairs = 0

    logger.log_info("========================================")
    logger.log_info("# of ops | Expression | Expression Pairs")
    for op in stats_op:
        n_exprs += stats_op[op][0]
        n_expr_pairs += stats_op[op][1]
        logger.log_info(f"{op:<8} | {stats_op[op][0]:<10} | {stats_op[op][1]}")
    logger.log_info(f"Total    | {n_exprs:<10} | {n_expr_pairs}")

    return


def stats(operators: List[str], filepath: str) -> Tuple[Dict, Dict]:
    stats_ = {op: [0, 0] for op in operators}
    stats_['poly'] = [0, 0]
    stats_['d'] = [0, 0]
    stats_op = {}

    file = open(file=filepath, mode='r')

    exprs = []

    for line in file:
        expr = line.strip()
        if expr:
            exprs.append(expr)
        else:
            for expr in exprs:
                n_ops = 0
                tokens = expr.split(sep=' ')
                for token in tokens:
                    if token in operators:
                        stats_[token][0] += 1
                        stats_[token][1] += len(exprs)-1
                        n_ops += 1
                    if token == 'd':
                        stats_['d'][0] += 1
                        stats_['d'][1] += len(exprs)-1
                if n_ops == 0:
                    stats_['poly'][0] += 1
                    stats_['poly'][1] += len(exprs)-1
                if n_ops not in stats_op:
                    stats_op[n_ops] = [1, len(exprs)-1]
                else:
                    stats_op[n_ops][0] += 1
                    stats_op[n_ops][1] += len(exprs)-1

            exprs = []

    file.close()

    return stats_, dict(sorted(stats_op.items()))


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="stats",
        description="Calculate statistics of equivalent expressions "
                    "or equivalent expression pairs",
    )
    parser.add_argument(
        "--filter",
        "-f",
        action="store_true",
        default=False,
        required=False,
        help="Whether to perform calculation on filtered expressions"
    )

    args = parser.parse_args()
    filter = args.filter

    if filter:
        stats_, stats_op = stats(
            operators=config.FUNC_OPS,
            filepath=config.EQUIV_EXPRS_FILTERED_FILEPATH,
        )
    else:
        stats_, stats_op = stats(
            operators=config.FUNC_OPS,
            filepath=config.EQUIV_EXPRS_RAW_FILEPATH,
        )
    pt_stats(stats_=stats_, stats_op=stats_op)

    return


if __name__ == "__main__":
    main()
