console.time("1-part2");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

var array = input.split("\n");

var current = 0;
var pastFreq = [];
outerLoop: while (true) {
  for (var i = 0; i < array.length; i++) {
    let value = array[i];
    if (value.substring(0, 1) === "+") {
      current += parseInt(value.substring(1).trim(), 10);
    } else if (value.substring(0, 1) === "-") {
      current -= parseInt(value.substring(1).trim(), 10);
    } else continue;
    if (pastFreq.includes(current)) break outerLoop;
    pastFreq.push(current);
  }
}

console.log("The answer is:");
console.log(current);
console.log(a);
console.timeEnd("1-part2");
