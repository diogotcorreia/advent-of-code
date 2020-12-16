import re
lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]

yourTicketIndex = lines.index("your ticket:")


def stringListToInt(l):
    return list(map(lambda x: int(x), l))


def parseField(f):
    groups = re.fullmatch(r'(.+): (\d+)-(\d+) or (\d+)-(\d+)', f).groups()
    return {'name': groups[0], 'intervals': stringListToInt(groups[1:])}


fields = [parseField(f) for f in lines[:yourTicketIndex]]

yourTicket = stringListToInt(lines[yourTicketIndex + 1].split(","))
nearbyTickets = [stringListToInt(t.split(","))
                 for t in lines[yourTicketIndex + 3:]]

##########################################
#                 PART 1                 #
##########################################


def part1(tickets, fields):
    def getErrorRate(attr):
        for f in fields:
            a, b, c, d = f["intervals"]
            if a <= attr <= b or c <= attr <= d:
                return 0
        return attr

    errorRate = 0
    validTickets = []
    for ticket in tickets:
        valid = True
        for attribute in ticket:
            er = getErrorRate(attribute)
            if er != 0:
                valid = False
            errorRate += er
        if valid:
            validTickets.append(ticket)

    return errorRate, validTickets


errorRate, validTickets = part1(nearbyTickets, fields)
print('Answer to part 1 is', errorRate)

##########################################
#                 PART 2                 #
##########################################


def part2(tickets, fields):
    possibilities = [set(range(len(fields))) for i in range(len(fields))]

    def getPossibleFieldsIndex(attr):
        s = set()
        for i in range(len(fields)):
            a, b, c, d = fields[i]["intervals"]
            if a <= attr <= b or c <= attr <= d:
                s.add(i)
        return s

    for ticket in tickets:
        for i in range(len(ticket)):
            attr = ticket[i]
            f = getPossibleFieldsIndex(attr)
            possibilities[i] = possibilities[i].intersection(f)

    columns = [None] * len(fields)
    while possibilities != [None] * len(fields):
        for i in range(len(possibilities)):
            v = possibilities[i]
            if type(v) == set and len(v) == 1:
                (j,) = v
                for p in possibilities:
                    if type(p) == set:
                        p.remove(j)
                possibilities[i] = None
                columns[i] = fields[j]['name']

    acc = 1
    for i in range(len(columns)):
        col = columns[i]
        if col.startswith("departure"):
            acc *= tickets[0][i]

    return acc


print('Answer to part 2 is', part2([yourTicket] + validTickets, fields))
