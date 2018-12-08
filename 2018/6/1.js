console.time("6-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

// Parse input
var coordinates = [];
var MAX_GRID_X = 0;
var MAX_GRID_Y = 0;

for (var value of input.split("\n")) {
  if (value.trim().length == 0) continue;
  var [x, y] = value.split(", ");
  x = parseInt(x);
  y = parseInt(y);
  coordinates.push({ x, y });
  if (x > MAX_GRID_X) MAX_GRID_X = x;
  if (y > MAX_GRID_Y) MAX_GRID_Y = y;
}

var locationArea = Array(coordinates.length).fill(0);
var infiniteLocations = [];

MAX_GRID_X++;
MAX_GRID_Y++;
for (var x1 = 0; x1 <= MAX_GRID_X; x1++)
  for (var y1 = 0; y1 <= MAX_GRID_Y; y1++) {
    let minDistance = Number.MAX_SAFE_INTEGER;
    let minCoordinate;
    coordinates.forEach(({ x, y }, index) => {
      let distance = Math.abs(x - x1) + Math.abs(y - y1);
      if (distance < minDistance) {
        minDistance = distance;
        minCoordinate = index;
      } else if (distance == minDistance) {
        minCoordinate = undefined;
      }
    });
    if (minCoordinate !== undefined) {
      if (x1 === 0 || y1 === 0 || x1 === MAX_GRID_X || y1 === MAX_GRID_Y)
        if (infiniteLocations.indexOf(minCoordinate) < 0) infiniteLocations.push(minCoordinate);
      locationArea[minCoordinate]++;
    }
  }

for (let loc of infiniteLocations) locationArea[loc] = -1;
var result = Math.max(...locationArea);

console.log("The answer is:");
console.log(result);
console.timeEnd("6-part1");
