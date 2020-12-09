
lines = [int(line.strip())
         for line in open("input.txt", 'r') if line.strip() != ""]

PREAMBLE_SIZE = 25


def calcPreambleSum(preamble):
    sums = []
    for n1 in preamble:
        for n2 in preamble:
            if n1 != n2:
                sums.append(n1 + n2)
    return sums


##########################################
#                 PART 1                 #
##########################################


def part1(numbers):
    preamble = numbers[:PREAMBLE_SIZE]
    toAnalyze = numbers[PREAMBLE_SIZE:]
    while len(toAnalyze) != 0:
        num = toAnalyze[0]
        if num not in calcPreambleSum(preamble):
            return num
        preamble = preamble[1:] + [num]
        toAnalyze = toAnalyze[1:]


print('Answer to part 1 is', part1(lines))

##########################################
#                 PART 2                 #
##########################################


def part2(numbers):
    invalidNum = part1(numbers)

    for i in range(len(numbers)):
        acc = numbers[i]
        j = i
        while acc < invalidNum:
            j += 1
            acc += numbers[j]
        if j != i and acc == invalidNum:
            responseSet = numbers[i:j+1]
            return max(responseSet) + min(responseSet)


print('Answer to part 2 is', part2(lines))
