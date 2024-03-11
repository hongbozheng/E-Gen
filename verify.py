#!/usr/bin/env python3


import config
import glob
import itertools
import logger
import numpy
import os
import sympy as sp
import tqdm
from collections import OrderedDict
from sympy import Expr, Interval, S, Symbol
from sympy.calculus.util import continuous_domain
from timeout import timeout


OPERATORS = {
    # Elementary functions
    "add": 2,
    "sub": 2,
    "mul": 2,
    "div": 2,
    "d": 2,
    "pow": 2,
    "rac": 2,
    "inv": 1,
    "pow2": 1,
    "pow3": 1,
    "pow4": 1,
    "pow5": 1,
    "sqrt": 1,
    "exp": 1,
    "ln": 1,
    "abs": 1,
    "sign": 1,
    # Trigonometric Functions
    "sin": 1,
    "cos": 1,
    "tan": 1,
    "cot": 1,
    "sec": 1,
    "csc": 1,
    # Trigonometric Inverses
    "asin": 1,
    "acos": 1,
    "atan": 1,
    "acot": 1,
    "asec": 1,
    "acsc": 1,
    # Hyperbolic Functions
    "sinh": 1,
    "cosh": 1,
    "tanh": 1,
    "coth": 1,
    "sech": 1,
    "csch": 1,
    # Hyperbolic Inverses
    "asinh": 1,
    "acosh": 1,
    "atanh": 1,
    "acoth": 1,
    "asech": 1,
    "acsch": 1,
}
CONSTANTS = ["pi", "e"]
VARIABLES = OrderedDict({
    "x": sp.Symbol("x", real=True, nonzero=True, positive=True),
})
SYMBOLS = ["I", "INT+", "INT-", "INT", "FLOAT", "-", ".", "10^", "Y"]
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
SPECIAL_WORDS = ["SOE", "EOE", "PAD"]
INT_BASE = 10
COEFFICIENTS = OrderedDict({
    f'a{i}': sp.Symbol(f'a{i}', real=True)
    for i in range(10)
})


def prefix_to_sympy(expr, evaluate=True):
    p, r = prefix_to_infix(expr)
    if len(r) > 0:
        raise Exception(f"Incorrect prefix expression \"{expr}\". \"{r}\" was not parsed.")

    local_dict = get_sympy_local_dict()
    expr = sp.parsing.sympy_parser.parse_expr(f'({p})', evaluate=evaluate, local_dict=local_dict)
    return expr


def get_sympy_local_dict() -> dict:
    local_dict = {}
    for k, v in list(VARIABLES.items()) + list(COEFFICIENTS.items()):
        assert k not in local_dict
        local_dict[k] = v
    return local_dict


def prefix_to_infix(expr):
    return _prefix_to_infix(expr.split(" "))


def _prefix_to_infix(expr):
    """
    Parse an expression in prefix mode, and output it in either:
        - infix mode (returns human readable string)
        - develop mode (returns a dictionary with the simplified expression)
    """
    if len(expr) == 0:
        raise Exception("Empty prefix list.")
    t = expr[0]

    if t in OPERATORS: #OPERATOR dict, t is an operator
        args = []
        l1 = expr[1:]
        for _ in range(OPERATORS[t]):
            i1, l1 = _prefix_to_infix(l1)
            args.append(i1)
        return write_infix(t, args), l1
    elif t in VARIABLES or t in COEFFICIENTS or t in CONSTANTS or t == 'I': #if t is variable 'x' or coefficient 'a1', 'a2'... , or constant "pi", "E", or 'I'
        return t, expr[1:]
    else: #else when t is INT+ INT-
        val, i = parse_int(expr)
        return str(val), expr[i:]


