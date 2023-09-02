with open('input\input_exp.txt', 'r', encoding='utf-8') as f:
    data = f.readlines()
    #print(data)
output=[] #The output list which contains all latex expressions.


num_input = {"+": 2, "-": 2, "*": 2, "/": 2, "pow": 2, "sqrt": 1}


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
            print(latex_token)
            num_token_del = num_input[exp[i]]
            del exp[i:i+num_token_del+1] #delete original token from the expression
            print("before delete:", exp)
            exp.insert(i, latex_token) #replace the original token with the new latex form expression
            i-=1
            print("after:", exp)
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
