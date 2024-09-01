#!/usr/bin/env python3


from typing import Dict, List, Tuple

import argparse
import config as cfg
import csv
import logger
import matplotlib.pyplot as plt


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


def pt_stats(
        stats_: Dict[str, List[int]],
        stats_op: Dict[int, List[int]],
) -> None:
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


def plt_stats(
        stats_: Dict[str, List[int]],
        stats_op: Dict[int, List[int]],
) -> None:
    x = list(stats_.keys())
    y = [vals[0] for vals in stats_.values()]

    colors = {
        "Arithmetic": '#377eb8',  # blue
        "Logarithmic": '#ff7f00',  # orange
        "Trigonometric": '#f781bf',  # pink
        "Inverse Trigonometric": "#4daf4a",  # green
        "Hyperbolic": '#dede00',  # yellow
        "Inverse Hyperbolic": '#984ea3',  # purple
    }
    bar_colors = (
        [colors["Arithmetic"]]*5 +
        [colors["Logarithmic"]]*1 +
        [colors["Trigonometric"]]*6 +
        [colors["Inverse Trigonometric"]]*6 +
        [colors["Hyperbolic"]]*6 +
        [colors["Inverse Hyperbolic"]]*6
    )

    legend = [
        plt.Line2D(
            xdata=[],
            ydata=[],
            color=color,
            lw=0,
            marker='s',
            markersize=10,
        )
        for color in colors.values()
    ]

    plt.figure(figsize=(15, 10))
    plt.bar(x, y, color=bar_colors)

    plt.xlabel(xlabel="Operator")
    plt.xticks(rotation=-90)
    plt.ylabel(ylabel="Number of Expressions")
    plt.legend(handles=legend, labels=colors.keys(), loc=1)

    plt.tight_layout()
    plt.savefig(fname="a.svg", transparent=True, dpi=500, format="svg")

    return


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="stats",
        description="Calculate statistics of equivalent expressions "
                    "or equivalent expression pairs",
    )
    parser.add_argument(
        "--filepath",
        "-f",
        type=str,
        required=True,
        help="dataset filepath",
    )

    args = parser.parse_args()
    filepath = args.filepath

    stats_, stats_op = stats(
        operators=cfg.ARITH_OPS+cfg.FUNC_OPS,
        filepath=filepath,
    )

    pt_stats(stats_=stats_, stats_op=stats_op)

    plt_stats(stats_=stats_, stats_op=stats_op)

    return


if __name__ == "__main__":
    main()
