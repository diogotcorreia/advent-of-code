console.time("2-part2");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

var inputSplit = [];
var result = "";
outerLoop: for (let value of input.split("\n")) {
  if (value.trim() === "") continue;
  var chars = value.trim().split("");
  for (let boxChars of inputSplit) {
    let incorrectIds = 0;
    result = "";
    for (var i = 0; i < boxChars.length; i++) {
      if (incorrectIds > 1) break;
      if (boxChars[i] !== chars[i]) incorrectIds++;
      else result += chars[i];
    }
    if (incorrectIds === 1) break outerLoop;
  }
  inputSplit.push(chars);
}

console.log("The answer is:");
console.log(result);
console.timeEnd("2-part2");
