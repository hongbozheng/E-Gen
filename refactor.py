import config
from fractions import Fraction


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
                max_denominator=config.N_DENOMINATOR_DIGITS
            )
            numerator = ref_int(s=str(fraction.numerator))
            denominator = ref_int(s=str(fraction.denominator))
            tokens[i] = f"div {numerator} {denominator}"
        elif is_int(s=token):
            token = ref_int(s=token)
            tokens[i] = token

    return ' '.join(tokens)