import editdistance


def _category(expr: str) -> str:
    tokens = expr.replace("INT+ ", "").replace("INT- ", "").split(sep=' ')

    if tokens[0] == 'd':
        if len(tokens[2:]) in [1, 3, 5, 7, 9, 11]:
            return f"poly_d_{len(tokens[2:])}"
        elif len(tokens[2:]) in [2, 4, 6, 8]:
            return f"op_d_{len(tokens[2:])}"
    else:
        if len(tokens) in [1, 3, 5, 7, 9, 11]:
            return f"poly_{len(tokens)}"
        elif len(tokens) in [2, 4, 6, 8]:
            return f"op_{len(tokens)}"


def _edit_dist(orig_expr: str, exprs: list[str], n: int) -> list[str]:
    idx_dist = []

    for i, expr in enumerate(exprs):
        dist = editdistance.eval(a=orig_expr, b=expr)
        idx_dist.append((i, dist))

    idx_dist = sorted(idx_dist, key=lambda x: x[1], reverse=True)
    idx_dist = idx_dist[:n]

    exprs_filtered = [exprs[i] for (i, _) in idx_dist]

    assert len(exprs_filtered) == n

    return exprs_filtered


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
        exprs = _edit_dist(orig_expr=equiv_exprs[0], exprs=exprs[1:], n=n-1)
        exprs.insert(0, equiv_exprs[0])
        return exprs
    else:
        exprs_op = list(set(equiv_exprs[1:])-set(exprs))
        k = n-len(exprs)
        exprs_op = _edit_dist(orig_expr=equiv_exprs[0], exprs=exprs_op, n=k)
        exprs.extend(exprs_op)
        return exprs


def filter(
        equiv_exprs_raw_filepath: str,
        n_exprs: dict,
        operators: list[str],
        n_ops: int,
        equiv_exprs_filtered_filepath: str,
) -> None:
    raw_file = open(file=equiv_exprs_raw_filepath, mode='r')
    filtered_file = open(file=equiv_exprs_filtered_filepath, mode='w')

    equiv_exprs = []

    for line in raw_file:
        expr = line.strip()

        if expr:
            equiv_exprs.append(expr)
        else:
            if len(equiv_exprs) == 1:
                equiv_exprs = []
                continue
            else:
                category = _category(expr=equiv_exprs[0])
                n = n_exprs[category]

                if len(equiv_exprs) > n:
                    if 'd' not in category:
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

                for expr in equiv_exprs:
                    filtered_file.write(f"{expr}\n")
                filtered_file.write("\n")

            equiv_exprs = []

    raw_file.close()
    filtered_file.close()

    return