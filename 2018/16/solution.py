import re

lines = open("input.txt", 'r').read().split('\n\n')


def parseSet(s, d=', '):
    return tuple(int(x) for x in s.split(d))


def parseSection(section):

    before, instruction, after = re.search(
        r'Before: \[(.+)\]\n(.+)\nAfter:  \[(.+)\]', section.strip()).groups()
    return {'b': parseSet(before), 'i': parseSet(instruction, ' '), 'a': parseSet(after)}


part1_input = tuple(parseSection(x) for x in lines[:-1] if x.strip() != '')
part2_input = tuple(parseSet(x.strip(), ' ') for x in lines[-1].split('\n'))

##########################################
#                 PART 1                 #
##########################################


def mutate_tuple(i, t, v):
    return t[:i] + (v,) + t[i + 1:]


def op_addr(ins, reg):
    return mutate_tuple(ins[3], reg, reg[ins[1]] + reg[ins[2]])


def op_addi(ins, reg):
    return mutate_tuple(ins[3], reg, reg[ins[1]] + ins[2])


def op_mulr(ins, reg):
    return mutate_tuple(ins[3], reg, reg[ins[1]] * reg[ins[2]])


def op_muli(ins, reg):
    return mutate_tuple(ins[3], reg, reg[ins[1]] * ins[2])


def op_banr(ins, reg):
    return mutate_tuple(ins[3], reg, reg[ins[1]] & reg[ins[2]])


def op_bani(ins, reg):
    return mutate_tuple(ins[3], reg, reg[ins[1]] & ins[2])


def op_borr(ins, reg):
    return mutate_tuple(ins[3], reg, reg[ins[1]] | reg[ins[2]])


def op_bori(ins, reg):
    return mutate_tuple(ins[3], reg, reg[ins[1]] | ins[2])


def op_setr(ins, reg):
    return mutate_tuple(ins[3], reg, reg[ins[1]])


def op_seti(ins, reg):
    return mutate_tuple(ins[3], reg, ins[1])


def op_gtir(ins, reg):
    return mutate_tuple(ins[3], reg, 1 if ins[1] > reg[ins[2]] else 0)


def op_gtri(ins, reg):
    return mutate_tuple(ins[3], reg, 1 if reg[ins[1]] > ins[2] else 0)


def op_gtrr(ins, reg):
    return mutate_tuple(ins[3], reg, 1 if reg[ins[1]] > reg[ins[2]] else 0)


def op_eqir(ins, reg):
    return mutate_tuple(ins[3], reg, 1 if ins[1] == reg[ins[2]] else 0)


def op_eqri(ins, reg):
    return mutate_tuple(ins[3], reg, 1 if reg[ins[1]] == ins[2] else 0)


def op_eqrr(ins, reg):
    return mutate_tuple(ins[3], reg, 1 if reg[ins[1]] == reg[ins[2]] else 0)


possible_ops = {
    'addr': op_addr,
    'addi': op_addi,
    'mulr': op_mulr,
    'muli': op_muli,
    'banr': op_banr,
    'bani': op_bani,
    'borr': op_borr,
    'bori': op_bori,
    'setr': op_setr,
    'seti': op_seti,
    'gtir': op_gtir,
    'gtri': op_gtri,
    'gtrr': op_gtrr,
    'eqir': op_eqir,
    'eqri': op_eqri,
    'eqrr': op_eqrr
}


def get_possible_ops(test):
    before, instruction, after = test['b'], test['i'], test['a']

    res = set()
    for op in possible_ops:
        if possible_ops[op](instruction, before) == after:
            res.add(op)

    return res


def part1(tests):
    return len(list(filter(lambda x: x, map(lambda x: len(get_possible_ops(x)) >= 3, tests))))


print('Answer to part 1 is', part1(part1_input))

##########################################
#                 PART 2                 #
##########################################


def find_opcodes(part1_input):
    possibilities = {k: set(possible_ops.keys())
                     for k in range(len(possible_ops))}

    for test in part1_input:
        possibilities[test['i'][0]] = possibilities[test['i']
                                                    [0]].intersection(get_possible_ops(test))

    mapping = {}
    while len(possibilities) > 0:
        for op in list(possibilities.keys()):
            if len(possibilities[op]) == 1:
                mapping[op] = list(possibilities[op])[0]
                s = possibilities[op]
                del possibilities[op]
                for k in possibilities:
                    possibilities[k] = possibilities[k].difference(s)

    return mapping


def part2(part1_input, program):
    opcodes = find_opcodes(part1_input)
    reg = (0,) * 4

    for ins in program:
        reg = possible_ops[opcodes[ins[0]]](ins, reg)

    return reg[0]


print('Answer to part 2 is', part2(part1_input, part2_input))
