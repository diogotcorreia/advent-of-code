console.time("10-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

const TRIES = 100000;

const regex = /position=< ?(-?\d+),  ?(-?\d+)> velocity=< ?(-?\d+),  ?(-?\d+)>/;

var stars = [];

for (let line of input.split("\n")) {
  if (line.trim().length === 0) continue;
  var [, x, y, vectorX, vectorY] = regex.exec(line.trim()).map((i) => parseInt(i));
  stars.push({ x, y, vectorX, vectorY });
}

const compareFunction = (a, b) => {
  if (a.y === b.y) return a.x - b.x;
  return a.y - b.y;
};

const drawStars = () => {
  var yArray = stars.map((a) => a.y);
  var MIN_Y = Math.min(...yArray);
  var MAX_Y = Math.max(...yArray);
  var xArray = stars.map((a) => a.x);
  var MIN_X = Math.min(...xArray);
  var MAX_X = Math.max(...xArray);
  var orderedStars = [...stars].sort(compareFunction);

  console.log("The answer is:");
  for (let y = MIN_Y; y <= MAX_Y; y++) {
    let line = "";
    if (orderedStars.length === 0 || orderedStars[0].y !== y) line = ".".repeat(MAX_X - MIN_X + 1);
    else
      for (let x = MIN_X; x <= MAX_X; x++) {
        if (orderedStars.length !== 0 && orderedStars[0].y === y && orderedStars[0].x === x) {
          line += "#";
          while (orderedStars.length !== 0 && orderedStars[0].x === x && orderedStars[0].y === y)
            orderedStars.shift();
        } else line += ".";
      }
    console.log(line);
  }
};

const moveStars = () => {
  stars = stars.map((s) => ({
    x: s.x + s.vectorX,
    y: s.y + s.vectorY,
    vectorX: s.vectorX,
    vectorY: s.vectorY,
  }));
};
const moveStarsBackwards = () => {
  stars = stars.map((s) => ({
    x: s.x - s.vectorX,
    y: s.y - s.vectorY,
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
    moveStarsBackwards();
    drawStars();
    break;
  }
  minArea = area;
}

console.timeEnd("10-part1");
