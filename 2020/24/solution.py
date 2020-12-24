lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]

tiles = []

# e - 0; se - 1; sw - 2; w - 3; nw - 4; ne - 5

directions = ((1, -1, 0), (0, -1, 1), (-1, 0, 1),
              (-1, 1, 0), (0, 1, -1), (1, 0, -1))

for line in lines:
    tile = []
    while len(line) > 0:
        if line[0] == 's':
            tile.append(1 if line[1] == 'e' else 2)
            line = line[2:]
        elif line[0] == 'n':
            tile.append(4 if line[1] == 'w' else 5)
            line = line[2:]
        elif line[0] == 'e':
            tile.append(0)
            line = line[1:]
        elif line[0] == 'w':
            tile.append(3)
            line = line[1:]
    tiles.append(tile)

##########################################
#                 PART 1                 #
##########################################


def part1(tiles):
    blackTiles = set()
    for tile in tiles:
        coord = [0, 0, 0]
        for move in tile:
            vector = directions[move]
            for i in range(3):
                coord[i] += vector[i]
        coord = tuple(coord)
        if coord in blackTiles:
            blackTiles.remove(coord)
        else:
            blackTiles.add(coord)
    return blackTiles


blackTiles = part1(tiles)

print('Answer to part 1 is', len(blackTiles))

##########################################
#                 PART 2                 #
##########################################


def get_adjacent(pos):
    adj = ()
    for vector in directions:
        res = list(pos)
        for i in range(3):
            res[i] += vector[i]
        adj += (tuple(res), )
    return adj


def part2(hexMap):
    for i in range(100):
        print('Day {}: {}'.format(i, len(hexMap)))
        frozen_map = set(hexMap)
        to_check = set()
        for tile in frozen_map:
            black_adj = 0
            for adj in get_adjacent(tile):
                if adj not in frozen_map:
                    to_check.add(adj)
                else:
                    black_adj += 1
            if black_adj == 0 or black_adj > 2:
                hexMap.remove(tile)
        for tile in to_check:
            black_adj = 0
            for adj in get_adjacent(tile):
                if adj in frozen_map:
                    black_adj += 1
            if black_adj == 2:
                hexMap.add(tile)
    return len(hexMap)


print('Answer to part 2 is', part2(blackTiles))
