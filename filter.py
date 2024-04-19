import mmap
import random
from tqdm import tqdm


def get_n_lines(filepath: str) -> int:
    fp = open(file=filepath, mode='r+')
    buf = mmap.mmap(fileno=fp.fileno(), length=0)
    n_lines = 0
    while buf.readline():
        n_lines += 1
    fp.close()

    return n_lines


def _n_exprs(expr: str, n_exprs: dict[str, dict[str, int]]) -> int:
    tokens = expr.replace("INT+ ", "").replace("INT- ", "").split(sep=' ')

    if tokens[0] != 'd':
        for token in tokens:
            if token in n_exprs:
                return n_exprs[token]['general']
        return n_exprs['poly']['general']
    else:
        for token in tokens[2:]:
            if token in n_exprs:
                return n_exprs[token]['d']
        return n_exprs['poly']['d']


def _filter(
        equiv_exprs: list[str],
        operators: list[str],
        n_ops: int,
        n: int,
        dx: bool,
) -> list[str]:
    exprs = []

    for expr in equiv_exprs[1:]:
        tokens = expr.split(sep=' ')
        op_cnt = sum(1 for token in tokens if token in operators)
        if not dx:
            if op_cnt <= n_ops:
                exprs.append(expr)
        else:
            if op_cnt <= n_ops and "d x" not in expr:
                exprs.append(expr)
    exprs.insert(0, equiv_exprs[0])

    if len(exprs) == n:
        return exprs
    elif len(exprs) > n:
        exprs = random.sample(population=exprs[1:], k=n-1)
        exprs.insert(0, equiv_exprs[0])
        return exprs
    else:
        exprs_op = list(set(equiv_exprs[1:])-set(exprs))
        exprs_op = random.sample(population=exprs_op, k=n-len(exprs))
        exprs.extend(exprs_op)
        return exprs


def filter_exprs(
        n_exprs:  dict[str, dict[str, int]],
        seed: int,
        operators: list[str],
        n_ops: int,
        raw_filepath: str,
        filtered_filepath: str,
) -> None:
    random.seed(a=seed)

    n_lines = get_n_lines(filepath=raw_filepath)
    raw_file = open(file=raw_filepath, mode='r')

    equiv_exprs = []

    for line in tqdm(
            iterable=raw_file,
            desc=f"[INFO]: Reading file '{raw_filepath}'",
            total=n_lines,
    ):
        expr = line.strip()

        if expr:
            equiv_exprs.append(expr)
        else:
            if len(equiv_exprs) == 1:
                equiv_exprs = []
                continue
            else:
                n = _n_exprs(expr=equiv_exprs[0], n_exprs=n_exprs)

                if len(equiv_exprs) > n:
                    if 'd x' not in equiv_exprs[0]:
                        equiv_exprs = _filter(
                            equiv_exprs=equiv_exprs,
                            operators=operators,
                            n_ops=n_ops,
                            n=n,
                            dx=False,
                        )
                    else:
                        equiv_exprs = _filter(
                            equiv_exprs=equiv_exprs,
                            operators=operators,
                            n_ops=n_ops,
                            n=n,
                            dx=True,
                        )

                filtered_file = open(file=filtered_filepath, mode='a')
                for expr in equiv_exprs:
                    filtered_file.write(f"{expr}\n")
                filtered_file.write("\n")
                filtered_file.close()

            equiv_exprs = []

    raw_file.close()

    return