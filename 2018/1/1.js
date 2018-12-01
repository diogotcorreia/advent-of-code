console.time("1-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

var result = 0;
input.split("\n").forEach((value) => {
  if (value.substring(0, 1) === "+") {
    result += parseInt(value.substring(1).trim(), 10);
  } else if (value.substring(0, 1) === "-") {
    result -= parseInt(value.substring(1).trim(), 10);
  }
});

console.log("The answer is:");
console.log(result);
console.timeEnd("1-part1");
