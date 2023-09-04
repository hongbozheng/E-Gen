with open('input\input_exp.txt', 'r', encoding='utf-8') as f:
    data = f.readlines()
    #print(data)
output=[] #The output list which contains all latex expressions.


num_input = {"+": 2, "-": 2, "*": 2, "/": 2, "pow": 2, "sqrt": 1, "d": 2, "sin": 1, "cos": 1, "tan": 1, "asin": 1, "acos": 1, "atan":1 , "acot": 1, "asec": 1, "acsc": 1, "sinh": 1, "cosh": 1, "tanh": 1, "csch": 1, "sech": 1, "coth": 1, "ln": 1, "log": 2}


def is_operator(token):
    if token in num_input:
        return True
    else:
        return False

def token_to_latex(i, exp, token):
    if token == "+":
        latex_token = str(exp[i+1]) + " + " + str(exp[i+2])
    elif token == "-":
        latex_token = str(exp[i+1]) + " - " + str(exp[i+2])
    elif token == "*":
        #print("exp is ", exp)
        #print("exp[i+1] is ", exp[i+1])
        #print("exp[i+2] is ", exp[i+2])
        latex_token = "(%s)" %(str(exp[i+1])) + "\cdot " + "(%s)" %(str(exp[i+2]))
    elif token == "/":
        latex_token = "\\frac{" + str(exp[i+1]) + "}{" + str(exp[i+2]) + "} " #latex form of fraction is "\frac{}{} "
    elif token == "pow":
        latex_token = "(%s)" %(str(exp[i+1])) + "^{" + str(exp[i+2]) + "} " #latex form of power is "^{} "
    elif token == "sqrt":
        latex_token = "\sqrt{%s} " %(str(exp[i+1]))
    elif token == "d":
        latex_token = "\\frac{\mathrm{d}" + str(exp[i+2]) + "}{\mathrm{d}" + str(exp[i+1]) + "} "
    elif token == "sin":
        latex_token = "\sin(%s) " %(str(exp[i+1]))
    elif token == "cos":
        latex_token = "\cos(%s) " %(str(exp[i+1]))
    elif token == "tan":
        latex_token = "\tan(%s) " %(str(exp[i+1]))
    elif token == "cot":
        latex_token = "\cot(%s) " %(str(exp[i+1]))
    elif token == "sec":
        latex_token = "\sec(%s) " %(str(exp[i+1]))
    elif token == "csc":
        latex_token = "\csc(%s) " %(str(exp[i+1]))
    elif token == "asin":
        latex_token = "\\arcsin(%s) " %(str(exp[i+1]))
    elif token == "acos":
        latex_token = "\\arccos(%s) " %(str(exp[i+1]))
    elif token == "atan":
        latex_token = "\\arctan(%s) " %(str(exp[i+1]))
    elif token == "acot":
        latex_token = "\operatorname{arccot} (%s) " %(str(exp[i+1]))
    elif token == "asec":
        latex_token = "\operatorname{arcsec} (%s) " %(str(exp[i+1]))
    elif token == "acsc":
        latex_token = "\operatorname{arccsc} (%s) " %(str(exp[i+1]))
    elif token == "sinh":
        latex_token = "\sinh(%s) " %(str(exp[i+1]))
    elif token == "cosh":
        latex_token = "\cosh(%s) " %(str(exp[i+1]))
    elif token == "tanh":
        latex_token = "\tanh(%s) " %(str(exp[i+1]))
    elif token == "coth":
        latex_token = "\coth(%s) " %(str(exp[i+1]))
    elif token == "csch":
        latex_token = "\operatorname{csch} (%s) " %(str(exp[i+1]))
    elif token == "sech":
        latex_token = "\operatorname{sech} (%s) " %(str(exp[i+1]))
    elif token == "ln":
        latex_token = "\ln_{}{(%s)} " %(str(exp[i+1]))
    elif token == "log":
        latex_token = "\log_{%s}" %(str(exp[i+1])) + "{(%s)}" %(str(exp[i+2]))
    return latex_token

def main(exp):
    i=len(exp)-1
    while i>=0:
        #print("current index is", i)
        if not is_operator(exp[i]):
            i-=1
            continue
        else:
            #print("current token is", exp[i])
            latex_token = token_to_latex(i, exp, exp[i])
            #print(latex_token)
            num_token_del = num_input[exp[i]]
            del exp[i:i+num_token_del+1] #delete original token from the expression
            #print("before delete:", exp)
            exp.insert(i, latex_token) #replace the original token with the new latex form expression
            i-=1
            #print("after:", exp)
    return exp


for i in range(0,len(data)): #enumerate all input expressions in the txt file, convert each expression by steps
    input_exp = data[i].replace("\n", "").split(" ")
    #print(input_exp)
    output_exp = main(input_exp)
    output.extend(output_exp)
    #print(output)

with open('output\output_exp.txt', 'w') as out_file:
    for expression in output:
        out_file.write(expression + "\n")
