import config
import logger
import numpy as np
import sympy as sp
from collections import OrderedDict
from sympy import Expr, Interval, S, Symbol
from sympy.calculus.util import continuous_domain
from timeout import timeout


VARIABLES = OrderedDict({
    "x": sp.Symbol("x", real=None, nonzero=None, positive=None),
})
SYMPY_OPERATORS = {
    # Elementary functions
    sp.Add: "add",
    sp.Mul: "mul",
    sp.Pow: "pow",
    sp.exp: "exp",
    sp.log: "ln",
    sp.Abs: "abs",
    sp.sign: "sign",
    # Trigonometric Functions
    sp.sin: "sin",
    sp.cos: "cos",
    sp.tan: "tan",
    sp.cot: "cot",
    sp.sec: "sec",
    sp.csc: "csc",
    # Trigonometric Inverses
    sp.asin: "asin",
    sp.acos: "acos",
    sp.atan: "atan",
    sp.acot: "acot",
    sp.asec: "asec",
    sp.acsc: "acsc",
    # Hyperbolic Functions
    sp.sinh: "sinh",
    sp.cosh: "cosh",
    sp.tanh: "tanh",
    sp.coth: "coth",
    sp.sech: "sech",
    sp.csch: "csch",
    # Hyperbolic Inverses
    sp.asinh: "asinh",
    sp.acosh: "acosh",
    sp.atanh: "atanh",
    sp.acoth: "acoth",
    sp.asech: "asech",
    sp.acsch: "acsch",
}

COEFFICIENTS = OrderedDict({
    f'a{i}': sp.Symbol(f'a{i}', real=True)
    for i in range(10)
})


def parse_int(lst):
    """
    Parse a list that starts with an integer.
    Return the integer value, and the position it ends in the list.
    """
    base = 10
    balanced = False
    val = 0
    # if first token is INT+ or INT-
    if not (balanced and lst[0] == 'INT' or base >= 2 and
            lst[0] in ['INT+', 'INT-'] or base <= -2 and lst[0] == 'INT'):
        raise Exception(f"Invalid integer in prefix expression")
    i = 0
    for x in lst[1:]:
        # if the rest part of the list is not a number, break
        if not (x.isdigit() or x[0] == '-' and x[1:].isdigit()):
            break
        # otherwise, convert the str into int
        val = val * base + int(x)
        i += 1
    if base > 0 and lst[0] == 'INT-':
        val = -val
    # i+1 is the position number ends in the list
    return val, i + 1


def write_infix(token, args):
    """
    Infix representation.
    Convert prefix expressions to a format that SymPy can parse.
    """
    if token == 'add':
        return f'({args[0]})+({args[1]})'
    elif token == 'sub' or token == 'subtract':
        return f'({args[0]})-({args[1]})'
    elif token == 'mul' or token == 'multiply':
        return f'({args[0]})*({args[1]})'
    elif token == 'div':
        return f'({args[0]})/({args[1]})'
    elif token == 'pow':
        return f'({args[0]})**({args[1]})'
    elif token == 'rac':
        return f'({args[0]})**(1/({args[1]}))'
    elif token == 'and':
        return f'({args[0]})&({args[1]})'
    elif token == 'or':
        return f'({args[0]})|({args[1]})'
    elif token == 'xor':
        return f'({args[0]})^({args[1]})'
    elif token == 'implies':
        return f'({args[0]})>>({args[1]})'
    elif token == 'not':
        return f'~({args[0]})'
    elif token == 'abs':
        return f'Abs({args[0]})'
    elif token == 'inv':
        return f'1/({args[0]})'
    elif token == 'pow2':
        return f'({args[0]})**2'
    elif token == 'pow3':
        return f'({args[0]})**3'
    elif token == 'pow4':
        return f'({args[0]})**4'
    elif token == 'pow5':
        return f'({args[0]})**5'
    elif token in ['sign', 'sqrt', 'exp', 'ln',
                   'sin', 'cos', 'tan',
                   'csc', 'sec', 'cot',
                   'sinh', 'cosh', 'tanh',
                   'csch', 'sech', 'coth',
                   'asin', 'acos', 'atan',
                   'acsc', 'asec', 'acot',
                   'asinh', 'acosh', 'atanh',
                   'acoth', 'asech', 'acsch']:
        return f'{token}({args[0]})'
    elif token == 'd':
        return f'Derivative({args[1]},{args[0]})'
    elif token == 'f':
        return f'f({args[0]})'
    elif token == 'g':
        return f'g({args[0]},{args[1]})'
    elif token == 'h':
        return f'h({args[0]},{args[1]},{args[2]})'
    elif token.startswith('INT'):
        return f'{token[-1]}{args[0]}'
    else:
        return token


def _prefix_to_infix(expr):
    """
    Parse an expression in prefix mode, and output it in either:
        - infix mode (returns human readable string)
        - develop mode (returns a dictionary with the simplified expression)
    """
    if len(expr) == 0:
        raise Exception("Empty prefix list.")
    t = expr[0]

    # OPERATOR dict, t is an operator
    if t in config.MATH_OPERATORS:
        args = []
        l1 = expr[1:]
        for _ in range(config.MATH_OPERATORS[t]):
            i1, l1 = _prefix_to_infix(l1)
            args.append(i1)
        return write_infix(t, args), l1
    # if t is variable 'x' or coefficient 'a1', 'a2'... ,
    # or constant "pi", "E", or 'I'
    elif (t in VARIABLES or t in COEFFICIENTS or t in config.CONSTANTS
          or t == 'I'):
        return t, expr[1:]
    # else when t is INT+ INT-
    else:
        val, i = parse_int(expr)
        return str(val), expr[i:]


def prefix_to_infix(expr):
    return _prefix_to_infix(expr.split(" "))


def get_sympy_local_dict() -> dict:
    local_dict = {}
    for k, v in list(VARIABLES.items()) + list(COEFFICIENTS.items()):
        assert k not in local_dict
        local_dict[k] = v
    return local_dict


def prefix_to_sympy(expr, evaluate=True):
    p, r = prefix_to_infix(expr)
    if len(r) > 0:
        raise Exception(
            f"Incorrect prefix expression \"{expr}\". \"{r}\" was not parsed."
        )

    local_dict = get_sympy_local_dict()
    expr = sp.parsing.sympy_parser.parse_expr(
        s=f'({p})',
        evaluate=evaluate,
        local_dict=local_dict
    )
    return expr


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
        logger.log_error(f"{expr}; prefix_to_sympy exception {e}")
        return False

    try:
        domain = _cont_domain(expr=expr, symbol=x, start=start, end=end)
        if isinstance(domain, sp.sets.sets.EmptySet):
            return False

    except Exception as e:
        logger.log_error(f"{expr}; continuous domain exception {e}")
        return False

    return True


def check_equiv(
        expr_pair: tuple[str, str],
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
        logger.log_error(
            f"prefix_to_sympy exception {e}; {expr_pair[0]} & {expr_pair[1]}"
        )
        return False
    try:
        expr_0 = _simplify(expr=expr_0)
        expr_1 = _simplify(expr=expr_1)
    except Exception as e:
        logger.log_error(
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
                logger.log_error(
                    f"subs_evalf, non-equiv; {expr_pair[0]} & {expr_pair[1]}"
                )
            return equiv
        except Exception as e:
            logger.log_error(
                f"_check_equiv exception {e}; {expr_pair[0]} & {expr_pair[1]}"
            )
            return False