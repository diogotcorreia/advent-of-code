console.time("11-part2");
var input = parseInt(
  require("../../utils/readTxt")(__dirname + "/input.txt")
    .split("\n")[0]
    .trim()
);

var grid = Array(300)
  .fill()
  .map((a) => Array(300).fill(0));

for (let x = 1; x <= 300; x++)
  for (let y = 1; y <= 300; y++) {
    let rackId = x + 10;
    let powerLevel = (rackId * y + input) * rackId;
    let digit = 0;
    if (powerLevel >= 100)
      digit = parseInt(
        powerLevel
          .toString(10)
          .substring(powerLevel.toString(10).length - 3, powerLevel.toString(10).length - 2)
      );
    grid[x - 1][y - 1] = digit - 5;
  }

var max = 0;
var result = "";

for (let x = 1; x <= 298; x++)
  for (let y = 1; y <= 298; y++) {
    for (let i = 1; i + x <= 301 && i + y <= 301; i++) {
      var power = 0;
      for (let k = 0; k < i; k++) for (let j = 0; j < i; j++) power += grid[x - 1 + k][y - 1 + j];
      if (power > max) {
        result = x + "," + y + "," + i;
        max = power;
      }
    }
  }

console.log("The answer is:");
console.log(result);
console.timeEnd("11-part2");
