from functools import reduce

lines = [line.strip() for line in open('input.txt', 'r')]

groups = []

currentGroup = []
for line in lines:
    if line == "":
        groups.append(currentGroup)
        currentGroup = []
    else:
        currentGroup.append(list(line))
if len(currentGroup) != 0:
    groups.append(currentGroup)


##########################################
#                 PART 1                 #
##########################################

def differentResponses(group):
    uniqueResponses = []
    for person in group:
        for response in person:
            if response not in uniqueResponses:
                uniqueResponses.append(response)
    return uniqueResponses


def countResponses(groups):
    return map(lambda a: len(a), groups)


def part1(groups):
    return reduce(lambda a, b: a + b, countResponses(map(lambda a: differentResponses(a), groups)))


print('Answer to part 1 is', part1(groups))

##########################################
#                 PART 2                 #
##########################################


def commonResponses(group):
    memberCount = len(group)
    responses = {}
    for person in group:
        for response in person:
            if response not in responses:
                responses[response] = 0
            responses[response] += 1
    return reduce(lambda a, b: a + [b] if responses[b] == memberCount else a, responses.keys(), [])


def part2(groups):
    return reduce(lambda a, b: a + b, countResponses(map(lambda a: commonResponses(a), groups)))


print('Answer to part 2 is', part2(groups))
