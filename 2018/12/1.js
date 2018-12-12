console.time("12-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

var plants = {};
var patterns = [];

for (let line of input.split("\n")) {
  if (line.startsWith("initial state:")) {
    line = line.substring(14).trim();
    for (let i = 0; i < line.length; i++) {
      plants[i] = line.charAt(i) === "#";
    }
  } else if (line.trim().length !== 0) {
    line = line.trim();
    if (line.charAt(9) === "#")
      patterns.push({
        left2: line.charAt(0) === "#",
        left: line.charAt(1) === "#",
        center: line.charAt(2) === "#",
        right: line.charAt(3) === "#",
        right2: line.charAt(4) === "#",
      });
  }
}

const nextGeneration = () => {
  var result = {};
  var values = [];
  for (let plant of Object.keys(plants)) if (plants[plant] === true) values.push(parseInt(plant));
  let minPlant = Math.min(...values) - 2;
  let maxPlant = Math.max(...values) + 2;
  plantLoop: for (let id = minPlant; id < maxPlant; id++) {
    for (let pattern of patterns) {
      if (
        (plants[id - 2] || false) === pattern.left2 &&
        (plants[id - 1] || false) === pattern.left &&
        (plants[id] || false) === pattern.center &&
        (plants[id + 1] || false) === pattern.right &&
        (plants[id + 2] || false) === pattern.right2
      ) {
        result[id] = true;
        continue plantLoop;
      }
      result[id] = false;
    }
  }
  return result;
};

for (let i = 0; i < 20; i++) {
  plants = nextGeneration();
}

var result = 0;
for (let plant of Object.keys(plants)) if (plants[plant] === true) result += parseInt(plant);

console.log("The answer is:");
console.log(result);
console.timeEnd("12-part1");
