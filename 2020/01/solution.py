puzzleInput = [int(n) for n in open('input.txt', 'r')]

##########################################
#                 PART 1                 #
##########################################


def part1(values, target):
    for v in values:
        if target - v in values:
            return v * (target - v)


print('Answer to part 1 is', part1(puzzleInput, 2020))

##########################################
#                 PART 2                 #
##########################################


def part2(values):
    for v in values:
        mul = part1(values, 2020 - v)
        if mul is not None:
            return v * mul


print('Answer to part 2 is', part2(puzzleInput))
