lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]

seatKey = {
    "#": 1,
    "L": 0,
    ".": -1,
}

columns = len(lines[0])

seats = [seatKey[seat] for line in lines for seat in line]


def simulate(seats, criteria):
    changed = True
    while changed:
        seatsClone = seats + []
        changed = False
        for i in range(len(seatsClone)):
            seat = seatsClone[i]
            if seat == -1:
                continue
            newSeat = criteria(seats, seat, i)
            if seat != newSeat:
                changed = True
                seatsClone[i] = newSeat
        seats = seatsClone

    return seats.count(1)


def printBoard(seats):
    """
    for debugging
    """
    k = {
        -1: ".",
        0: "L",
        1: "#"
    }
    for i in range(len(seats)):
        seat = seats[i]
        if i % columns == 0:
            print()
        print(k[seat], end="")
    print()

##########################################
#                 PART 1                 #
##########################################


def getAdjecentSeats(seats, i):
    adjecentSeats = []

    # if not leftbound seat
    if i % columns != 0:
        if seats[i - 1] >= 0:
            adjecentSeats.append(seats[i - 1])
        # Diagonals
        if (i - 1 - columns) >= 0 and seats[i - 1 - columns] >= 0:
            adjecentSeats.append(seats[i - 1 - columns])
        if (i - 1 + columns) < len(seats) and seats[i - 1 + columns] >= 0:
            adjecentSeats.append(seats[i - 1 + columns])
    # if not rightbound seat
    if (i + 1) % columns != 0:
        if seats[i + 1] >= 0:
            adjecentSeats.append(seats[i + 1])
        # Diagonals
        if (i + 1 - columns) >= 0 and seats[i + 1 - columns] >= 0:
            adjecentSeats.append(seats[i + 1 - columns])
        if (i + 1 + columns) < len(seats) and seats[i + 1 + columns] >= 0:
            adjecentSeats.append(seats[i + 1 + columns])
    # if not in first row
    if i - columns >= 0 and seats[i - columns] >= 0:
        adjecentSeats.append(seats[i - columns])
    # if not in last row
    if i + columns < len(seats) and seats[i + columns] >= 0:
        adjecentSeats.append(seats[i + columns])

    return adjecentSeats


def part1(seats, seat, i):
    adjecentSeats = getAdjecentSeats(seats, i)
    if seat == 0:
        if adjecentSeats == [] or all(adSeat == 0 for adSeat in adjecentSeats):
            return 1
    elif seat == 1:
        if adjecentSeats.count(1) >= 4:
            return 0
    return seat


print('Answer to part 1 is', simulate(seats, part1))

##########################################
#                 PART 2                 #
##########################################


def getSeatsInView(seats, i):
    def toRowCol(pos): return (pos // columns, pos % columns)
    def toPos(row, col): return row * columns + col

    def getFirst(row, col, nextPos):
        row, col = nextPos(row, col)
        def withinBounds(
            row, col): return 0 <= col < columns and 0 <= row < len(lines)
        while withinBounds(row, col):
            if seats[toPos(row, col)] >= 0:
                return seats[toPos(row, col)]
            row, col = nextPos(row, col)
        return -1

    inView = []

    row, col = toRowCol(i)

    # to left
    inView.append(getFirst(row, col, lambda r, c: (r, c - 1)))
    # to right
    inView.append(getFirst(row, col, lambda r, c: (r, c + 1)))
    # up
    inView.append(getFirst(row, col, lambda r, c: (r - 1, c)))
    # down
    inView.append(getFirst(row, col, lambda r, c: (r + 1, c)))
    # up left
    inView.append(getFirst(row, col, lambda r, c: (r - 1, c - 1)))
    # up right
    inView.append(getFirst(row, col, lambda r, c: (r - 1, c + 1)))
    # down left
    inView.append(getFirst(row, col, lambda r, c: (r + 1, c - 1)))
    # down right
    inView.append(getFirst(row, col, lambda r, c: (r + 1, c + 1)))

    return list(filter(lambda x: x >= 0, inView))


def part2(seats, seat, i):
    inViewSeats = getSeatsInView(seats, i)
    if seat == 0:
        if inViewSeats == [] or all(inViewSeat == 0 for inViewSeat in inViewSeats):
            return 1
    elif seat == 1:
        if inViewSeats.count(1) >= 5:
            return 0
    return seat


print('Answer to part 2 is', simulate(seats, part2))
