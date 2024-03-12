#!/usr/bin/env python3


import argparse
import config
import csv
import glob
import logger
import os
import tqdm
import matplotlib.pyplot as plt


def stats(dataset_dir: str) -> tuple[dict, dict]:
    filepath = os.path.join(dataset_dir, "**", "equiv_exprs.txt")
    filepaths = glob.glob(pathname=filepath, recursive=True)

    stats = {}
    stats_op = {}

    progbar = tqdm.tqdm(iterable=filepaths)

    for filepath in progbar:
        parts = filepath.split(sep=os.path.sep)
        cls = parts[-3]
        category = parts[-2]
        progbar.set_description(desc=f"[INFO]: Processing class '{cls}', category '{category}'", refresh=True)

        if cls == "poly":
            n_ops = 0
        else:
            n_ops = len(cls.split(sep='_'))

        file = open(file=filepath, mode='r')

        exprs = set()
        n_expr_pairs = 0

        for line in file:
            expr_pair = line.strip().split(sep='\t')
            exprs.add(expr_pair[0])
            n_expr_pairs += 1

        info = {"n_exprs": len(exprs), "n_expr_pairs": n_expr_pairs}
        if cls not in stats:
            stats[cls] = {}
        stats[cls][category] = info

        info_op = {"n_exprs": 0, "n_expr_pairs": 0}
        if n_ops not in stats_op:
            stats_op[n_ops] = info_op
        stats_op[n_ops]["n_exprs"] += len(exprs)
        stats_op[n_ops]["n_expr_pairs"] += n_expr_pairs

        file.close()

    return stats, stats_op


def pt_stats(stats: dict, stats_op: dict) -> None:
    n_exprs = []
    n_expr_pairs = []
    categories = []

    logger.log_info("Class               | Category | N Exprs | N Expr Pairs")
    for cls in sorted(stats):
        for category in stats[cls]:
            logger.log_info(f"{cls:<19} | {category:<8} | {stats[cls][category]['n_exprs']:<7} | "
                            f"{stats[cls][category]['n_expr_pairs']:<12}")
            n_exprs.append(stats[cls][category]['n_exprs'])
            n_expr_pairs.append(stats[cls][category]['n_expr_pairs'])
            categories.append(f"{cls}, {category}")
    logger.log_info("------------------------------------------------")
    logger.log_info(f"Total               |          | {sum(n_exprs):<7} | {sum(n_expr_pairs):<12}\n")

    data = list(zip(categories, n_exprs, n_expr_pairs))
    file = open(file="stats.csv", mode='w', newline='')
    writer = csv.writer(file)
    writer.writerow(["Category", "Number of Expressions", "Number of Expression Pairs"])
    for row in data:
        writer.writerow(row)
    writer.writerow(["Total", sum(n_exprs), sum(n_expr_pairs)])
    file.close()

    n_exprs = []
    n_expr_pairs = []

    logger.log_info("N OP  | N Exprs | N Expr Pairs")
    for n_ops in sorted(stats_op):
        logger.log_info(f"{n_ops:<5} | {stats_op[n_ops]['n_exprs']:<7} | {stats_op[n_ops]['n_expr_pairs']:<12}")
        n_exprs.append(stats_op[n_ops]['n_exprs'])
        n_expr_pairs.append(stats_op[n_ops]['n_expr_pairs'])
    logger.log_info("-------------------------------")
    logger.log_info(f"Total | {sum(n_exprs):<7} | {sum(n_expr_pairs):<12}")

    # plt.rc(group="font", family="serif")
    # plt.rc(group="text", usetex=True)
    # fig, ax = plt.subplots(figsize=(30, 10))
    # bars = ax.bar(categories, n_exprs, align="center")
    # for bar in bars:
    #     plt.text(bar.get_width()+20, bar.get_y() + bar.get_height()/2, s=f'{bar.get_width():.0f}', ha='left', va='center')
    # plt.xlabel("Number of Expressions")
    # plt.xticks(rotation=90)
    # plt.ylabel("Categories")
    # plt.title("Number of Expressions vs Categories")
    # plt.show()

    # fig, ax = plt.subplots(figsize=(10, 15))
    # bars = ax.barh(categories, n_expr_pairs, align="center")
    # for bar in bars:
    #     plt.text(bar.get_width()+3500, bar.get_y() + bar.get_height()/2, s=f'{bar.get_width():.0f}', ha='left', va='center')
    # plt.xlabel("Number of Expression Pairs")
    # plt.ylabel("Categories")
    # plt.title("Number of Expression Pairs vs Categories")
    # plt.show()

    return


def main() -> None:
    parser = argparse.ArgumentParser(prog="stats", description="Calculate statistics of dataset")
    parser.add_argument("--dataset_dir", "-d", required=True, help="dataset directory")

    args = parser.parse_args()
    dataset_dir = args.dataset_dir

    stats_, stats_op = stats(dataset_dir=dataset_dir)
    pt_stats(stats=stats_, stats_op=stats_op)

    return


if __name__ == "__main__":
    main()