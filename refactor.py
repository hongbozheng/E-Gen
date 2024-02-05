#!/usr/bin/env python3


import argparse
import config
import fractions
import glob
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


def ref_expr(equiv_exprs: list[str]) -> list[str]:
    equiv_exprs_ref = []

    for expr in equiv_exprs[:-1]:
        flag = False
        expr = expr.strip()
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
                if len(numerator) > 6 or len(denominator) > 6:
                    flag = True
                    break
                tokens[i] = f"div {numerator} {denominator}"
            elif is_int(s=token):
                token = ref_int(s=token)
                tokens[i] = token

        if not flag:
            expr = ' '.join(tokens)
            equiv_exprs_ref.append(expr+'\n')

    equiv_exprs_ref.append(equiv_exprs[-1])

    return equiv_exprs_ref


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

        equiv_exprs = []

        for line in file:
            if line.strip():
                equiv_exprs.append(line)
            else:
                equiv_exprs.append(line)

                equiv_exprs = ref_expr(equiv_exprs=equiv_exprs)

                if len(equiv_exprs) != 2:
                    for expr in equiv_exprs:
                        ref_file.write(expr)

                equiv_exprs = []

        file.close()
        ref_file.close()

    return


def main() -> None:
    if os.path.exists(path=config.DATA_REFACTORED_DIR):
        logger.log_error(f"{config.DATA_REFACTORED_DIR} directory already exists!")
        logger.log_error(f"Make sure to delete {config.DATA_REFACTORED_DIR} directory first.")
        logger.log_error("Operation aborted.")
        exit(1)

    parser = argparse.ArgumentParser(prog="create refactored dataset",
                                     description="Create refactored dataset")
    parser.add_argument("--dataset_dir", "-d", required=True, help="dataset directory")

    args = parser.parse_args()
    dataset_dir = args.dataset_dir

    logger.log_info("Refactoring expressions...")
    refactor(data_dir=dataset_dir, data_refactored_dir=config.DATA_REFACTORED_DIR)
    logger.log_info("Finish refactoring expressions.")

    return


if __name__ == "__main__":
    main()