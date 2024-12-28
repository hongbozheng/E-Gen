#!/usr/bin/env python3


import argparse
import config as cfg
import logger
import os
from clean import clean_block, int_add_space
from glob import glob
from logger import timestamp
from mmap import mmap
from notation import ref_expr
from tqdm import tqdm
from verify import check_equiv


def get_n_lines(filepath: str) -> int:
    fp = open(file=filepath, mode='r+')
    buf = mmap(fileno=fp.fileno(), length=0)
    n_lines = 0
    while buf.readline():
        n_lines += 1
    fp.close()

    return n_lines


def preproc(
        equiv_exprs_dir: str,
        convert: bool,
        verify: bool,
        start: int,
        end: int,
        n: int,
        tol: float,
        secs: int,
        exprs_filepath: str,
        invalids_filepath: str,
        duplicates_filepath: str,
        equiv_exprs_filepath: str,
) -> None:
    pathname = os.path.join(equiv_exprs_dir, "*.txt")
    filepaths = glob(pathname=pathname)
    filepaths = [
        filepath for filepath in filepaths if not filepath.endswith("_time.txt")
    ]
    filepaths.sort()

    exprs = set()

    progbar = tqdm(iterable=filepaths, leave=True, position=0)

    for filepath in progbar:
        progbar.set_description(
            desc=f"[{timestamp()}] [INFO]: Processing file '{filepath}'",
            refresh=True,
        )

        file = open(file=filepath, mode='r', encoding="utf-8")
        exprs_file = open(file=exprs_filepath, mode='a', encoding="utf-8")
        invalids_file = open(
            file=invalids_filepath,
            mode='a',
            encoding="utf-8",
        )
        duplicates_file = open(
            file=duplicates_filepath,
            mode='a',
            encoding="utf-8",
        )

        n_lines = get_n_lines(filepath=filepath)
        equiv_exprs = []


        for line in tqdm(
            iterable=file,
            desc=f"[{timestamp()}] [INFO]: Reading file '{filepath}'",
            total=n_lines,
            leave=False,
            position=1,
        ):
            expr = line.strip()

            if expr:
                tokens = expr.split(sep=' ')
                if 'd' in tokens:
                    i = tokens.index('d')
                    if tokens[i+1] != 'x':
                        continue
                if convert or verify:
                    expr = ref_expr(expr=expr)
                if expr not in equiv_exprs:
                    equiv_exprs.append(expr)
            else:
                if equiv_exprs[0] not in exprs:
                    exprs.add(equiv_exprs[0])
                    exprs_file.write(f"{equiv_exprs[0]}\n")

                    equiv_exprs = clean_block(equiv_exprs=equiv_exprs)
                    equiv_exprs = int_add_space(equiv_exprs)

                    if verify and len(equiv_exprs) > 1:
                        verified = [equiv_exprs[0]]

                        for expr in equiv_exprs[1:]:
                            if check_equiv(
                                expr_pair=(equiv_exprs[0], expr),
                                start=start,
                                end=end,
                                n=n,
                                tol=tol,
                                secs=secs,
                            ):
                                verified.append(expr)
                            else:
                                invalids_file.write(f"{expr}\n")

                        equiv_exprs = verified

                    equiv_exprs_file = open(
                        file=equiv_exprs_filepath,
                        mode='a',
                        encoding='utf-8'
                    )
                    for expr in equiv_exprs:
                        equiv_exprs_file.write(f"{expr}\n")
                    equiv_exprs_file.write("\n")
                    equiv_exprs_file.close()
                else:
                    duplicates_file.write(f"{equiv_exprs[0]}\n")

                equiv_exprs = []

        exprs_file.close()
        invalids_file.close()
        duplicates_file.close()
        file.close()

        if equiv_exprs:
            logger.log_error(
                f"'{filepath}' file is missing a '\\n' character at the end of "
                f"the file!"
            )
            logger.log_error(
                f"Make sure all equiv_exprs_*.txt files have 2 '\\n' "
                f"characters at the end of the file!")
            logger.log_error("Operation aborted.")
            exit(1)

    return


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="preprocess",
        description="remove repetitive original expressions and their "
                    "equivalent expressions from generated equivalent "
                    "expressions .txt files under a folder"
    )
    parser.add_argument(
        "--equiv_exprs_dir",
        "-d",
        type=str,
        required=True,
        help="Equivalent expressions directory"
    )
    parser.add_argument(
        "--convert",
        "-c",
        action="store_true",
        default=False,
        required=False,
        help="Whether to convert the expressions"
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
    convert = args.convert
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
        convert=convert,
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
