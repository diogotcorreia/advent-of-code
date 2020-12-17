lines = [line.strip() for line in open("input.txt", 'r') if line.strip() != ""]


class ThreeDSpace:

    def __init__(self, space=[]):
        self.maxX, self.minX, self.maxY, self.minY, self.maxZ, self.minZ = 0, 0, 0, 0, 0, 0
        self.space = space

    def ensureMinMax(self, x, y, z):
        self.minX = x if x < self.minX else self.minX
        self.minY = y if y < self.minY else self.minY
        self.minZ = z if z < self.minZ else self.minZ
        self.maxX = x if x > self.maxX else self.maxX
        self.maxY = y if y > self.maxY else self.maxY
        self.maxZ = z if z > self.maxZ else self.maxZ

    def getState(self, x, y, z):
        return {'x': x, 'y': y, 'z': z} in self.space

    def setState(self, x, y, z, active):
        if active:
            self.ensureMinMax(x, y, z)
            if not self.getState(x, y, z):
                self.space.append({'x': x, 'y': y, 'z': z})
        else:
            if self.getState(x, y, z):
                self.space.remove({'x': x, 'y': y, 'z': z})

    def getSpaceGrid(self):
        return self.space

    def getSpaceSize(self):
        return (self.minX, self.maxX, self.minY, self.maxY, self.minZ, self.maxZ)

    def clone(self):
        return ThreeDSpace(self.space + [])


class FourDSpace(ThreeDSpace):

    def __init__(self, space=[]):
        super(FourDSpace, self).__init__(space)

        self.maxW, self.minW = 0, 0

    def ensureMinMax(self, x, y, z, w):
        super(FourDSpace, self).ensureMinMax(x, y, z)
        self.minW = w if w < self.minW else self.minW
        self.maxW = w if w > self.maxW else self.maxW

    def getState(self, x, y, z, w):
        return {'x': x, 'y': y, 'z': z, 'w': w} in self.space

    def setState(self, x, y, z, w, active):
        if active:
            self.ensureMinMax(x, y, z, w)
            if not self.getState(x, y, z, w):
                self.space.append({'x': x, 'y': y, 'z': z, 'w': w})
        else:
            if self.getState(x, y, z, w):
                self.space.remove({'x': x, 'y': y, 'z': z, 'w': w})

    def getSpaceSize(self):
        return super().getSpaceSize() + (self.minW, self.maxW)

    def clone(self):
        return FourDSpace(self.space + [])


spacePart1 = ThreeDSpace()
spacePart2 = FourDSpace()

for y in range(len(lines)):
    line = lines[y]
    for x in range(len(line)):
        spacePart1.setState(x, y, 0, line[x] == "#")
        spacePart2.setState(x, y, 0, 0, line[x] == "#")

##########################################
#                 PART 1                 #
##########################################


def part1(space):
    def getNeighbors(x, y, z):
        acc = []
        for a in range(x - 1, x + 2):
            for b in range(y - 1, y + 2):
                for c in range(z - 1, z + 2):
                    if not (a == x and b == y and c == z):
                        acc.append((a, b, c))
        return acc

    for i in range(6):
        localSpace = space.clone()
        minX, maxX, minY, maxY, minZ, maxZ = space.getSpaceSize()
        for x in range(minX - 1, maxX + 2):
            for y in range(minY - 1, maxY + 2):
                for z in range(minZ - 1, maxZ + 2):
                    count = 0
                    for nX, nY, nZ in getNeighbors(x, y, z):
                        if count > 3:
                            break
                        if localSpace.getState(nX, nY, nZ):
                            count += 1
                    if localSpace.getState(x, y, z):
                        if count < 2 or count > 3:
                            space.setState(x, y, z, False)
                    else:
                        if count == 3:
                            space.setState(x, y, z, True)

    return len(space.getSpaceGrid())


print('Answer to part 1 is', part1(spacePart1))

##########################################
#                 PART 2                 #
##########################################


def part2(space):
    def getNeighbors(x, y, z, w):
        acc = []
        for a in range(x - 1, x + 2):
            for b in range(y - 1, y + 2):
                for c in range(z - 1, z + 2):
                    for d in range(w - 1, w + 2):
                        if not (a == x and b == y and c == z and d == w):
                            acc.append((a, b, c, d))
        return acc

    for i in range(6):
        localSpace = space.clone()
        minX, maxX, minY, maxY, minZ, maxZ, minW, maxW = space.getSpaceSize()
        for x in range(minX - 1, maxX + 2):
            for y in range(minY - 1, maxY + 2):
                for z in range(minZ - 1, maxZ + 2):
                    for w in range(minW - 1, maxW + 2):
                        count = 0
                        for nX, nY, nZ, nW in getNeighbors(x, y, z, w):
                            if count > 3:
                                break
                            if localSpace.getState(nX, nY, nZ, nW):
                                count += 1
                        if localSpace.getState(x, y, z, w):
                            if count < 2 or count > 3:
                                space.setState(x, y, z, w, False)
                        else:
                            if count == 3:
                                space.setState(x, y, z, w, True)

    return len(space.getSpaceGrid())


# takes 5 min to execute ;-;
print('Answer to part 2 is', part2(spacePart2))
