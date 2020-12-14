import re
from functools import reduce


lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]


##########################################
#                 PART 1                 #
##########################################

def applyMask(mask, i):
    i = int(i) & int(mask.replace("X", "1"), 2)
    return i | int(mask.replace("X", "0"), 2)


def part1(lines):
    mask = ""
    memory = {}
    for line in lines:
        if line.startswith("mask = "):
            mask = line.replace("mask = ", "")
        else:
            addr, value = re.match(r"mem\[(\d+)\] = (\d+)", line).groups()
            memory[addr] = applyMask(mask, value)

    return reduce(lambda a, b: a + b, memory.values())


print('Answer to part 1 is', part1(lines))

##########################################
#                 PART 2                 #
##########################################


def getPossibleMasks(mask):
    if 'X' in mask:
        return getPossibleMasks(mask.replace("X", "0", 1)) + getPossibleMasks(mask.replace("X", "1", 1))
    return [mask]


def part2(lines):
    mask = []
    memory = {}
    for line in lines:
        if line.startswith("mask = "):
            mask = list(map(lambda a: a.replace("Y", "X"), getPossibleMasks(line.replace(
                "mask = ", "").replace("0", "Y"))))
        else:
            addr, value = re.match(r"mem\[(\d+)\] = (\d+)", line).groups()
            for m in mask:
                memory[applyMask(m, addr)] = int(value)

    return reduce(lambda a, b: a + b, memory.values())


print('Answer to part 2 is', part2(lines))
