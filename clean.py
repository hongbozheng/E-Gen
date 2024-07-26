import config
import logger
from verify import VARIABLES, COEFFICIENTS


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


def ref_int(val:int):
    val = str(val)
    return ' '.join(val)


def calculate_INT(args:list, sign:str):
    # print(args)
    if args[0].startswith('INT+'):
        val0 = int(args[0].split(' ')[1])
    elif args[0].startswith('INT-'):
        val0 = -int(args[0].split(' ')[1])

    if args[1].startswith('INT'):
        if args[1].startswith('INT+'):
            val1 = int(args[1].split(' ')[1])
        elif args[1].startswith('INT-'):
            val1 = -int(args[1].split(' ')[1])
    else:
        if args[1].split()[1] ==  'INT+':
            val1 = int(args[1].split(' ')[2])
        elif args[1].split()[1] ==  'INT-':
            val1 = -int(args[1].split(' ')[2])

    if sign == 'add':
        return val0 + val1
    elif sign == 'sub':
        return val0 - val1
    elif sign == 'mul':
        return val0 * val1
    elif sign == 'div':
        if val0%val1==0 and val0/val1>0:
            return 'INT+ ' + str(int(val0/val1))
        elif val0%val1==0 and val0/val1<0:
            return 'INT- ' + str(int(-val0/val1))
        elif val0/val1 == 0:
            return 'INT+ 0'
        elif val0<0 and val1<0:
            return 'div INT+ ' + str(-val0) + ' INT+ ' + str(-val1)
        else:
            if args[1].startswith('INT'):
                return f'div {args[0]} {args[1]}'
            else:
                return f'div {args[0]} ' + ' '.join(args[1].split()[1:3])


