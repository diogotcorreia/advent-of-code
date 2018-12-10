console.time("10-part2");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

const TRIES = 100000;

const regex = /position=< ?(-?\d+),  ?(-?\d+)> velocity=< ?(-?\d+),  ?(-?\d+)>/;

var stars = [];

for (let line of input.split("\n")) {
  if (line.trim().length === 0) continue;
  var [, x, y, vectorX, vectorY] = regex.exec(line.trim()).map((i) => parseInt(i));
  stars.push({ x, y, vectorX, vectorY });
}

const moveStars = () => {
  stars = stars.map((s) => ({
    x: s.x + s.vectorX,
    y: s.y + s.vectorY,
    vectorX: s.vectorX,
    vectorY: s.vectorY,
  }));
};

var minArea = Number.MAX_SAFE_INTEGER;

for (let i = 0; i < TRIES; i++) {
  moveStars();
  var yArray = stars.map((a) => a.y);
  var MIN_Y = Math.min(...yArray);
  var MAX_Y = Math.max(...yArray);
  var xArray = stars.map((a) => a.x);
  var MIN_X = Math.min(...xArray);
  var MAX_X = Math.max(...xArray);
  var area = (MAX_Y - MIN_Y + 1) * (MAX_X - MIN_X + 1);
  if (area > minArea) {
    console.log("The answer is:");
    console.log(i + " seconds");
    break;
  }
  minArea = area;
}

console.timeEnd("10-part2");
