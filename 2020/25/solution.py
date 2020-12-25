lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]

card_publickey = int(lines[0])
door_publickey = int(lines[1])

##########################################
#                 PART 1                 #
##########################################


def loop(subject, value):
    value *= subject
    value %= 20201227
    return value


def find_loopsize(publickey):
    v = 1
    i = 0
    while v != publickey:
        v = loop(7, v)
        i += 1
    return i


def part1(card_publickey, door_publickey):
    card_loopsize = find_loopsize(card_publickey)

    encry_key = 1
    for i in range(card_loopsize):
        encry_key = loop(door_publickey, encry_key)
    return encry_key


print('Answer to part 1 is', part1(card_publickey, door_publickey))

##########################################
#                 PART 2                 #
##########################################


def part2():
    pass


print('Answer to part 2 is', part2())
