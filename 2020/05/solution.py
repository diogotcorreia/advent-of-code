from functools import reduce

lines = [line.strip() for line in open('input.txt', 'r') if line.strip() != ""]


def ticketToRowCol(ticket):
    def binarySpacePartioning(value, downChar, upChar, maxSize):
        l, h = 0, maxSize - 1
        for char in value:
            mid = (l+h) // 2
            if char == downChar:
                h = mid
            elif char == upChar:
                l = mid + 1
        return l

    rowEncoded = ticket[:7]
    colEncoded = ticket[7:]

    row = binarySpacePartioning(rowEncoded, 'F', 'B', 128)
    col = binarySpacePartioning(colEncoded, 'L', 'R', 8)
    return row, col


def ticketToId(ticket):
    row, col = ticket
    return row * 8 + col


tickets = [ticketToId(ticketToRowCol(line)) for line in lines]


##########################################
#                 PART 1                 #
##########################################


def part1(tickets):
    return max(tickets)


print('Answer to part 1 is', part1(tickets))

##########################################
#                 PART 2                 #
##########################################


def part2(tickets):
    for seat in range(part1(tickets)):
        if seat not in tickets and seat - 1 in tickets and seat + 1 in tickets:
            return seat


print('Answer to part 2 is', part2(tickets))
