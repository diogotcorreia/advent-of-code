console.time("3-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

const regex = /#(\d+) @ (\d+),(\d+): (\d+)x(\d+)/;
var fabricMap = [];

for (let value of input.split("\n")) {
  if (value.trim() === "") continue;
  let match = regex.exec(value.trim());
  let x = parseInt(match[2]);
  let starterY = parseInt(match[3]);
  let maxX = x + parseInt(match[4]);
  let maxY = starterY + parseInt(match[5]);
  for (; x < maxX; x++) {
    for (let y = starterY; y < maxY; y++) {
      if (!fabricMap[x]) fabricMap[x] = [];
      if (!fabricMap[x][y]) fabricMap[x][y] = 1;
      else fabricMap[x][y]++;
    }
  }
}

var result = 0;

for (let x = 0; x < fabricMap.length; x++)
  if (fabricMap[x]) for (let y = 0; y < fabricMap[x].length; y++) if (fabricMap[x][y] > 1) result++;

console.log("The answer is:");
console.log(result);
console.timeEnd("3-part1");
