from functools import reduce

lines = [int(line.strip())
         for line in open("input.txt", 'r') if line.strip() != ""]
lines = sorted(lines)

##########################################
#                 PART 1                 #
##########################################


def part1(adapters):
    adapterCount = len(adapters)

    differences = {1: 0, 3: 1}

    for i in range(adapterCount):
        if i == 0:
            differences[adapters[i]] += 1
        else:
            differences[adapters[i] - adapters[i - 1]] += 1

    return differences[1] * differences[3]


print('Answer to part 1 is', part1(lines))

##########################################
#                 PART 2                 #
##########################################


def getBranchesSizes(adapters):
    branches = []

    adapterCount = len(adapters)

    currentBranch = []
    for i in range(adapterCount):
        if i == 0:
            diff = adapters[i]
            if diff == 1:
                currentBranch.append(i)
        else:
            diff = adapters[i] - adapters[i - 1]
            if diff == 1:
                currentBranch.append(i)
            elif currentBranch != []:
                branches.append(len(currentBranch))
                currentBranch = []

    if currentBranch != []:
        branches.append(len(currentBranch))
        currentBranch = []
    return branches


def part2(adapters):
    branches = getBranchesSizes(adapters)

    # number of arrangements by count of contiguous numbers with a difference of 1
    # (my puzzle input didn't have any above 4)
    branchArrangementKey = {1: 1, 2: 2, 3: 4, 4: 7}

    return reduce(lambda a, b: a * branchArrangementKey[b], branches, 1)


print('Answer to part 2 is', part2(lines))
