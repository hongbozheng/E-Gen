from typing import Tuple

import logger
import numpy as np
import sympy as sp
from notation import VARIABLES, prefix_to_sympy
from sympy import Expr, Interval, Symbol
from sympy.calculus.util import continuous_domain
from timeout import timeout


def check_domain(expr: str, secs: int, start: float, end: float) -> bool:
    @timeout(secs=secs)
    def _cont_domain(expr: Expr, symbol: Symbol, start: float, end: float):
        return continuous_domain(
            f=expr,
            symbol=symbol,
            domain=Interval(
                start=start,
                end=end,
                left_open=False,
                right_open=False
            )
        )

    x = VARIABLES['x']

    try:
        expr = prefix_to_sympy(expr=expr)
    except Exception as e:
        logger.log_debug(f"{expr}; prefix_to_sympy exception {e}")
        return False

    try:
        domain = _cont_domain(expr=expr, symbol=x, start=start, end=end)
        if isinstance(domain, sp.sets.sets.EmptySet):
            return False

    except Exception as e:
        logger.log_debug(f"{expr}; continuous domain exception {e}")
        return False

    return True


def check_equiv(
        expr_pair: Tuple[str, str],
        secs: int,
        start: float,
        end: float,
        n: int,
        tol: float,
) -> bool:
    @timeout(secs=secs)
    def _simplify(expr: Expr) -> Expr:
        return sp.simplify(expr=expr)

    # @timeout(secs=secs)
    # def _cont_domain(expr: Expr, symbol: Symbol, start: float, end: float):
    #     return continuous_domain(
    #         f=expr,
    #         symbol=symbol,
    #         domain=Interval(
    #             start=start,
    #             end=end,
    #             left_open=False,
    #             right_open=False
    #         )
    #     )

    @timeout(secs=secs)
    def _check_equiv(
            x: Symbol,
            expr: Expr,
            start: float,
            end: float,
            n: int,
            tol: float,
    ) -> bool:
        rand_nums = np.random.uniform(low=start, high=end, size=n)
        for num in rand_nums:
            val = expr.subs(x, num).evalf()
            if abs(val) > tol:
                return False

        return True

    x = VARIABLES['x']

    try:
        expr_0 = prefix_to_sympy(expr=expr_pair[0])
        expr_1 = prefix_to_sympy(expr=expr_pair[1])
    except Exception as e:
        logger.log_debug(
            f"prefix_to_sympy exception {e}; {expr_pair[0]} & {expr_pair[1]}"
        )
        return False
    try:
        expr_0 = _simplify(expr=expr_0)
        expr_1 = _simplify(expr=expr_1)
    except Exception as e:
        logger.log_debug(
            f"simplify exception {e}; {expr_pair[0]} & {expr_pair[1]}"
        )
        return False

    expr = expr_0 - expr_1

    if expr == 0:
        logger.log_debug(
            f"simplify  , equiv    ; {expr_pair[0]} & {expr_pair[1]}"
        )
        return True
    else:
        try:
            equiv = _check_equiv(
                x=x,
                expr=expr,
                start=start,
                end=end,
                n=n,
                tol=tol
            )
            if equiv:
                logger.log_debug(
                    f"subs_evalf, equiv    ; {expr_pair[0]} & {expr_pair[1]}"
                )
            else:
                logger.log_debug(
                    f"subs_evalf, non-equiv; {expr_pair[0]} & {expr_pair[1]}"
                )
            return equiv
        except Exception as e:
            logger.log_debug(
                f"_check_equiv exception {e}; {expr_pair[0]} & {expr_pair[1]}"
            )
            return False
