import re

lines = [line.strip() for line in open('input.txt', 'r')]

passports = []
currentPassport = {}
for line in lines:
    if line == "":
        passports.append(currentPassport)
        currentPassport = {}
        continue
    for prop in line.split(" "):
        propSplit = prop.split(":")
        currentPassport[propSplit[0]] = propSplit[1]
passports.append(currentPassport)


def heightCheck(x):
    match = re.fullmatch(r'(\d+)(cm|in)', x)
    if match is None:
        return False
    (height, unit) = match.groups()
    if unit == 'cm':
        return 150 <= int(height) <= 193
    return 59 <= int(height) <= 76


fields = (('byr', lambda x: 1920 <= int(x) <= 2002),
          ('iyr', lambda x: 2010 <= int(x) <= 2020),
          ('eyr', lambda x: 2020 <= int(x) <= 2030),
          ('hgt', heightCheck),
          ('hcl', lambda x: re.fullmatch(r'#[a-f0-9]{6}', x) != None),
          ('ecl', lambda x: x in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth')),
          ('pid', lambda x: re.fullmatch(r'\d{9}', x) != None))

##########################################
#                 PART 1                 #
##########################################


def part1(pp, advancedValidation):
    count = 0
    for passport in pp:
        valid = True
        for field, verification in fields:
            if field not in passport or (advancedValidation and not verification(passport[field])):
                valid = False
                break
        count += valid
    return count


print('Answer to part 1 is', part1(passports, False))

##########################################
#                 PART 2                 #
##########################################


print('Answer to part 2 is', part1(passports, True))
