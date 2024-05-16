#!/usr/bin/env python3


import argparse
import config
import logger
import os
from proc_exprs import postproc


def main():
    parser = argparse.ArgumentParser(
        prog="postproc",
        description="create expression pairs and verify if they are "
                    "equivalent")
    parser.add_argument(
        "--verify",
        "-v",
        action="store_true",
        default=False,
        required=False,
        help="Whether to verify the domain of the expressions"
    )
    args = parser.parse_args()
    verify = args.verify

    if not os.path.exists(path=config.EXPR_PAIRS_FILEPATH):
        if verify:
            logger.log_info(f"Verifying expression pairs and creating file "
                            f"'{config.EXPR_PAIRS_FILEPATH}'...")
        else:
            logger.log_info(f"Creating file '{config.EXPR_PAIRS_FILEPATH}'...")

        postproc(
            seed=config.SEED,
            verify=verify,
            start=config.START,
            end=config.END,
            n=config.N,
            tol=config.TOL,
            secs=config.SECS,
            filtered_filepath=config.EQUIV_EXPRS_FILTERED_FILEPATH,
            expr_pairs_filepath=config.EXPR_PAIRS_FILEPATH,
            incorrects_filepath=config.INCORRECTS_FILEPATH,
        )

        if verify:
            logger.log_info(
                f"Finish verifying expression pairs and creating file "
                f"'{config.EXPR_PAIRS_FILEPATH}'."
            )
        else:
            logger.log_info(
                f"Finish creating file '{config.EXPR_PAIRS_FILEPATH}'."
            )
    else:
        logger.log_info(
            f"File '{config.EXPR_PAIRS_FILEPATH}' "
            f"already exists!"
        )

    return


if __name__ == '__main__':
    main()