def parse_int(lst):
    """
    Parse a list that starts with an integer.
    Return the integer value, and the position it ends in the list.
    """
    base = 10
    balanced = False
    val = 0
    if not (balanced and lst[0] == 'INT' or base >= 2 and lst[0] in ['INT+', 'INT-'] or base <= -2 and lst[0] == 'INT'): #if first token is INT+ or INT-
        raise Exception(f"Invalid integer in prefix expression")
    i = 0
    for x in lst[1:]:
        if not (x.isdigit() or x[0] == '-' and x[1:].isdigit()):#if the rest part of the list is not a number, break
            break
        val = val * base + int(x)#otherwise, convert the str into int
        i += 1
    if base > 0 and lst[0] == 'INT-':
        val = -val
    return val, i + 1#i+1 is the position number ends in the list


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
    elif token in ['sign', 'sqrt', 'exp', 'ln', 'sin', 'cos', 'tan', 'cot', 'sec', 'csc', 'asin', 'acos', 'atan', 'acot', 'asec', 'acsc', 'sinh', 'cosh', 'tanh', 'coth', 'sech', 'csch', 'asinh', 'acosh', 'atanh', 'acoth', 'asech', 'acsch']:
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
# ================================================================================


def create_pairs(equiv_exprs: list) -> list:
    expr_pairs = []

    for expr_pair in itertools.permutations(iterable=equiv_exprs, r=2):
        expr_pairs.append(f"{expr_pair[0]}\t{expr_pair[1]}")

    return expr_pairs


def verify(expr_pairs: list, n: int, tol: float, secs: int) -> tuple[list, list]:
    @timeout(secs=secs)
    def _simplify(expr: Expr) -> Expr:
        return sp.simplify(expr=expr)

    @timeout(secs=secs)
    def _cont_domain(expr: Expr, symbol: Symbol):
        return continuous_domain(f=expr, symbol=symbol,
                                 domain=Interval(start=0, end=10, left_open=True, right_open=False))

    @timeout(secs=secs*2)
    def _check_equiv(x: Symbol, expr: Expr, start: float, end: float, n: int, tol: float) -> bool:
        rand_nums = numpy.random.uniform(low=start, high=end, size=n)
        for num in rand_nums:
            val = expr.subs(x, num).evalf()
            if val > tol:
                return False

        return True

    @timeout(secs=secs*2)
    def _check_equiv_compl(x: Symbol, expr: Expr, start: float, end: float, n: int, tol: float) -> bool:
        i = 0
        while i < n:
            rand_num = numpy.random.uniform(low=start, high=end, size=1)
            val = expr.subs(x, rand_num).evalf()
            if val in S.Reals:
                if val > tol:
                    return False
                i += 1

        return True

    corrects = []
    incorrects = []

    x = VARIABLES['x']

    for expr_pair in expr_pairs:
        pair = expr_pair.rstrip().split('\t')

        if "coth" in pair[0] or "coth" in pair[1]:
            continue

        try:
            expr_0 = prefix_to_sympy(expr=pair[0])
            expr_1 = prefix_to_sympy(expr=pair[1])
        except Exception as e:
            logger.log_error(f"prefix_to_sympy exception {e}; {pair[0]} & {pair[1]}")
            incorrects.append(expr_pair)
            continue
        try:
            expr_0 = _simplify(expr=expr_0)
            expr_1 = _simplify(expr=expr_1)
        except Exception as e:
            logger.log_error(f"simplify exception {e}; {pair[0]} & {pair[1]}")
            incorrects.append(expr_pair)
            continue

        expr = expr_0 - expr_1

        if expr == 0:
            logger.log_info(f" simplify  , equivalent    ; {pair[0]} & {pair[1]}")
            equiv = True
        else:
            try:
                domain = _cont_domain(expr=expr, symbol=x)
                try:
                    if isinstance(domain, sp.sets.sets.Union):
                        if isinstance(domain.args[0], sp.sets.sets.Complement):
                            case = "Union-Comp"
                            equiv = _check_equiv_compl(x=x, expr=expr, start=1, end=10, n=n, tol=tol)
                        else:
                            case = "Union"
                            start = float(domain.args[0].start)
                            end = float(domain.args[0].end)
                            equiv = _check_equiv(x=x, expr=expr, start=start, end=end, n=n, tol=tol)
                        if equiv:
                            logger.log_info(f" {case:<10}, equivalent    ; {pair[0]} & {pair[1]}")
                        else:
                            logger.log_error(f"{case:<10}, non-equivalent; {pair[0]} & {pair[1]}")
                    elif isinstance(domain, sp.sets.sets.Complement):
                        case = "Complement"
                        equiv = _check_equiv_compl(x=x, expr=expr, start=1, end=10, n=n, tol=tol)
                        if equiv:
                            logger.log_info(f" {case:<10}, equivalent    ; {pair[0]} & {pair[1]}")
                        else:
                            logger.log_error(f"{case:<10}, non-equivalent; {pair[0]} & {pair[1]}")
                    elif isinstance(domain, sp.sets.sets.Interval):
                        case = "Interval"
                        start = float(domain.start)
                        end = float(domain.end)
                        equiv = _check_equiv(x=x, expr=expr, start=start, end=end, n=n, tol=tol)
                        if equiv:
                            logger.log_info(f" {case:<10}, equivalent    ; {pair[0]} & {pair[1]}")
                        else:
                            logger.log_error(f"{case:<10}, non-equivalent; {pair[0]} & {pair[1]}")
                    else:
                        logger.log_error(f"Invalid domain type {domain}")
                        equiv = False

                except Exception as e:
                    logger.log_error(f"{case:<10}, eval exception {e}; {pair[0]} & {pair[1]} ")
                    equiv = False
            except Exception as e:
                logger.log_error(f"continuous domain exception {e}; {pair[0]} & {pair[1]}")
                equiv = False

        if equiv:
            corrects.append(expr_pair)
        else:
            incorrects.append(expr_pair)

    return corrects, incorrects