def write_infix(token, args):
    """
    Infix representation.
    Convert prefix expressions to a format that SymPy can parse.
    """
    if token == 'add':
        if not args[0].startswith('INT') and args[1].startswith('INT'):
            tem = args[0]
            args[0] = args[1]
            args[1] = tem
        #int + int
        if args[0].startswith('INT') and args[1].startswith('INT'):
            val = calculate_INT(args=args, sign='add')
            if val >= 0:
                return f'INT+ {str(val)}'
            elif val < 0:
                return f'INT- {str(-val)}'
        #0 + xxx
        elif args[0] == 'INT+ 0' and not args[1].startswith('INT'):
            return args[1]
        elif args[1] == 'INT+ 0' and not args[0].startswith('INT'):
            return args[0]
        #int + (int +- xxx)
        elif args[0].startswith('INT') and args[1].startswith('add INT'):
            lst0 = args[0].split(' ')
            lst1 = args[1].split(' ')
            val = calculate_INT(args=args, sign='add')
            if val > 0:
                lst1[1] = 'INT+'
                lst1[2] = str(val)
                return ' '.join(lst1)
            if val == 0:
                return ' '.join(lst1[3:])
            elif val < 0:
                lst1[1] = 'INT-'
                lst1[2] = str(-val)
                return ' '.join(lst1)
        elif args[0].startswith('INT') and args[1].startswith('sub INT'):
            lst0 = args[0].split(' ')
            lst1 = args[1].split(' ')
            val = calculate_INT(args=args, sign='add')
            if val > 0:
                lst1[1] = 'INT+'
                lst1[2] = str(val)
                return ' '.join(lst1)
            if val == 0:
                xxx = ' '.join(lst1[3:])
                if xxx.startswith('mul INT') or xxx.startswith('div INT'): #0-(mul INT xxx)
                    lst = xxx.split(' ')
                    if lst[1] == 'INT+':
                        lst[1] = 'INT-'
                        return ' '.join(lst)
                    elif lst[1] == 'INT-':
                        lst[1] = 'INT+'
                        return ' '.join(lst)
                else:
                    return 'mul INT- 1' + ' ' + ' '.join(lst1[3:])
            elif val < 0:
                lst1[1] = 'INT-'
                lst1[2] = str(-val)
                return ' '.join(lst1)
        else:
            return f'add {args[0]} {args[1]}'
    elif token == 'sub' or token == 'subtract':
        #int - int
        if args[0].startswith('INT') and args[1].startswith('INT'):
            val = calculate_INT(args=args, sign='sub')
            if val >= 0:
                return f'INT+ {str(val)}'
            elif val < 0:
                return f'INT- {str(-val)}'
        #0 - xxx or xxx - 0
        elif args[0] == 'INT+ 0' and not args[1].startswith('INT') and not args[1].startswith('add') and not args[1].startswith('sub'):
            if args[1].startswith('mul INT') or args[1].startswith('div INT'):
                lst1 = args[1].split(' ')
                if lst1[1] == 'INT+':
                    lst1[1] = 'INT-'
                    return ' '.join(lst1)
                elif lst1[1] == 'INT-':
                    lst1[1] = 'INT+'
                    return ' '.join(lst1)
            else:
                return f'mul INT- 1 {args[1]}'
        elif args[1] == 'INT+ 0' and not args[0].startswith('INT'):
            return args[0]
        #int - (int +- xxx)
        elif args[0].startswith('INT') and args[1].startswith('add INT'):
            lst0 = args[0].split(' ')
            lst1 = args[1].split(' ')
            val = calculate_INT(args=args, sign='sub')
            if val > 0:
                lst1[0] = 'sub'
                lst1[1] = 'INT+'
                lst1[2] = str(val)
                return ' '.join(lst1)
            if val == 0:
                xxx = ' '.join(lst1[3:])
                if xxx.startswith('mul INT') or xxx.startswith('div INT'): #0-(mul INT xxx)
                    lst = xxx.split(' ')
                    if lst[1] == 'INT+':
                        lst[1] = 'INT-'
                        return ' '.join(lst)
                    elif lst[1] == 'INT-':
                        lst[1] = 'INT+'
                        return ' '.join(lst)
                else:
                    return 'mul INT- 1' + ' ' + ' '.join(lst1[3:])
            elif val < 0:
                lst1[0] = 'sub'
                lst1[1] = 'INT-'
                lst1[2] = str(-val)
                return ' '.join(lst1)
        elif args[0].startswith('INT') and args[1].startswith('sub INT'):
            lst0 = args[0].split(' ')
            lst1 = args[1].split(' ')
            val = calculate_INT(args=args, sign='sub')
            if val > 0:
                lst1[0] = 'add'
                lst1[1] = 'INT+'
                lst1[2] = str(val)
                return ' '.join(lst1)
            if val == 0:
                return ' '.join(lst1[3:])
            elif val < 0:
                lst1[0] = 'add'
                lst1[1] = 'INT-'
                lst1[2] = str(-val)
                return ' '.join(lst1)
        else:
            return f'sub {args[0]} {args[1]}'
    elif token == 'mul' or token == 'multiply':
        #if any argument is 0
        if args[0] == 'INT+ 0' or args[1] == 'INT+ 0':
            return 'INT+ 0'
        #make sure the first argument of mul is number
        if not args[0].startswith('INT') and args[1].startswith('INT'):
            tem = args[0]
            args[0] = args[1]
            args[1] = tem
        #int * int
        if args[0].startswith('INT') and args[1].startswith('INT'):
            val = calculate_INT(args=args, sign='mul')
            if val >= 0:
                return f'INT+ {str(val)}'
            elif val < 0:
                return f'INT- {str(-val)}'
        #1 * xxx
        elif args[0] == 'INT+ 1' and not args[1].startswith('INT'):
            return args[1]
        #0 * xxx
        # elif args[0] == 'INT+ 0' and not args[1].startswith('INT'):
        #     return 'INT+ 0'
        #int * (int * xxx)
        elif args[0].startswith('INT') and args[1].startswith('mul INT'):
            lst0 = args[0].split(' ')
            lst1 = args[1].split(' ')
            val = calculate_INT(args=args, sign='mul')
            if val == 1:
                return ' '.join(lst1[3:])
            elif val == 0:
                return 'INT+ 0'
            elif val > 0 and val != 1:
                lst1[1] = 'INT+'
                lst1[2] = str(val)
                return ' '.join(lst1)
            elif val < 0:
                lst1[1] = 'INT-'
                lst1[2] = str(-val)
                return ' '.join(lst1)
        #int * (int / xxx)
        elif args[0].startswith('INT') and args[1].startswith('div INT'):
            lst0 = args[0].split(' ')
            lst1 = args[1].split(' ')
            val = calculate_INT(args=args, sign='mul')
            if val == 1:
                return 'div INT+ 1 ' + ' '.join(lst1[3:])
            elif val == 0:
                return 'INT+ 0'
            elif val > 0 and val != 1:
                lst1[1] = 'INT+'
                lst1[2] = str(val)
                return ' '.join(lst1)
            elif val < 0:
                lst1[1] = 'INT-'
                lst1[2] = str(-val)
                return ' '.join(lst1)
        else:
            return f'mul {args[0]} {args[1]}'
    elif token == 'div':
        #0 / xxx
        if args[0] == 'INT+ 0':
            return 'INT+ 0'
        #xxx / 1
        if args[1] == 'INT+ 1':
            return args[0]
        #INT / INT
        if args[1] == 'INT+ 0':
            return f'div {args[0]} {args[1]}'
        if args[0].startswith('INT') and args[1].startswith('INT'):
            val = calculate_INT(args=args, sign='div')
            # print(val)
            return val
        #INT / (INT / xxx)
        elif args[0].startswith('INT') and args[1].startswith('div INT'):
            lst0 = args[0].split(' ')
            lst1 = args[1].split(' ')
            val = calculate_INT(args=args, sign='div')
            if val == 'INT+ 1':
                return ' '.join(lst1[3:])
            elif val == 'INT+ 0':
                return 'INT+ 0'
            else:
                return f'mul {str(val)} ' + ' '.join(lst1[3:])
        #INT / (INT * xxx)
        elif args[0].startswith('INT') and args[1].startswith('mul INT'):
            lst0 = args[0].split(' ')
            lst1 = args[1].split(' ')
            val = calculate_INT(args=args, sign='div')
            if val == 'INT+ 1':
                return 'div INT+ 1 ' + ' '.join(lst1[3:])
            elif val == 'INT+ 0':
                return 'INT+ 0'
            else:
                return f'div {str(val)} ' + ' '.join(lst1[3:])
        else:
            return f'div {args[0]} {args[1]}'
    elif token == 'pow':
        if args[1] == 'INT+ 1':
            return args[0]
        elif args[1] == 'INT+ 0':
            return 'INT+ 1'
        else:
            return f'pow {args[0]} {args[1]}'
    elif token == 'abs':
        return f'abs {args[0]}'
    elif token in ['sqrt', 'ln',
                   'sin', 'cos', 'tan',
                   'csc', 'sec', 'cot',
                   'sinh', 'cosh', 'tanh',
                   'csch', 'sech', 'coth',
                   'asin', 'acos', 'atan',
                   'acsc', 'asec', 'acot',
                   'asinh', 'acosh', 'atanh',
                   'acoth', 'asech', 'acsch']:
        return f'{token} {args[0]}'
    elif token == 'd':
        return f'd {args[0]} {args[1]}'
    elif token.startswith('INT'):
        # print(token)
        return f'{token} {args[0]}'
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
        if val >= 0:
            return t + ' ' + str(val), expr[i:]
        else:
            return t + ' ' + str(-val), expr[i:]


