console.time("4-part2");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

const regex = /\[1518-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.+)/;
const shiftRegex = /Guard #(\d+) begins shift/;

// Functions

const getHighestEntry = (object) => {
  let result = -1;
  let value = 0;
  for (let key of Object.keys(object)) {
    if (object[key] > value) {
      result = key;
      value = object[key];
    }
  }
  return [result, value];
};

// End functions

var guardMinutes = {};
let fellAsleep = -1;
let guardId = -1;

// Parse data
for (let value of input.split("\n").sort()) {
  if (value.trim() === "") continue;
  let match = regex.exec(value.trim());
  let minute = parseInt(match[4]);
  let content = match[5];
  if (content === "wakes up") {
    for (var i = fellAsleep; i < minute; i++) guardMinutes[guardId][i]++;
  } else if (content === "falls asleep") {
    fellAsleep = minute;
  } else {
    let contentMatch = shiftRegex.exec(content);
    if (contentMatch === null) continue; // Invalid content? Shouldn't happen
    guardId = contentMatch[1];
    if (!guardMinutes[guardId]) guardMinutes[guardId] = Array(60).fill(0);
  }
}

// Get the highest one
let minute = -1;
let guard = -1;
let value = 0;
for (let guardId of Object.keys(guardMinutes)) {
  let bestEntry = getHighestEntry(guardMinutes[guardId]);
  if (bestEntry[1] > value) {
    minute = bestEntry[0];
    guard = guardId;
    value = bestEntry[1];
  }
}

console.log("The answer is:");
console.log(guard * minute);
console.timeEnd("4-part2");
