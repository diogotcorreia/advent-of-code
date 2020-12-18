from functools import reduce

lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]

##########################################
#                 PART 1                 #
##########################################


def calculate(expression):
    def calculate_aux(op, number, acc):
        if op == '*':
            acc *= int(number)
        elif op == '+':
            acc += int(number)
        return acc

    acc = 0
    i = 0
    currentNumber = ""
    lastOp = "+"
    while i < len(expression):
        c = expression[i]
        i += 1
        if c == ' ':
            continue
        if c in ('*', '+'):
            acc = calculate_aux(lastOp, currentNumber, acc)
            currentNumber = ""
            lastOp = c
        elif c == '(':
            res, newI = calculate(expression[i:])
            i += newI
            currentNumber = res
        elif c == ')':
            break
        else:
            currentNumber += c
    acc = calculate_aux(lastOp, currentNumber, acc)
    return acc, i


def part1(expressions):
    return reduce(lambda a, b: a + b, [calculate(e)[0] for e in expressions])


print('Answer to part 1 is', part1(lines))

##########################################
#                 PART 2                 #
##########################################


def calculate2(expression):
    def get_expression(e):
        depth = 0
        i = 0
        while i < len(e):
            if e[i] == '(':
                depth += 1
            if e[i] == ')':
                depth -= 1
                if depth < 0:
                    return e[:i]
            i += 1
    if '(' in expression:
        i = 0
        while i < len(expression):
            c = expression[i]
            i += 1
            if c == "(":
                exp = get_expression(expression[i:])
                expression = expression[:i - 1] + str(calculate2(exp)) + \
                    expression[i + len(exp) + 1:]

    mul = expression.split('*')
    return reduce(lambda a, b: int(a) * int(b), [reduce(lambda a, b: int(a) + int(b), s.split('+')) for s in mul])


def part2(expressions):
    return reduce(lambda a, b: a + b, [calculate2(e) for e in expressions])


print('Answer to part 2 is', part2(lines))
