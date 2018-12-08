console.time("6-part2");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

const MAX_DISTANCE = 10000;

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

var result = 0;

MAX_GRID_X++;
MAX_GRID_Y++;
for (var x1 = 0; x1 <= MAX_GRID_X; x1++)
  for (var y1 = 0; y1 <= MAX_GRID_Y; y1++) {
    let totalDistance = 0;
    coordinates.forEach(({ x, y }) => {
      totalDistance += Math.abs(x - x1) + Math.abs(y - y1);
    });
    if (totalDistance < MAX_DISTANCE) result++;
  }

console.log("The answer is:");
console.log(result);
console.timeEnd("6-part2");
