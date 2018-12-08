console.time("8-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt")
  .split("\n")[0]
  .trim()
  .split(" ")
  .map((i) => parseInt(i));

var result = 0;

const parseNode = (start, nodeCount) => {
  if (nodeCount === 0) return 0;
  var metadataRemaining = -1;
  for (let i = start; i < input.length; i++) {
    let value = input[i];
    if (metadataRemaining === -1) {
      metadataRemaining = input[i + 1];
      i += parseNode(i + 2, value);
      i++;
      continue;
    }
    result += value;
    metadataRemaining--;
    if (metadataRemaining === 0) {
      nodeCount--;
      if (nodeCount === 0) return i - start + 1;
      else metadataRemaining = -1;
    }
  }
};

parseNode(0, 1);

console.log("The answer is:");
console.log(result);
console.timeEnd("8-part1");
