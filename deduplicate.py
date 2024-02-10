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
                # rm fraction with numerator or denominator more than 1-digit
                if len(numerator) > 6 or len(denominator) > 6:
                    flag = True
                    break
                tokens[i] = f"div {numerator} {denominator}"
            elif is_int(s=token):
                token = ref_int(s=token)
                # rm integer with more than 2-digit
                if len(token) > 8:
                    flag = True
                    break
                tokens[i] = token

        if not flag:
            expr = ' '.join(tokens)
            equiv_exprs_ref.append(expr+'\n')

    equiv_exprs_ref.append(equiv_exprs[-1])

    return equiv_exprs_ref


def deduplicate(
        equiv_exprs_dir: str,
        refactor: bool,
        exprs_filepath: str,
        equiv_exprs_filepath: str,
        duplicates_filepath: str
) -> None:
    pathname = os.path.join(equiv_exprs_dir, "equiv_exprs_*.txt")
    filepaths = glob.glob(pathname=pathname)

    exprs = []
    duplicates = []

    progbar = tqdm.tqdm(iterable=filepaths)

    for filepath in progbar:
        progbar.set_description(desc=f"[INFO]: Processing file '{filepath}'", refresh=True)

        file = open(file=filepath, mode='r')
        equiv_exprs_file = open(file=equiv_exprs_filepath, mode='a')

        equiv_exprs = []

        for line in file:
            if line.strip() and line not in equiv_exprs:
                equiv_exprs.append(line)
            elif not line.strip():
                equiv_exprs.append(line)

                if equiv_exprs[0] not in exprs:
                    exprs.append(equiv_exprs[0])

                    if refactor:
                        equiv_exprs = ref_expr(equiv_exprs=equiv_exprs)
                        if len(equiv_exprs) == 1:
                            equiv_exprs = []
                            continue
                    for expr in equiv_exprs:
                        equiv_exprs_file.write(expr)
                else:
                    duplicates.append(equiv_exprs[0])

                equiv_exprs = []

        file.close()
        equiv_exprs_file.close()

        if equiv_exprs:
            logger.log_error(f"{filepath} file is missing a '\\n' character at the end of the file!")
            logger.log_error(f"Make sure all equiv_exprs_*.txt files have 2 '\\n' characters at the end of the file!")
            logger.log_error("Operation aborted.")
            exit(1)

    assert len(set(exprs)) == len(exprs)

    exprs_file = open(file=exprs_filepath, mode='w')
    for expr in exprs:
        exprs_file.write(expr)
    exprs_file.close()

    duplicates_file = open(file=duplicates_filepath, mode='w')
    for expr in duplicates:
        duplicates_file.write(expr)
    duplicates_file.close()

    return


def main() -> None:
    if os.path.exists(path=config.EQUIV_EXPRS_FILEPATH):
        logger.log_error(f"{config.EQUIV_EXPRS_FILEPATH} file already exists!")
        logger.log_error(f"Make sure to delete {config.EQUIV_EXPRS_FILEPATH} file first.")
        logger.log_error("Operation aborted.")
        exit(1)

    parser = argparse.ArgumentParser(prog="deduplicate",
                                     description="remove repetitive original expressions and their equivalent "
                                                 "expressions from generated equivalent expressions .txt files "
                                                 "under a folder")
    parser.add_argument("--equiv_exprs_dir", "-d", type=str, required=True, help="Equivalent expressions directory")
    parser.add_argument("--refactor", "-r", action="store_true", default=False, required=False,
                        help="Whether to refactor the expressions")

    args = parser.parse_args()
    equiv_exprs_dir = args.equiv_exprs_dir
    refactor = args.refactor

    logger.log_info(f"Creating files '{config.EXPRS_FILEPATH}', '{config.EQUIV_EXPRS_FILEPATH}', and "
                    f"'{config.DUPLICATES_FILEPATH}'...")
    deduplicate(equiv_exprs_dir=equiv_exprs_dir, refactor=refactor, exprs_filepath=config.EXPRS_FILEPATH,
                equiv_exprs_filepath=config.EQUIV_EXPRS_FILEPATH, duplicates_filepath=config.DUPLICATES_FILEPATH)
    logger.log_info(f"Finish creating files '{config.EXPRS_FILEPATH}', '{config.EQUIV_EXPRS_FILEPATH}', and "
                    f"'{config.DUPLICATES_FILEPATH}'.")

    return


if __name__ == "__main__":
    main()