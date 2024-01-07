#!/usr/bin/env python3


import argparse
import config
import glob
import logger
import math
import os
import tqdm
import matplotlib.pyplot as plt


def cal_stats(dataset_dir: str) -> dict:
    filepath = os.path.join(dataset_dir, "**", "equiv_exprs.txt")
    filepaths = glob.glob(pathname=filepath, recursive=True)

    stats = {}

    progbar = tqdm.tqdm(iterable=filepaths)

    for filepath in progbar:
        parts = filepath.split(os.path.sep)
        cls = parts[-3]
        category = parts[-2]
        progbar.set_description(desc=f"[INFO]: Processing class '{cls}', category '{category}'", refresh=True)

        file = open(file=filepath, mode='r')

        n_exprs = 0
        n_expr_pairs = 0
        equiv_exprs = []

        for line in file:
            if line.strip() and line not in equiv_exprs:
                equiv_exprs.append(line)
            elif not line.strip():
                n_exprs += 1
                n_expr_pairs += math.perm(len(equiv_exprs), 2)
                equiv_exprs = []

        info = {"n_exprs": n_exprs, "n_expr_pairs": n_expr_pairs}
        if cls not in stats:
            stats[cls] = {}
        stats[cls][category] = info

        file.close()

    return stats


def pt_stats(stats: dict) -> None:
    keys = sorted(stats)

    n_exprs = []
    n_expr_pairs = []
    categories = []

    logger.log_info("Class | Category | N Exprs | N Expr Pairs")
    for cls in keys:
        for category in stats[cls]:
            logger.log_info(f"{cls:<5} | {category:<8} | {stats[cls][category]['n_exprs']:<7} | "
                            f"{stats[cls][category]['n_expr_pairs']:<12}")
            n_exprs.append(stats[cls][category]['n_exprs'])
            n_expr_pairs.append(stats[cls][category]['n_expr_pairs'])
            categories.append(f"{cls}, {category}")
    logger.log_info("-----------------------------------")
    logger.log_info(f"Total |          | {sum(n_exprs):<7} | {sum(n_expr_pairs):<12}")

    plt.rc(group="font", family="serif")
    plt.rc(group="text", usetex=True)
    fig, ax = plt.subplots(figsize=(15, 8))
    bars = ax.barh(categories, n_exprs, align="center")
    for bar in bars:
        plt.text(bar.get_width()+20, bar.get_y() + bar.get_height()/2, f'{bar.get_width():.0f}', ha='left', va='center')
    plt.xlabel("Number of Expressions")
    plt.ylabel("Categories")
    plt.title("Number of Expressions vs Categories")
    plt.show()

    fig, ax = plt.subplots(figsize=(15, 8))
    bars = ax.barh(categories, n_expr_pairs, align="center")
    for bar in bars:
        plt.text(bar.get_width()+3500, bar.get_y() + bar.get_height()/2, f'{bar.get_width():.0f}', ha='left', va='center')
    plt.xlabel("Number of Expression Pairs")
    plt.ylabel("Categories")
    plt.title("Number of Expression Pairs vs Categories")
    plt.show()

    return


def main() -> None:
    parser = argparse.ArgumentParser(prog="stats", description="Calculate statistics of dataset")
    parser.add_argument("--dataset_dir", "-d", required=True, help="dataset directory")

    args = parser.parse_args()
    dataset_dir = args.dataset_dir

    stats = cal_stats(dataset_dir=dataset_dir)
    pt_stats(stats=stats)

    return


if __name__ == "__main__":
    main()