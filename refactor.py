#!/usr/bin/env python3


import argparse
import config
import fractions
import itertools
import logger
import os


def rm_idt_expr(filepath: str, dataset_dir: str) -> None:
    exprs_filepath = os.path.join(dataset_dir, "exprs.txt")

    try:
        with open(file=filepath, mode='r') as input_file, open(file=exprs_filepath, mode='w') as output_file:
            for expr in input_file:
                expr = expr.strip()

                if '(' in expr:
                    expr = expr.replace("(", "").replace(")", "")
                    expr_orig = expr
                    output_file.write(f"{expr}\n")
                elif expr != expr_orig:
                    output_file.write(f"{expr}\n")

    except FileNotFoundError:
        logger.log_error(f"Input filepath {filepath} does not exist.")

    return


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


def refactor(dataset_dir: str) -> None:
    exprs_filepath = os.path.join(dataset_dir, "exprs.txt")
    ref_filepath = os.path.join(dataset_dir, "ref.txt")

    with open(file=exprs_filepath, mode='r') as exprs_file, open(file=ref_filepath, mode='w') as output_file:
        for expr in exprs_file:
            expr = expr.strip()
            expr = ref_expr(expr=expr)

            output_file.write(f"{expr}\n")

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
    parser = argparse.ArgumentParser(prog="refactor", description="refactor expressions into training form")
    parser.add_argument("--input_filepath", "-i", type=str, required=True, help="Input filepath")
    parser.add_argument("--dataset_dir", "-d", type=str, required=True, help="Dataset directory")

    args = parser.parse_args()
    filepath = args.input_filepath
    dataset_dir = args.dataset_dir

    logger.log_info("Removing identical expressions...")
    rm_idt_expr(filepath=filepath, dataset_dir=dataset_dir)
    logger.log_info("Finish removing identical expressions...")
    logger.log_info("Refactoring expressions...")
    refactor(dataset_dir=dataset_dir)
    logger.log_info("Finish refactoring expressions...")
    logger.log_info("Creating dataset ...")
    create_dataset(dataset_dir=dataset_dir)
    logger.log_info("Finish creating dataset ...")

    return

if __name__ == "__main__":
    main()