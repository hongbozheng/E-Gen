#!/usr/bin/env python3


import argparse
import config as cfg
import logger
import os
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

    args = parser.parse_args()
    equiv_exprs_dir = args.equiv_exprs_dir
    refactor = args.refactor
    verify = args.verify

    if os.path.exists(path=cfg.EQUIV_EXPRS_PROC_FILEPATH):
        logger.log_info(
            f"File '{cfg.EQUIV_EXPRS_PROC_FILEPATH}' already exists!"
        )
        exit(1)
    
    logger.log_info(
        f"Creating files '{cfg.EXPRS_FILEPATH}', "
        f"'{cfg.EQUIV_EXPRS_PROC_FILEPATH}', "
        f"'{cfg.DUPLICATES_FILEPATH}', and "
        f"'{cfg.INVALIDS_FILEPATH}'..."
    )

    preproc(
        equiv_exprs_dir=equiv_exprs_dir,
        refactor=refactor,
        verify=verify,
        secs=cfg.SECS,
        start=cfg.START,
        end=cfg.END,
        n=cfg.N,
        tol=cfg.TOL,
        exprs_filepath=cfg.EXPRS_FILEPATH,
        invalids_filepath=cfg.INVALIDS_FILEPATH,
        duplicates_filepath=cfg.DUPLICATES_FILEPATH,
        equiv_exprs_filepath=cfg.EQUIV_EXPRS_PROC_FILEPATH,
    )

    logger.log_info(
        f"Finish creating files '{cfg.EXPRS_FILEPATH}', "
        f"'{cfg.EQUIV_EXPRS_PROC_FILEPATH}', "
        f"'{cfg.DUPLICATES_FILEPATH}', and "
        f"'{cfg.INVALIDS_FILEPATH}'..."
    )   

    return


if __name__ == '__main__':
    main()
