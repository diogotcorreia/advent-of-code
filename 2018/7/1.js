console.time("7-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

// Parse input
var steps = {};
for (let line of input.split("\n")) {
  line = line.trim();
  if (line.length === 0) continue;
  let condition = line.substring(5, 6);
  let step = line.substring(36, 37);
  if (!steps[condition]) steps[condition] = [];
  if (!steps[step]) steps[step] = [];
  steps[step].push(condition);
}

var totalSteps = Object.keys(steps).length;

var result = "";

while (result.length < totalSteps) {
  let possibleSteps = [];
  for (let step of Object.keys(steps)) {
    let conditions = steps[step];
    if (conditions === undefined) continue;
    if (conditions.length === 0) possibleSteps.push(step);
  }
  let nextStep = possibleSteps.sort()[0];
  for (let step of Object.keys(steps)) {
    let conditions = steps[step];
    if (conditions === undefined) continue;
    let index = conditions.indexOf(nextStep);
    if (index !== -1) conditions.splice(index, 1);
  }
  steps[nextStep] = undefined;
  result += nextStep;
}

console.log("The answer is:");
console.log(result);
console.timeEnd("7-part1");
