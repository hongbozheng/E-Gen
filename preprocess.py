#!/usr/bin/env python3


import argparse
import config
import glob
import logger
import os
import tqdm
from refactor import ref_expr
from verify import check_domain


def preprocess(
        equiv_exprs_dir: str,
        refactor: bool,
        verify: bool,
        secs: int,
        invalids_filepath,
        equiv_exprs_filepath: str,
        duplicates_filepath: str,
        exprs_filepath: str,
) -> None:
    pathname = os.path.join(equiv_exprs_dir, "*.txt")
    filepaths = glob.glob(pathname=pathname)

    filepaths = [filepath for filepath in filepaths if not filepath.endswith("_time.txt")]
    filepaths.sort()

    exprs = set()

    invalids_file = open(file=invalids_filepath, mode='w')
    equiv_exprs_file = open(file=equiv_exprs_filepath, mode='w')
    duplicates_file = open(file=duplicates_filepath, mode='w')

    progbar = tqdm.tqdm(iterable=filepaths)

    for filepath in progbar:
        progbar.set_description(desc=f"[INFO]: Processing file '{filepath}'", refresh=True)

        file = open(file=filepath, mode='r')

        equiv_exprs = []

        for line in file:
            if line.strip() and line not in equiv_exprs:
                expr = line.strip()
                if refactor and not verify:
                    expr = ref_expr(expr=expr)
                elif verify:
                    expr = ref_expr(expr=expr)
                    if check_domain(expr=expr, secs=secs):
                        equiv_exprs.append(expr)
                    else:
                        invalids_file.write(f"{expr}\n")
                        continue
                equiv_exprs.append(expr)

            elif not line.strip():
                equiv_exprs.append(line.strip())

                if len(equiv_exprs) == 1:
                    equiv_exprs = []
                    continue

                if equiv_exprs[0] not in exprs:
                    exprs.add(equiv_exprs[0])

                    for expr in equiv_exprs:
                        equiv_exprs_file.write(f"{expr}\n")
                else:
                    duplicates_file.write(f"{equiv_exprs[0]}\n")

                equiv_exprs = []

        file.close()

        if equiv_exprs:
            logger.log_error(f"'{filepath}' file is missing a '\\n' character at the end of the file!")
            logger.log_error(f"Make sure all equiv_exprs_*.txt files have 2 '\\n' characters at the end of the file!")
            logger.log_error("Operation aborted.")
            exit(1)

    invalids_file.close()
    equiv_exprs_file.close()
    duplicates_file.close()

    exprs = list(exprs)
    exprs.sort()

    exprs_file = open(file=exprs_filepath, mode='w')
    for expr in exprs:
        exprs_file.write(f"{expr}\n")
    exprs_file.close()

    return


def main() -> None:
    if os.path.exists(path=config.EQUIV_EXPRS_RAW_FILEPATH):
        logger.log_error(f"'{config.EQUIV_EXPRS_RAW_FILEPATH}' file already exists!")
        logger.log_error(f"Make sure to delete '{config.EQUIV_EXPRS_RAW_FILEPATH}' file first.")
        logger.log_error("Operation aborted.")
        exit(1)

    parser = argparse.ArgumentParser(prog="preprocess",
                                     description="remove repetitive original expressions and their equivalent "
                                                 "expressions from generated equivalent expressions .txt files "
                                                 "under a folder")
    parser.add_argument("--equiv_exprs_dir", "-d", type=str, required=True, help="Equivalent expressions directory")
    parser.add_argument("--refactor", "-r", action="store_true", default=False, required=False,
                        help="Whether to refactor the expressions")
    parser.add_argument("--verify", "-v", action="store_true", default=False, required=False,
                        help="Whether to verify the domain of the expressions")

    args = parser.parse_args()
    equiv_exprs_dir = args.equiv_exprs_dir
    refactor = args.refactor
    verify = args.verify

    logger.log_info(f"Creating files '{config.EXPRS_FILEPATH}', '{config.EQUIV_EXPRS_RAW_FILEPATH}' "
                    f"'{config.DUPLICATES_FILEPATH}', and '{config.INVALIDS_FILEPATH}'...")
    preprocess(equiv_exprs_dir=equiv_exprs_dir, refactor=refactor, verify=verify, secs=2,
               invalids_filepath=config.INVALIDS_FILEPATH, equiv_exprs_filepath=config.EQUIV_EXPRS_RAW_FILEPATH,
               duplicates_filepath=config.DUPLICATES_FILEPATH, exprs_filepath=config.EXPRS_FILEPATH)
    logger.log_info(f"Finish creating files '{config.EXPRS_FILEPATH}', '{config.EQUIV_EXPRS_RAW_FILEPATH}' "
                    f"'{config.DUPLICATES_FILEPATH}', and '{config.INVALIDS_FILEPATH}'.")

    return


if __name__ == "__main__":
    main()