def w_data(equiv_exprs: list[str], data_dir: str, cls: str, category: str) -> None:
    path = os.path.join(data_dir, cls, category)

    if not os.path.exists(path=path):
        os.makedirs(name=path, exist_ok=True)

    filepath = os.path.join(path, "equiv_exprs.txt")
    file = open(file=filepath, mode='a')

    for expr in equiv_exprs:
        file.write(expr+'\n')

    file.write('\n')

    file.close()

    return


def create_dataset(data_dir: str, n: int, tol: float, secs: int, verified_dir: str, incorrect_dir: str) -> None:
    filepath = os.path.join(data_dir, "**", "equiv_exprs.txt")
    filepaths = glob.glob(pathname=filepath, recursive=True)

    n_corrects = 0
    n_incorrects = 0

    progbar = tqdm.tqdm(iterable=filepaths)

    for filepath in progbar:
        parts = filepath.split(os.path.sep)
        cls = parts[-3]
        category = parts[-2]
        progbar.set_description(desc=f"[INFO]: Processing class '{cls}', category '{category}'", refresh=True)

        file = open(file=filepath, mode='r')

        equiv_exprs = []

        for line in file:
            if line.strip():
                equiv_exprs.append(line.strip())
            else:
                expr_pairs = create_pairs(equiv_exprs=equiv_exprs)
                corrects, incorrects = verify(expr_pairs=expr_pairs, n=n, tol=tol, secs=secs)
                w_data(equiv_exprs=expr_pairs, data_dir=verified_dir, cls=cls, category=category)
                w_data(equiv_exprs=incorrects, data_dir=incorrect_dir, cls=cls, category=category)

                n_corrects += len(corrects)
                n_incorrects += len(incorrects)

                equiv_exprs = []

        file.close()

    logger.log_info(f"Total number of correct expression pairs:   {n_corrects}")
    logger.log_info(f"Total number of incorrect expression pairs: {n_incorrects}")
    logger.log_info(f"Accuracy: {n_corrects/(n_corrects+n_incorrects)*100:.4f}%")

    return


def main() -> None:
    if os.path.exists(path=config.DATA_VERIFIED_DIR):
        logger.log_error(f"'{config.DATA_VERIFIED_DIR}' directory already exists!")
        logger.log_error(f"Make sure to remove '{config.DATA_VERIFIED_DIR}' directory first.")
        logger.log_error("Operation aborted.")
        exit(1)
    if not os.path.exists(path=config.DATA_FILTERED_DIR):
        logger.log_error(f"'{config.DATA_FILTERED_DIR}' directory does not exist!")
        logger.log_error(f"Make sure to run './create_dataset.py -f -n <n_exprs>' first to create "
                         f"'{config.DATA_FILTERED_DIR}' directory.")
        logger.log_error("Operation aborted.")
        exit(1)

    create_dataset(data_dir=config.DATA_FILTERED_DIR, verified_dir=config.DATA_VERIFIED_DIR,
                   incorrect_dir=config.DATA_INCORRECT_DIR, n=3, tol=1e-6, secs=4)

    return


if __name__ == '__main__':
    main()