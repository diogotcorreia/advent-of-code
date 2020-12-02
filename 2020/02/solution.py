import re

puzzleInput = [re.match(r'(\d+)-(\d+) (\w): (\w+)', line).groups()
               for line in open('input.txt', 'r')]


def countValid(passwords, criteria):
    count = 0
    for password in passwords:
        count += criteria(password)  # 0 - false, 1 - true
    return count

##########################################
#                 PART 1                 #
##########################################


def part1(password):
    (lower, upper, char, string) = password
    numChar = len(string) - len(string.replace(char, ''))
    return int(lower) <= numChar <= int(upper)


print('Answer to part 1 is', countValid(puzzleInput, part1))

##########################################
#                 PART 2                 #
##########################################


def part2(password):
    (lower, upper, char, string) = password
    # XOR
    return (string[int(lower) - 1] == char) != (string[int(upper) - 1] == char)


print('Answer to part 2 is', countValid(puzzleInput, part2))
