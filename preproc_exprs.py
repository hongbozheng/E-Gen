#!/usr/bin/env python3


import argparse
import config
import fractions
import itertools
import logger


def is_int(s: str) -> bool:
    if s[0] == '-' and s[1:].isdigit():
        return True
    elif s.isdigit():
        return True
    else:
        return False


def ref_int(s: str) -> str:
    if s[0] == '-':
        return "INT- " + ' '.join(s[1:])
    else:
        return "INT+ " + ' '.join(s)


def ref_expr(expr: str) -> str:
    if '(' in expr:
        expr = expr.replace("(", "").replace(")", "")

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


def refactor(input_filepath: str, ref_filepath: str) -> None:

    try:
        with open(file=input_filepath, mode='r') as input_file, open(file=ref_filepath, mode='w') as output_file:
            for expr in input_file:
                expr = expr.strip()
                expr = ref_expr(expr=expr)

                output_file.write(f"{expr}\n")

    except FileNotFoundError:
        logger.log_error("Input filepath does not exist")

    return


def create_dataset(ref_filepath: str, dataset_filepath: str) -> None:
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


def main():
    parser = argparse.ArgumentParser(prog="refactor", description="refactor expressions into training form")
    parser.add_argument("--input_filepath", "-i", type=str, required=True, help="Input filepath")
    parser.add_argument("--ref_filepath", "-r", type=str, required=True, help="Refactor filepath")
    parser.add_argument("--dataset_filepath", "-d", type=str, required=True, help="Dataset filepath")

    args = parser.parse_args()
    input_filepath = args.input_filepath
    ref_filepath = args.ref_filepath
    dataset_filepath = args.dataset_filepath

    refactor(input_filepath=input_filepath, ref_filepath=ref_filepath)
    create_dataset(ref_filepath=ref_filepath, dataset_filepath=dataset_filepath)

    return

if __name__ == "__main__":
    main()