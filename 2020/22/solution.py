from itertools import takewhile

lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]

it = iter(lines[1:])

deck1 = [int(x) for x in takewhile(lambda x: not x.startswith("Player "), it)]
deck2 = [int(x) for x in it]


##########################################
#                 PART 1                 #
##########################################

def calculate_score(deck):
    deck = deck[::-1]
    score = 0
    for i in range(len(deck)):
        score += deck[i] * (i + 1)
    return score


def part1(deck1, deck2):
    while len(deck1) > 0 and len(deck2) > 0:
        c1, c2 = deck1.pop(0), deck2.pop(0)
        if c1 > c2:
            deck1.extend((c1, c2))
        else:
            deck2.extend((c2, c1))
    if len(deck1) > 0:
        return calculate_score(deck1)
    return calculate_score(deck2)


print('Answer to part 1 is', part1(deck1.copy(), deck2.copy()))

##########################################
#                 PART 2                 #
##########################################


def serialize_decks(deck1, deck2):
    return "{}:{}".format(','.join(map(str, deck1)), ','.join(map(str, deck2)))


def recursive_combat(deck1, deck2):
    deck_history = set()
    while len(deck1) > 0 and len(deck2) > 0:
        if serialize_decks(deck1, deck2) in deck_history:
            return deck1 + deck2, []
        deck_history.add(serialize_decks(deck1, deck2))
        c1, c2 = deck1.pop(0), deck2.pop(0)
        winner = c1 > c2
        if c1 <= len(deck1) and c2 <= len(deck2):
            winner = len(recursive_combat(deck1[:c1], deck2[:c2])[0]) > 0
        if winner:
            deck1.extend((c1, c2))
        else:
            deck2.extend((c2, c1))
    return deck1, deck2


def part2(deck1, deck2):
    deck1, deck2 = recursive_combat(deck1, deck2)
    if len(deck1) > 0:
        return calculate_score(deck1)
    return calculate_score(deck2)


print('Answer to part 2 is', part2(deck1.copy(), deck2.copy()))
