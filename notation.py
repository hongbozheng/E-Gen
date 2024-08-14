import config as cfg
import sympy as sp
from collections import OrderedDict
from fractions import Fraction


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
    # Derivative
    # sp.Derivative: "d x",
}

COEFFICIENTS = OrderedDict({
    f'a{i}': sp.Symbol(f'a{i}', real=True)
    for i in range(10)
})


# ================================= eeg -> tx ==================================
def ref_int(s: str) -> str:
    if s[0] == '-':
        return "INT- " + ' '.join(s[1:])
    else:
        return "INT+ " + ' '.join(s)


def is_int(s: str) -> bool:
    if s[0] == '-' and s[1:].isdigit():
        return True
    elif s.isdigit():
        return True
    else:
        return False


def ref_expr(expr: str) -> str:
    expr = expr.replace("+", "add").replace("*", "mul").replace("/", "div")

    tokens = expr.split(sep=' ')
    for i, token in enumerate(tokens):
        if not token:
            continue
        elif token == '-':
            tokens[i] = "sub"
        elif '.' in token:
            fraction = Fraction(token).limit_denominator(
                max_denominator=cfg.N_DENOMINATOR_DIGITS
            )
            numerator = ref_int(s=str(fraction.numerator))
            denominator = ref_int(s=str(fraction.denominator))
            tokens[i] = f"div {numerator} {denominator}"
        elif is_int(s=token):
            token = ref_int(s=token)
            tokens[i] = token

    return ' '.join(tokens)
# ==============================================================================


# ============================== prefix -> sympy ===============================
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
    if t in cfg.MATH_OPERATORS:
        args = []
        l1 = expr[1:]
        for _ in range(cfg.MATH_OPERATORS[t]):
            i1, l1 = _prefix_to_infix(l1)
            args.append(i1)
        return write_infix(t, args), l1
    # if t is variable 'x' or coefficient 'a1', 'a2'... ,
    # or constant "pi", "E", or 'I'
    elif (t in VARIABLES or t in COEFFICIENTS or t in cfg.CONSTANTS
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
# ==============================================================================


# ============================== sympy -> prefix ===============================
def _sympy_to_prefix(op, expr):
    """
    Parse a SymPy expression given an initial root operator.
    """
    n_args = len(expr.args)

    # derivative operator
    if op == 'derivative':
        assert n_args >= 2
        assert all(len(arg) == 2 and str(arg[0]) in VARIABLES and \
                   int(arg[1]) >= 1 for arg in expr.args[1:]), expr.args
        parse_list = sympy_to_prefix_helper(expr.args[0])
        for var, degree in expr.args[1:]:
            parse_list = ['derivative' for _ in range(int(degree))] \
                + parse_list + [str(var) for _ in range(int(degree))]
        return parse_list

    assert (op == 'add' or op == 'mul') and (n_args >= 2) \
        or (op == 'not') and (n_args == 1) \
        or (op == 'and' or op == 'or' or op == 'xor') and (n_args >= 2) \
        or (op != 'add' and op != 'mul' and op != 'and' or op != 'or' or op != 'xor') and (1 <= n_args <= 2)

    # square root
    if op == 'pow' and isinstance(expr.args[1], sp.Rational) \
        and expr.args[1].p == 1 and expr.args[1].q == 2:
        return ['sqrt'] + sympy_to_prefix_helper(expr.args[0])

    # parse children
    parse_list = []
    for i in range(n_args):
        if i == 0 or i < n_args - 1:
            parse_list.append(op)
        parse_list += sympy_to_prefix_helper(expr.args[i])

    return parse_list


def write_int(val):
    """
    Convert a decimal integer to a representation in the given base.
    The base can be negative.
    In balanced bases (positive), digits range from -(base-1)//2 to (base-1)//2
    """
    base = 10
    balanced = False
    res = []
    max_digit = abs(base)
    if balanced:
        max_digit = (base - 1) // 2
    else:
        if base > 0:
            neg = val < 0
            val = -val if neg else val
    while True:
        rem = val % base
        val = val // base
        if rem < 0 or rem > max_digit:
            rem -= base
            val += 1
        res.append(str(rem))
        if val == 0:
            break
    if base < 0 or balanced:
        res.append('INT')
    else:
        res.append('INT-' if neg else 'INT+')
    return res[::-1]

def div(arg0, arg1):
    parse_list = ['div']
    parse_list += sympy_to_prefix_helper(arg0)
    parse_list += sympy_to_prefix_helper(arg1.args[0])
    return parse_list

def mul(expr):
    
    n_args = len(expr.args)
    if n_args == 2 and isinstance(expr.args[1], sp.Pow) \
        and expr.args[1].args[1] == -1:
        return div(expr.args[0], expr.args[1])
    i = 0
    parse_list = ['mul']
    while i < n_args:
        if i+1 < n_args and isinstance(expr.args[i+1], sp.Pow) \
            and expr.args[i+1].args[1] == -1:
            if i > 0 and i < n_args - 2:
                parse_list.append('mul')
            parse_list += div(expr.args[i], expr.args[i+1])
            i+=2
        else:
            if i > 0 and i < n_args - 1:
                parse_list.append('mul')
            parse_list += sympy_to_prefix_helper(expr.args[i])
            i+=1
    return parse_list

def sub(arg0, arg1):
    # arg0 = expr.args[0]
    # arg1 = expr.args[1]
    if arg1.args[0] == -1:
        parse_list = ['sub']
        parse_list += sympy_to_prefix_helper(arg0)
        parse_list += sympy_to_prefix_helper(arg1.args[1])
    # elif isinstance(arg1.args[0], sp.Integer) and arg1.args[0] != -1:


    return parse_list

def add(expr):
    n_args = len(expr.args)
    if n_args == 2 and isinstance(expr.args[1], sp.Mul) \
        and expr.args[1].args[0] == -1:
        return sub(expr.args[0], expr.args[1])
    i = 0
    parse_list = ['add']
    while i < n_args:
        if i+1 < n_args and isinstance(expr.args[i+1], sp.Mul) \
            and expr.args[i+1].args[0] == -1:
            if i > 0 and i < n_args - 2:
                parse_list.append('add')
            parse_list += sub(expr.args[i], expr.args[i+1])
            i+=2
        else:
            if i > 0 and i < n_args - 1:
                parse_list.append('add')
            parse_list += sympy_to_prefix_helper(expr.args[i])
            i+=1
    return parse_list

def sympy_to_prefix_helper(expr):
    """
    Convert a SymPy expression to a prefix one.
    """
    if isinstance(expr, sp.Symbol):
        return [str(expr)]
    elif isinstance(expr, sp.Integer):
        return write_int(int(str(expr)))
    elif isinstance(expr, sp.Rational):
        return ['div'] + write_int(int(expr.p)) + write_int(int(expr.q))
    elif expr == sp.E:
        return ['E']
    elif expr == sp.pi:
        return ['pi']
    elif expr == sp.I:
        return ['I']
    elif expr == sp.false:
        return ['false']
    elif expr == sp.true:
        return ['true']
    elif isinstance(expr, sp.Mul):
        return mul(expr)
    elif isinstance(expr, sp.Add):
        return add(expr)
    # SymPy operator
    for op_type, op_name in SYMPY_OPERATORS.items():
        if isinstance(expr, op_type):
            return _sympy_to_prefix(op_name, expr)
    # unknown operator
    raise Exception(f"Unknown SymPy operator: {expr}")


def sympy_to_prefix(expr):
    return " ".join(sympy_to_prefix_helper(expr))
# ==============================================================================
