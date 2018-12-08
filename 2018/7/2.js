console.time("7-part2");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

const BASE_TIME = 60;
const TOTAL_WORKERS = 5;

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

// Functions

Array.prototype.diff = function(a) {
  return this.filter(function(i) {
    return a.indexOf(i) < 0;
  });
};

const getPossibleSteps = () => {
  let possibleSteps = [];
  for (let step of Object.keys(steps)) {
    let conditions = steps[step];
    if (conditions === undefined) continue;
    if (conditions.length === 0) possibleSteps.push(step);
  }
  return possibleSteps.sort().diff(getCurrentSteps());
};

const getCurrentSteps = () => {
  let currentSteps = [];
  for (let worker of workers) if (worker.task !== undefined) currentSteps.push(worker.task);
  return currentSteps;
};

const markStepAsCompleted = (nextStep) => {
  for (let step of Object.keys(steps)) {
    let conditions = steps[step];
    if (conditions === undefined) continue;
    let index = conditions.indexOf(nextStep);
    if (index !== -1) conditions.splice(index, 1);
  }
  steps[nextStep] = undefined;
  completedSteps++;
};

// End functions

var totalSteps = Object.keys(steps).length;

var workers = Array(TOTAL_WORKERS)
  .fill()
  .map(() => ({}));
var completedSteps = 0;
var secondsElapsed = 0;

while (completedSteps < totalSteps) {
  let possibleSteps = getPossibleSteps();
  for (let id = 0; id < workers.length; id++) {
    var worker = workers[id];
    if (worker.task === undefined) {
      if (possibleSteps.length != 0) {
        workers[id].task = possibleSteps[0];
        workers[id].remaining = BASE_TIME + (possibleSteps[0].charCodeAt(0) - 64);
        possibleSteps.shift();
      }
    }
    worker.remaining--;
    if (worker.remaining === 0) {
      markStepAsCompleted(worker.task);
      worker.task = undefined;
    }
  }
  secondsElapsed++;
}

console.log("The answer is:");
console.log(secondsElapsed);
console.timeEnd("7-part2");
