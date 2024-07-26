from typing import Dict, List

import glob
import logger
import os
import random
from clean import clean_block, int_add_space
from filter import get_n_lines, get_n_exprs, filter
from itertools import combinations
from logger import timestamp
from refactor import ref_expr
from tqdm import tqdm
from verify import check_equiv
from write import write


def preproc(
        equiv_exprs_dir: str,
        refactor: bool,
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
    filepaths = glob.glob(pathname=pathname)
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

        equiv_exprs = []

        n_lines = get_n_lines(filepath=filepath)

        for line in tqdm(
            iterable=file,
            desc=f"[{timestamp()}] [INFO]: Reading file '{filepath}'",
            total=n_lines,
            leave=False,
            position=1,
        ):
            expr = line.strip()

            if expr:
                if refactor or verify:
                    expr = ref_expr(expr)
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

                    write(
                        filepath=equiv_exprs_filepath,
                        mode='a',
                        encoding='utf-8',
                        exprs=equiv_exprs,
                        newline=True,
                    )
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


def postproc(
        seed: int,
        n_exprs:  Dict[str, Dict[str, int]],
        operators: List[str],
        n_ops: int,
        verified_filepath: str,
        expr_pairs_filepath: str,
        incorrects_filepath: str,
) -> None:
    random.seed(a=seed)

    n_lines = get_n_lines(filepath=verified_filepath)

    verified_file = open(file=verified_filepath, mode='r')
    incorrects_file = open(file=incorrects_filepath, mode='w')

    exprs = []

    for line in tqdm(
            iterable=verified_file,
            desc=f"[INFO]: Reading file '{verified_filepath}'",
            total=n_lines,
    ):
        expr = line.strip()

        if expr:
            exprs.append(expr)
        else:
            if len(equiv_exprs) == 1:
                equiv_exprs = []
                continue
            else:
                n = get_n_exprs(expr=equiv_exprs[0], n_exprs=n_exprs)

                if len(equiv_exprs) > n:
                    if 'd x' not in equiv_exprs[0]:
                        equiv_exprs = filter(
                            equiv_exprs=equiv_exprs,
                            operators=operators,
                            n_ops=n_ops,
                            n=n,
                            dx=False,
                        )
                    else:
                        equiv_exprs = filter(
                            equiv_exprs=equiv_exprs,
                            operators=operators,
                            n_ops=n_ops,
                            n=n,
                            dx=True,
                        )

            expr_pairs = list(combinations(iterable=exprs, r=2))

            # random.shuffle(x=expr_pairs)

            expr_pairs_file = open(file=expr_pairs_filepath, mode='a')
            for expr_pair in expr_pairs:
                expr_pairs_file.write(f"{expr_pair[0]}\t{expr_pair[1]}\n")
                expr_pairs_file.write(f"{expr_pair[1]}\t{expr_pair[0]}\n")
            expr_pairs_file.write("\n")
            expr_pairs_file.close()

            exprs = []

    filtered_file.close()
    incorrects_file.close()

    return
