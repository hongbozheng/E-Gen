#!/usr/bin/env python3


import argparse
import config
import fractions
import glob
import itertools
import logger
import os
import tqdm


def ref_int(s: str) -> str:
    if s[0] == '-':
        return "INT- " + ' '.join(s[1:])
    else:
        return "INT+ " + ' '.join(s)


def is_int(s: str) -> bool:
    if s[0] == '-' and s[1:].isdigit():
        return True
    elif s.isdigit():
        return True
    else:
        return False


def ref_expr(expr: str) -> str:
    expr = expr.replace("+", "add").replace("*", "mul").replace("/", "div")

    tokens = expr.split(sep=' ')
    for i, token in enumerate(tokens):
        if not token:
            continue
        elif token == '-':
            tokens[i] = "sub"
        elif '.' in token:
            fraction = fractions.Fraction(token)
            numerator = ref_int(s=str(fraction.numerator))
            denominator = ref_int(s=str(fraction.denominator))
            tokens[i] = f"div {numerator} {denominator}"
        elif is_int(s=token):
            token = ref_int(s=token)
            tokens[i] = token

    expr = ' '.join(tokens)

    return expr


def refactor(data_dir: str, data_refactored_dir: str) -> None:
    filepath = os.path.join(data_dir, "**", "equiv_exprs.txt")
    filepaths = glob.glob(pathname=filepath, recursive=True)

    progbar = tqdm.tqdm(iterable=filepaths)

    for filepath in progbar:
        parts = filepath.split(os.path.sep)
        cls = parts[-3]
        category = parts[-2]
        progbar.set_description(desc=f"[INFO]: Processing class '{cls}', category '{category}'", refresh=True)

        class_category = os.path.join(cls, category)
        path = os.path.join(data_refactored_dir, class_category)
        if not os.path.exists(path=path):
            os.makedirs(name=path, exist_ok=True)
        ref_filepath = os.path.join(path, "equiv_exprs.txt")

        file = open(file=filepath, mode='r')
        ref_file = open(file=ref_filepath, mode='w')

        for line in file:
            expr = ref_expr(expr=line.strip())
            ref_file.write(f"{expr}\n")

    return


def create_dataset(dataset_dir: str) -> None:
    ref_filepath = os.path.join(dataset_dir, "ref.txt")
    dataset_filepath = os.path.join(dataset_dir, "dataset.txt")

    with open(file=ref_filepath, mode='r') as input_file, open(file=dataset_filepath, mode='w') as output_file:
        exprs = []

        for expr in input_file:
            expr = expr.strip()

            if not expr:
                for pair in itertools.permutations(iterable=exprs, r=2):
                    output_file.write(f"{pair[0]}\t{pair[1]}\n")
                exprs = []
            else:
                exprs.append(expr)

    return


def main() -> None:
    parser = argparse.ArgumentParser(prog="create refactored dataset",
                                     description="Create refactored dataset")
    parser.add_argument("--dataset_dir", "-d", required=True, help="dataset directory")

    args = parser.parse_args()
    dataset_dir = args.dataset_dir

    logger.log_info("Refactoring expressions...")
    refactor(data_dir=dataset_dir, data_refactored_dir=config.DATA_REFACTORED_DIR)
    logger.log_info("Finish refactoring expressions.")
    logger.log_info("Creating dataset...")
    # create_expr_pairs(dataset_dir=config.DATA_REFACTORED_DIR)
    logger.log_info("Finish creating dataset.")

    return


if __name__ == "__main__":
    main()