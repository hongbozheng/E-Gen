import glob
import logger
import os
import random
from filter import get_n_lines
from itertools import combinations
from refactor import ref_expr
from tqdm import tqdm
from verify import check_domain, verify_pair


def preproc(
        equiv_exprs_dir: str,
        refactor: bool,
        verify: bool,
        secs: int,
        start: int,
        end: int,
        invalids_filepath,
        equiv_exprs_filepath: str,
        duplicates_filepath: str,
        exprs_filepath: str,
) -> None:
    pathname = os.path.join(equiv_exprs_dir, "*.txt")
    filepaths = glob.glob(pathname=pathname)

    filepaths = [
        filepath for filepath in filepaths if not filepath.endswith("_time.txt")
    ]
    filepaths.sort()

    exprs = set()

    duplicates_file = open(file=duplicates_filepath, mode='w')

    progbar = tqdm(iterable=filepaths, position=0)

    for filepath in progbar:
        progbar.set_description(
            desc=f"[INFO]: Processing file '{filepath}'",
            refresh=True
        )

        file = open(file=filepath, mode='r')
        invalids_file = open(file=invalids_filepath, mode='a')

        equiv_exprs = []

        for line in file:
            expr = line.strip()

            if expr:
                if refactor and not verify:
                    expr = ref_expr(expr=expr)
                elif verify:
                    expr = ref_expr(expr=expr)
                    if check_domain(expr=expr, secs=secs, start=start, end=end):
                        equiv_exprs.append(expr)
                    else:
                        invalids_file.write(f"{expr}\n")
                        continue
                if expr not in equiv_exprs:
                    equiv_exprs.append(expr)
            else:
                if equiv_exprs:
                    if equiv_exprs[0] not in exprs:
                        exprs.add(equiv_exprs[0])
                        equiv_exprs_file = open(file=equiv_exprs_filepath, mode='a')
                        for expr in equiv_exprs:
                            equiv_exprs_file.write(f"{expr}\n")
                        equiv_exprs_file.write("\n")
                        equiv_exprs_file.close()
                    else:
                        duplicates_file.write(f"{equiv_exprs[0]}\n")

                equiv_exprs = []

        invalids_file.close()
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

    duplicates_file.close()

    exprs = list(exprs)
    exprs.sort()

    exprs_file = open(file=exprs_filepath, mode='w')
    for expr in exprs:
        exprs_file.write(f"{expr}\n")
    exprs_file.close()

    return


def postproc(
        seed: int,
        verify: bool,
        start: float,
        end: float,
        n: int,
        tol: float,
        secs: int,
        filtered_filepath: str,
        expr_pairs_filepath: str,
        incorrects_filepath: str,
) -> None:
    random.seed(a=seed)

    n_lines = get_n_lines(filepath=filtered_filepath)

    filtered_file = open(file=filtered_filepath, mode='r')
    incorrects_file = open(file=incorrects_filepath, mode='w')

    exprs = []

    for line in tqdm(
            iterable=filtered_file,
            desc=f"[INFO]: Reading file '{filtered_filepath}'",
            total=n_lines,
    ):
        expr = line.strip()

        if expr:
            exprs.append(expr)
        else:
            if verify:
                pairs = list(combinations(iterable=exprs, r=2))

                expr_pairs = []

                for expr_pair in pairs:
                    if verify_pair(
                        expr_pair=expr_pair,
                        start=start,
                        end=end,
                        n=n,
                        tol=tol,
                        secs=secs,
                    ):
                        expr_pairs.append(expr_pair)
                    else:
                        incorrects_file.write(
                            f"{expr_pair[0]}\t{expr_pair[1]}\n"
                        )
            else:
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