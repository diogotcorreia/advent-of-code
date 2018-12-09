console.time("9-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt")
  .split("\n")[0]
  .trim();

var [, totalPlayers, lastMarble] = /(\d+) players; last marble is worth (\d+) points/.exec(input);

totalPlayers = parseInt(totalPlayers);
lastMarble = parseInt(lastMarble);

// Vector: positive for clockwise movement, negative for counter-clockwise movement
const getPosition = (currentPosition, vector, boardSize) => {
  var result = currentPosition + vector;
  while (result < 0) result += boardSize;
  while (result > boardSize) result -= boardSize;
  return result;
};

const getNextPlayer = (currentPlayer) => {
  var result = currentPlayer + 1;
  if (result >= totalPlayers) result = 0;
  return result;
};

var playersScore = Array(totalPlayers).fill(0);

var gameBoard = [0];
var currentPosition = 0;
var currentPlayer = 0;

for (var i = 1; i <= lastMarble; i++) {
  if (i % 23 === 0) {
    playersScore[currentPlayer] += i;
    var removePos = getPosition(currentPosition, -7, gameBoard.length);
    playersScore[currentPlayer] += gameBoard[removePos];
    gameBoard.splice(removePos, 1);
    currentPosition = removePos;
  } else {
    var position = getPosition(currentPosition, 2, gameBoard.length);
    gameBoard.splice(position, 0, i);
    currentPosition = position;
  }
  currentPlayer = getNextPlayer(currentPlayer);
}

var result = Math.max(...playersScore);

console.log("The answer is:");
console.log(result);
console.timeEnd("9-part1");
