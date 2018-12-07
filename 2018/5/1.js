console.time("5-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

input = input.split("\n")[0].trim(); // We only want the first line

const getCharCase = (char) => {
  if (char === char.toUpperCase()) return true;
  return false;
};

var madeChanges = false;
do {
  madeChanges = false;

  let lastChar;
  for (let i = 0; i < input.length; i++) {
    let currentChar = input.charAt(i);
    if (!lastChar) {
      lastChar = currentChar;
      continue;
    }
    if (
      lastChar.toUpperCase() === currentChar.toUpperCase() &&
      getCharCase(lastChar) !== getCharCase(currentChar)
    ) {
      madeChanges = true;
      input = input.substring(0, i - 1) + input.substring(i + 1);
      lastChar = undefined;
    } else lastChar = currentChar;
  }
} while (madeChanges);

console.log("The answer is:");
console.log(input.length);
console.timeEnd("5-part1");
