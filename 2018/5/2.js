console.time("5-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt");

input = input.split("\n")[0].trim(); // We only want the first line

const getCharCase = (char) => {
  if (char === char.toUpperCase()) return true;
  return false;
};

const reactPolymer = (input) => {
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
  return input.length;
};

console.log("Hold on... This might take a while (around 30 seconds)");

var result = Number.MAX_SAFE_INTEGER;
var i = 26;
while (i--) {
  var polymerLength = reactPolymer(
    input.replace(new RegExp(String.fromCharCode(97 + i), "ig"), "")
  );
  if (polymerLength < result) result = polymerLength;
}

console.log("The answer is:");
console.log(result);
console.timeEnd("5-part1");
