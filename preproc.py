#!/usr/bin/env python3


import argparse
import config
import logger
import os
from filter import filter_exprs
from proc_exprs import preproc


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="preprocess",
        description="remove repetitive original expressions and their "
                    "equivalent expressions from generated equivalent "
                    "expressions .txt files under a folder")
    parser.add_argument(
        "--equiv_exprs_dir",
        "-d",
        type=str,
        required=True,
        help="Equivalent expressions directory"
    )
    parser.add_argument(
        "--refactor",
        "-r",
        action="store_true",
        default=False,
        required=False,
        help="Whether to refactor the expressions"
    )
    parser.add_argument(
        "--verify",
        "-v",
        action="store_true",
        default=False,
        required=False,
        help="Whether to verify the domain of the expressions"
    )
    parser.add_argument(
        "--filter",
        "-f",
        action="store_true",
        default=False,
        required=False,
        help="Whether to filter the expressions"
    )

    args = parser.parse_args()
    equiv_exprs_dir = args.equiv_exprs_dir
    refactor = args.refactor
    verify = args.verify
    filter = args.filter

    if not os.path.exists(path=config.EQUIV_EXPRS_VERIFIED_FILEPATH):
        logger.log_info(
            f"Creating files '{config.EXPRS_FILEPATH}', "
            f"'{config.EQUIV_EXPRS_VERIFIED_FILEPATH}', "
            f"'{config.DUPLICATES_FILEPATH}', and "
            f"'{config.INVALIDS_FILEPATH}'..."
        )

        preproc(
            equiv_exprs_dir=equiv_exprs_dir,
            refactor=refactor,
            verify=verify,
            secs=config.SECS,
            start=config.START,
            end=config.END,
            n=config.N,
            tol=config.TOL,
            invalids_filepath=config.INVALIDS_FILEPATH,
            equiv_exprs_filepath=config.EQUIV_EXPRS_VERIFIED_FILEPATH,
            duplicates_filepath=config.DUPLICATES_FILEPATH,
            exprs_filepath=config.EXPRS_FILEPATH,
        )

        logger.log_info(
            f"Finish creating files '{config.EXPRS_FILEPATH}', "
            f"'{config.EQUIV_EXPRS_VERIFIED_FILEPATH}', "
            f"'{config.DUPLICATES_FILEPATH}', and "
            f"'{config.INVALIDS_FILEPATH}'..."
        )
    else:
        logger.log_info(
            f"File '{config.EQUIV_EXPRS_VERIFIED_FILEPATH}' already exists!"
        )

    if filter:
        if not os.path.exists(config.EQUIV_EXPRS_FILTERED_FILEPATH):
            logger.log_info(
                f"Filtering file '{config.EQUIV_EXPRS_VERIFIED_FILEPATH}'..."
            )
            filter_exprs(
                n_exprs=config.N_EXPRS,
                seed=config.SEED,
                operators=config.OPERATORS,
                n_ops=config.N_OPS_PER_EXPR,
                raw_filepath=config.EQUIV_EXPRS_VERIFIED_FILEPATH,
                filtered_filepath=config.EQUIV_EXPRS_FILTERED_FILEPATH,
            )
            logger.log_info(
                f"Finish creating file "
                f"'{config.EQUIV_EXPRS_FILTERED_FILEPATH}'."
            )
        else:
            logger.log_info(
                f"File '{config.EQUIV_EXPRS_FILTERED_FILEPATH}' "
                f"already exists!"
            )

    return


if __name__ == '__main__':
    main()