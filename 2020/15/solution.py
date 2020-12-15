lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]
startingNumbers = [int(i) for i in lines[0].split(",")]

##########################################
#                 PART 1                 #
##########################################


def part1(startingNumbers, target):
    i = 0
    lastNumber = -1
    history = {}
    for n in startingNumbers:
        if n not in history:
            history[n] = []
        history[n].append(i)
        i += 1

    while i < target:
        if lastNumber in history and len(history[lastNumber]) >= 2:
            a, b = history[lastNumber][-2:]
            n = b - a
        else:
            n = 0
        if n not in history:
            history[n] = []
        history[n].append(i)
        lastNumber = n
        i += 1

    return lastNumber


print('Answer to part 1 is', part1(startingNumbers, 2020))

##########################################
#                 PART 2                 #
##########################################

# takes a minute but works
print('Answer to part 2 is', part1(startingNumbers, 30000000))
