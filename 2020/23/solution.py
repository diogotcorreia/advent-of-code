label = open("input.txt", 'r').read().strip()

cups = [int(x) for x in list(label)]


##########################################
#                 PART 1                 #
##########################################

def build_list(cups):
    circular = {cups[0]: {'label': cups[0]}}
    for i in range(1, len(cups)):
        cup = cups[i]
        circular[cup] = {'label': cup, 'prev': circular[cups[i-1]]}
        circular[cups[i - 1]]['next'] = circular[cup]
    circular[cups[-1]]['next'] = circular[cups[0]]
    circular[cups[0]]['prev'] = circular[cups[-1]]

    return circular


def simulate(cups, current, moves, max_cup):
    for i in range(moves):
        # pick up
        picked_up = current['next']
        last_pickup = picked_up['next']['next']
        after_pickup = last_pickup['next']
        current['next'] = after_pickup
        after_pickup['prev'] = current

        value = current['label'] - 1
        picked_up_values = (picked_up['label'],
                            picked_up['next']['label'], last_pickup['label'])
        while value == 0 or value in picked_up_values:
            if value <= 1:
                value = max_cup
            else:
                value -= 1

        destination = cups[value]

        last_pickup['next'] = destination['next']
        destination['next']['prev'] = last_pickup
        destination['next'] = picked_up
        picked_up['prev'] = destination

        current = current['next']

    return cups[1]


def part1(cups):
    circ = build_list(cups)
    cup = simulate(circ, circ[cups[0]], 100, 9)['next']

    digits = ''
    while cup['label'] != 1:
        digits += str(cup['label'])
        cup = cup['next']

    return digits


print('Answer to part 1 is', part1(cups.copy()))

##########################################
#                 PART 2                 #
##########################################


def part2(cups):
    circ = build_list(cups + list(range(10, 1000001)))
    cup = simulate(circ, circ[cups[0]], 10000000, 1000000)
    # print(cups[:15])
    return cup['next']['label'] * cup['next']['next']['label']


print('Answer to part 2 is', part2(cups))
