console.time("9-part2");
var input = require("../../utils/readTxt")(__dirname + "/input.txt")
  .split("\n")[0]
  .trim();

var [, totalPlayers, lastMarble] = /(\d+) players; last marble is worth (\d+) points/.exec(input);

totalPlayers = parseInt(totalPlayers);
lastMarble = parseInt(lastMarble) * 100;

const addAfter = (value, marble) => {
  const toAdd = {
    value,
    prev: marble,
    next: marble.next,
  };
  marble.next.prev = toAdd;
  marble.next = toAdd;
  return toAdd;
};

const getNextPlayer = (currentPlayer) => {
  var result = currentPlayer + 1;
  if (result >= totalPlayers) result = 0;
  return result;
};

var playersScore = Array(totalPlayers).fill(0);

var currentPlayer = 0;

let current = {
  value: 0,
};
current.next = current;
current.prev = current;

for (var i = 1; i <= lastMarble; i++) {
  if (i % 23 === 0) {
    playersScore[currentPlayer] += i;
    current = current.prev.prev.prev.prev.prev.prev;
    playersScore[currentPlayer] += current.prev.value;
    current.prev.prev.next = current;
    current.prev = current.prev.prev;
  } else {
    current = addAfter(i, current.next);
  }
  currentPlayer = getNextPlayer(currentPlayer);
}

var result = Math.max(...playersScore);

console.log("The answer is:");
console.log(result);
console.timeEnd("9-part2");