def prefix_to_infix(expr):
    return _prefix_to_infix(expr.split(" "))


def get_sympy_local_dict() -> dict:
    local_dict = {}
    for k, v in list(VARIABLES.items()) + list(COEFFICIENTS.items()):
        assert k not in local_dict
        local_dict[k] = v
    return local_dict


def clean(expr):
    p, r = prefix_to_infix(expr)
    if len(r) > 0:
        raise Exception(
            f"Incorrect prefix expression \"{expr}\". \"{r}\" was not parsed."
        )
    local_dict = get_sympy_local_dict()
    return p


def clean_block(equiv_exprs: list):
    equiv_exprs_after_cleaning = []
    for expr in equiv_exprs:
        # print(f'raw: {expr}')
        try:
            expr = clean(expr)
            # print(f'clean: {expr}')
            if expr not in equiv_exprs_after_cleaning:
                equiv_exprs_after_cleaning.append(expr)
        except Exception as e:
            logger.log_error(
                f"can not clean expression {e};"
            ) 
    return equiv_exprs_after_cleaning


def int_add_space(equiv_exprs_after_cleaning_pre: list):
    equiv_exprs_after_cleaning = []
    for expr in equiv_exprs_after_cleaning_pre:
        expr = expr.split(' ')
        for i in range(len(expr)):
            if expr[i].isdigit():
                expr[i] = ' '.join(expr[i])
        equiv_exprs_after_cleaning.append(' '.join(expr))
    return equiv_exprs_after_cleaning


def write_block(equiv_exprs_after_cleaning: list):
    write_file = open(config.EQUIV_EXPRS_CLEANING, mode='a')
    for expr in equiv_exprs_after_cleaning:
        # print(expr)
        write_file.write(f"{expr}\n")
    write_file.write("\n")
    write_file.close()
