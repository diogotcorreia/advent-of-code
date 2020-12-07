import re

lines = [line.strip() for line in open('input.txt', 'r') if line.strip() != ""]


def createBagIfNotExists(bags, bagName):
    if bagName not in bags:
        bags[bagName] = {"up": [], "down": []}


# key: bag color
# value: {up: [(bag color, qnt)], down: [(bag color, qnt)]}
bags = {}

# parse input
for line in lines:
    rulesRegex = r"(.+) bags contain ((?:\d [a-z ]+? bags?,? ?)*)"
    (color, rules) = re.match(rulesRegex, line).groups()

    createBagIfNotExists(bags, color)

    if rules:
        for rule in rules.split(", "):
            (qnt, ruleColor) = re.match(
                r"(\d) ([a-z ]+?) bags?", rule).groups()

            createBagIfNotExists(bags, ruleColor)
            bags[ruleColor]["up"].append((color, qnt))

            bags[color]["down"].append((ruleColor, qnt))

##########################################
#                 PART 1                 #
##########################################


def part1(bags, start):
    result = set()
    upLine = bags[start]["up"]
    for (bag, qnt) in upLine:
        result.add(bag)
        result.update(part1(bags, bag))
    return result


print('Answer to part 1 is', len(part1(bags, "shiny gold")))

##########################################
#                 PART 2                 #
##########################################


def part2(bags, start):
    result = 1
    downLine = bags[start]["down"]
    for (bag, qnt) in downLine:
        result += int(qnt) * part2(bags, bag)
    return result


print('Answer to part 2 is', part2(bags, "shiny gold") - 1)
