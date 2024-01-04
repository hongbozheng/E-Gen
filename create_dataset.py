#!/usr/bin/env python3


import argparse
import config
import editdistance
import logger
import os
import tqdm


def filter(equiv_exprs: list[str], n_exprs: int) -> list[str]:
    edit_dists = []
    for i, expr in enumerate(equiv_exprs[1:-1]):
        dist = editdistance.eval(a=equiv_exprs[0], b=expr)
        edit_dists.append((i, dist))

    indices_dists = sorted(edit_dists, key=lambda x: x[1], reverse=True)
    indices_dists = indices_dists[:n_exprs-1]

    equiv_exprs_filtered = []
    equiv_exprs_filtered.append(equiv_exprs[0])
    for index, _ in indices_dists:
        equiv_exprs_filtered.append(equiv_exprs[index+1])
    equiv_exprs_filtered.append('\n')

    assert len(equiv_exprs_filtered) == n_exprs+1

    return equiv_exprs_filtered


def w_data(equiv_exprs: list[str], data_processed_dir: str, class_dir: str, category_dir: str) -> None:
    path = os.path.join(data_processed_dir, class_dir, category_dir)

    if not os.path.exists(path=path):
        os.makedirs(name=path, exist_ok=True)

    filepath = os.path.join(path, "equiv_exprs.txt")
    with open(file=filepath, mode='a') as file:
        for expr in equiv_exprs:
            file.write(expr)

    return


def create_dataset(data_raw_dir: str, n_exprs: int, data_processed_dir: str) -> None:
    class_dirs = os.listdir(path=data_raw_dir)

    progbar = tqdm.tqdm(iterable=class_dirs)

    for class_dir in progbar:
        progbar.set_description(desc="[INFO]: Processing class '%s'" % class_dir, refresh=True)

        class_path = os.path.join(data_raw_dir, class_dir)
        category_dirs = os.listdir(path=class_path)

        for category_dir in category_dirs:
            filepath = os.path.join(class_path, category_dir, "equiv_exprs.txt")

            with open(file=filepath, mode='r') as file:
                equiv_exprs = []

                for line in file:
                    if line.strip() and line not in equiv_exprs:
                        equiv_exprs.append(line)
                    elif not line.strip():
                        equiv_exprs.append(line)

                        if len(equiv_exprs) == 2:
                            equiv_exprs = []
                            continue
                        elif len(equiv_exprs) > n_exprs+1:
                            equiv_exprs = filter(equiv_exprs=equiv_exprs, n_exprs=n_exprs)

                        w_data(equiv_exprs=equiv_exprs, data_processed_dir=data_processed_dir, class_dir=class_dir,
                               category_dir=category_dir)

                        equiv_exprs = []

    return


def main() -> None:
    if not os.path.exists(path=config.DATA_RAW_DIR):
        logger.log_error(f"Raw dataset directory '{config.DATA_RAW_DIR}' does not exist!")
        logger.log_error("Make sure to run './create_raw_dataset.py' first.")
        return
    if os.path.exists(path=config.DATA_PROCESSED_DIR):
        logger.log_error(f"{config.DATA_PROCESSED_DIR} directory already exists!")
        logger.log_error(f"Make sure to delete {config.DATA_PROCESSED_DIR} directory first.")
        logger.log_error("Operation aborted.")
        exit(1)

    parser = argparse.ArgumentParser(prog="create_dataset",
                                     description="Create dataset by removing expressions with 0 equivalent expressions "
                                                 "& filter them with specified limit")
    parser.add_argument("--n_exprs", "-n", type=int, required=True, help="Number of equivalent expressions to keep")

    args = parser.parse_args()
    n_exprs = args.n_exprs

    logger.log_info("Creating dataset...")
    create_dataset(data_raw_dir=config.DATA_RAW_DIR, n_exprs=n_exprs, data_processed_dir=config.DATA_PROCESSED_DIR)
    logger.log_info("Finish creating dataset.")

    return


if __name__ == "__main__":
    main()