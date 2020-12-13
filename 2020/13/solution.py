import numpy as np

lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]

startTimestamp = int(lines[0])
buses = [int(bus) if bus != "x" else -1 for bus in lines[1].split(",")]

##########################################
#                 PART 1                 #
##########################################


def part1(buses, startTimestamp):
    time = startTimestamp
    buses = list(filter(lambda x: x >= 0, buses))
    while True:
        for bus in buses:
            if time % bus == 0:
                return (time - startTimestamp) * bus
        time += 1


print('Answer to part 1 is', part1(buses, startTimestamp))

##########################################
#                 PART 2                 #
##########################################


def part2(buses):

    def get_next_valid(busA, busB):
        i = busA[1]
        while (i + busB[1]) % busB[0] != 0:
            i += busA[0]
        return np.lcm(busA[0], busB[0]), i

    buses = [(x, i) for i, x in enumerate(buses) if x != "-1"]

    current_bus = buses[0]
    buses = buses[1:]
    while len(buses) > 0:
        current_bus = get_next_valid(current_bus, buses[0])
        buses = buses[1:]

    return current_bus[1]


print('Answer to part 2 is', part2(buses))
