from functools import reduce

lines = [line.strip() for line in open('input.txt', 'r') if line.strip() != ""]

trees = [(row, col) for row in range(len(lines))
         for col in range(len(lines[row])) if lines[row][col] == '#']

colN = len(lines[0])


def isTree(row, col):
    while col >= colN:
        col -= colN

    return (row, col) in trees


##########################################
#                 PART 1                 #
##########################################


def part1(forest, rightCount, downCount):
    treeCount = 0
    for row in range(0, len(lines), downCount):
        col = (rightCount / downCount) * row
        if isTree(row, col):
            treeCount += 1

    return treeCount


print('Answer to part 1 is', part1(trees, 3, 1))

##########################################
#                 PART 2                 #
##########################################


def part2(forest):
    return reduce(lambda a, b: a * b, [part1(forest, right, down) for (right, down) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]])


print('Answer to part 2 is', part2(trees))
