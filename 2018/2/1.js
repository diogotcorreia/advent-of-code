console.time("2-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

var doubleCount = 0;
var tripleCount = 0;

input.split("\n").forEach((value) => {
  let results = {};
  value
    .trim()
    .split("")
    .forEach((char) => {
      if (results[char]) results[char]++;
      else results[char] = 1;
    });
  let resultsVal = Object.values(results);
  if (resultsVal.includes(2)) doubleCount++;
  if (resultsVal.includes(3)) tripleCount++;
});

var result = doubleCount * tripleCount;

console.log("The answer is:");
console.log(result);
console.timeEnd("2-part1");
