from itertools import takewhile
from functools import reduce

lines = iter(line.strip() for line in open("input.txt", 'r'))

tiles = {}
for line in lines:
    tile = list(takewhile(lambda x: x != "", lines))
    tiles[int(line[5:9])] = tile

##########################################
#                 PART 1                 #
##########################################


def get_tile_borders(tile):
    left = ""
    right = ""
    for section in tile:
        left += section[0]
        right += section[-1]
    return (tile[0], right, tile[-1], left)


def get_borderset(tiles):
    borders = {}
    for tileId in tiles:
        tileBorders = get_tile_borders(tiles[tileId])
        for b in tileBorders:
            if b not in borders and b[::-1] not in borders:
                borders[b] = []
            elif b not in borders:
                b = b[::-1]
            borders[b].append(tileId)
    corner = list()
    for b in borders:
        if len(borders[b]) == 1:
            corner.append(borders[b][0])
    corner = set(filter(lambda x: corner.count(x) == 2, corner))
    return borders, corner


def part1(tiles):
    _, corner = get_borderset(tiles)
    return reduce(lambda a, b: a * b, corner)


print('Answer to part 1 is', part1(tiles))

##########################################
#                 PART 2                 #
##########################################


def is_equals_ignore_flip(side1, side2):
    return side1 == side2 or side1[::-1] == side2


def rotate_90deg_cw(tile):
    side_len = len(tile)
    new_tile = ['' for _ in range(side_len)]
    for column in range(side_len):
        for row in range(side_len):
            new_tile[column] += tile[row][column]
        new_tile[column] = new_tile[column][::-1]
    return new_tile


def flip_tile(tile, anchor):
    # flip vertical anchor
    if anchor % 2 == 0:
        return [row[::-1] for row in tile]
    # flip horizontal anchor
    return tile[::-1]


def get_correct_orientation(tile, targetSide, target):
    borders = get_tile_borders(tile)
    while not is_equals_ignore_flip(borders[targetSide], target):
        tile = rotate_90deg_cw(tile)
        borders = get_tile_borders(tile)

    if borders[targetSide] != target:
        tile = flip_tile(tile, targetSide)
    return tile


def get_first_tile_left_side(tile, tileId, borders):
    possible_corners = filter(
        lambda x: tileId in borders[x] and len(borders[x]) == 1, borders)
    tile_borders = get_tile_borders(tile)
    c1, c2 = list(
        map(lambda x: x if x in tile_borders else x[::-1], possible_corners))
    c1i, c2i = [tile_borders.index(x) for x in [c1, c2]]

    if c2i > c1i or (c2i == 0 and c1i == 3):
        c1, c2 = c2, c1
    if c2i <= 1:
        return c2[::-1]
    return c2


def get_tiles_with_border(borders, border):
    if border in borders:
        return borders[border]
    if border[::-1] in borders:
        return borders[border[::-1]]


def get_tile_row(leftmost_left, tiles, borders):
    row = []
    left_border = leftmost_left
    while True:
        tileId = list(filter(lambda x: x in tiles,
                             get_tiles_with_border(borders, left_border)))[0]
        tile = tiles[tileId]
        tile = get_correct_orientation(tile, 3, left_border)
        del tiles[tileId]
        row.append(tile)
        left_border = get_tile_borders(tile)[1]
        if len(get_tiles_with_border(borders, left_border)) == 1:
            break

    return row, tiles


def prettify_image(image):
    def remove_tile_borders(tile):
        tile = tile[1:-1]
        return [x[1:-1] for x in tile]

    res = []
    for row in image:
        row_img = ['' for _ in range(len(row[0][0]) - 2)]
        for tile in row:
            tile = remove_tile_borders(tile)
            for i in range(len(tile)):
                row_img[i] += tile[i]
        res.extend(row_img)

    return res


def find_seamonsters(image):
    col_count = len(image[0])

    #                   #
    # #    ##    ##    ###
    #  #  #  #  #  #  #
    monster_format = [(0, 0), (1, 1), (4, 1), (5, 0), (6, 0), (7, 1), (10, 1),
                      (11, 0), (12, 0), (13, 1), (16, 1), (17, 0), (18, -1), (18, 0), (19, 0)]

    def is_monster(x, y):
        for offsetX, offsetY in monster_format:
            if image[y + offsetY][x + offsetX] != "#":
                return False
        return True

    monster_count = 0

    for row in range(1, len(image) - 1):
        for col in range(col_count - 19):
            if is_monster(col, row):
                monster_count += 1
                for offsetX, offsetY in monster_format:
                    new_row = list(image[row + offsetY])
                    new_row[col + offsetX] = '.'
                    image[row + offsetY] = ''.join(new_row)

    return monster_count, image


def part2(tiles):
    borders, corner = get_borderset(tiles)
    leftmost_corner = list(corner)[0]  # get a random corner
    left_side = get_first_tile_left_side(
        tiles[leftmost_corner], leftmost_corner, borders)

    image = []
    while(len(tiles) > 0):
        row, tiles = get_tile_row(left_side, tiles, borders)
        image.append(row)
        bottom_side = get_tile_borders(row[0])[2]
        if len(tiles) == 0:
            break
        tileId = list(filter(lambda x: x in tiles,
                             get_tiles_with_border(borders, bottom_side)))[0]
        tile = tiles[tileId]
        tile = get_correct_orientation(tile, 0, bottom_side)
        left_side = get_tile_borders(tile)[3]

    image = prettify_image(image)

    seamonster_count, new_image = find_seamonsters(image.copy())
    transformation = 3
    while seamonster_count == 0:
        if transformation != 0:
            image = rotate_90deg_cw(image)
        else:
            image = flip_tile(image, 0)
        transformation -= 1
        seamonster_count, new_image = find_seamonsters(image.copy())

    # for x in image:
    #    print(x)

    # print(seamonster_count)

    return reduce(lambda a, x: a + x.count("#"), new_image, 0)


print('Answer to part 2 is', part2(tiles.copy()))
