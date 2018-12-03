console.time("3-part2");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

const removeFromAnswers = (id) => {
  var index = possibleAnswers.indexOf(id);
  if (index !== -1) possibleAnswers.splice(index, 1);
};

const regex = /#(\d+) @ (\d+),(\d+): (\d+)x(\d+)/;
var fabricMap = [];
var possibleAnswers = [];

for (let value of input.split("\n")) {
  if (value.trim() === "") continue;
  let match = regex.exec(value.trim());
  let id = parseInt(match[1]);
  possibleAnswers.push(id);
  let x = parseInt(match[2]);
  let starterY = parseInt(match[3]);
  let maxX = x + parseInt(match[4]);
  let maxY = starterY + parseInt(match[5]);
  for (; x < maxX; x++) {
    for (let y = starterY; y < maxY; y++) {
      if (!fabricMap[x]) fabricMap[x] = [];
      if (!fabricMap[x][y]) fabricMap[x][y] = [id];
      else {
        fabricMap[x][y].push(id);
        for (let tileOwners of fabricMap[x][y]) removeFromAnswers(tileOwners);
      }
    }
  }
}

var result = possibleAnswers[0];

console.log("The answer is:");
console.log(result);
console.timeEnd("3-part2");
