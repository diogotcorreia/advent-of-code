import re
from lark import Lark

lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]

messages = []
rules = []

for line in lines:
    if ':' not in line:
        messages.append(line)
    else:
        rules.append(line)

##########################################
#                 PART 1                 #
##########################################


def build_grammar(rules):
    return Lark('\n'.join(map(lambda x: re.sub(r"(\d+)", r"rule\1", x), rules)), start="rule0")


def matches_grammar(grammar, line):
    try:
        grammar.parse(line)
        return True
    except:
        return False


def part1(rules, messages):
    grammar = build_grammar(rules)
    return len(list(filter(lambda x: matches_grammar(grammar, x), messages)))


print('Answer to part 1 is', part1(rules, messages))

##########################################
#                 PART 2                 #
##########################################


def part2(rules, messages):
    rules = rules + []
    for i in range(len(rules)):
        if rules[i] == "8: 42":
            rules[i] = "8: 42 | 42 8"
        if rules[i] == "11: 42 31":
            rules[i] = "11: 42 31 | 42 11 31"
    grammar = build_grammar(rules)
    return len(list(filter(lambda x: matches_grammar(grammar, x), messages)))


print('Answer to part 2 is', part2(rules, messages))
