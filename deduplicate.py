#!/usr/bin/env python3


import argparse
import config
import glob
import logger
import os
import tqdm


def deduplicate(equiv_exprs_dir: str, exprs_filepath: str, equiv_exprs_filepath: str, duplicates_filepath: str) -> None:
    pathname = os.path.join(equiv_exprs_dir, "equiv_exprs_*.txt")
    filepaths = glob.glob(pathname=pathname)

    exprs = []
    duplicates = []

    progbar = tqdm.tqdm(iterable=filepaths)

    for filepath in progbar:
        progbar.set_description(desc="[INFO]: Processing file '%s'" % filepath, refresh=True)

        input_file = open(file=filepath, mode='r')
        equiv_exprs_file = open(file=equiv_exprs_filepath, mode='w')

        equiv_exprs = []

        for line in input_file:
            if line.strip() and line not in equiv_exprs:
                equiv_exprs.append(line)
            elif not line.strip():
                equiv_exprs.append(line)

                if equiv_exprs[0] not in exprs:
                    exprs.append(equiv_exprs[0])
                    for expr in equiv_exprs:
                        equiv_exprs_file.write(expr)
                else:
                    duplicates.append(equiv_exprs[0])

                equiv_exprs = []

        input_file.close()
        equiv_exprs_file.close()

        if equiv_exprs:
            logger.log_error(f"{filepath} file is missing a '\\n' at the end of the file!")
            logger.log_error(f"Make sure all equiv_exprs_*.txt files have 2 '\\n' at the end of the file!")
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
    parser = argparse.ArgumentParser(prog="deduplicate",
                                     description="remove repetitive original expressions and their equivalent "
                                                 "expressions from generated equivalent expressions .txt files "
                                                 "under a folder")
    parser.add_argument("--equiv_exprs_dir", "-d", type=str, required=True, help="Equivalent expressions directory")

    args = parser.parse_args()
    equiv_exprs_dir = args.equiv_exprs_dir

    logger.log_info(f"Creating files '{config.EXPRS_FILEPATH}', '{config.EQUIV_EXPRS_FILEPATH}', and "
                    f"'{config.DUPLICATES_FILEPATH}'...")
    deduplicate(equiv_exprs_dir=equiv_exprs_dir, exprs_filepath=config.EXPRS_FILEPATH,
                equiv_exprs_filepath=config.EQUIV_EXPRS_FILEPATH, duplicates_filepath=config.DUPLICATES_FILEPATH)
    logger.log_info(f"Finish creating files '{config.EXPRS_FILEPATH}', '{config.EQUIV_EXPRS_FILEPATH}', and "
                    f"'{config.DUPLICATES_FILEPATH}'")

    return


if __name__ == "__main__":
    main()