import numpy as np

instructions = [(line[0], int(line[1:]))
                for line in open("input.txt", 'r') if line.strip() != ""]


def run(instructions, ship):
    for (action, value) in instructions:
        ship.execute_instruction(action, value)
    return ship.get_location()

##########################################
#                 PART 1                 #
##########################################


class Ship:
    def __init__(self):
        self.east = 0
        self.north = 0
        self.direction = 0  # 0 - east; 1 - south; 2 - west; 3 - north

    def rotate(self, angle):
        self.direction = (angle // 90 + self.direction) % 4

    def move(self, direction, value):
        if direction == 0:
            self.east += value
        elif direction == 1:
            self.north -= value
        elif direction == 2:
            self.east -= value
        elif direction == 3:
            self.north += value

    def execute_instruction(self, action, value):
        if action == "L":
            self.rotate(-value)
        elif action == "R":
            self.rotate(value)
        elif action in ["E", "S", "W", "N"]:
            dic = {'E': 0, 'S': 1, 'W': 2, 'N': 3}
            self.move(dic[action], value)
        elif action == "F":
            self.move(self.direction, value)

    def get_location(self):
        return abs(self.east) + abs(self.north)


print('Answer to part 1 is', run(instructions, Ship()))

##########################################
#                 PART 2                 #
##########################################


class ShipWithWaypoint:
    def __init__(self):
        self.east = 0
        self.north = 0
        self.waypointEast = 10
        self.waypointNorth = 1

    def rotate(self, angle):
        angle = np.deg2rad(angle)
        self.waypointEast, self.waypointNorth = \
            self.waypointEast * \
            round(np.cos(angle)) - self.waypointNorth * round(np.sin(angle)), \
            self.waypointEast * \
            round(np.sin(angle)) + self.waypointNorth * round(np.cos(angle))

    def move_waypoint(self, direction, value):
        if direction == 0:
            self.waypointEast += value
        elif direction == 1:
            self.waypointNorth -= value
        elif direction == 2:
            self.waypointEast -= value
        elif direction == 3:
            self.waypointNorth += value

    def move_to_waypoint(self, count):
        self.east += self.waypointEast * count
        self.north += self.waypointNorth * count

    def execute_instruction(self, action, value):
        if action == "L":
            self.rotate(value)
        elif action == "R":
            self.rotate(-value)
        elif action in ["E", "S", "W", "N"]:
            dic = {'E': 0, 'S': 1, 'W': 2, 'N': 3}
            self.move_waypoint(dic[action], value)
        elif action == "F":
            self.move_to_waypoint(value)

    def get_location(self):
        return abs(self.east) + abs(self.north)


print('Answer to part 2 is', run(instructions, ShipWithWaypoint()))
