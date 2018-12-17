console.time("15-part2");
var input = require("../../utils/readTxt")(__dirname + "/input.txt")
  .split("\n")
  .map((a) => a.trim());

Object.filter = (obj, predicate) =>
  Object.keys(obj)
    .filter((key) => predicate(obj[key]))
    .reduce((res, key) => ((res[key] = obj[key]), res), {});

const DPH = 3; // Damage Per Hit
const START_HP = 200;

var air = [],
  units = [];

for (let y = 0; y < input.length; y++) {
  let line = input[y];
  if (line.length === 0) continue;
  let chars = line.split("");
  for (let x = 0; x < chars.length; x++) {
    let char = chars[x];
    if (char !== "#") air.push(x + "," + y);
    if (char === "E" || char === "G")
      units.push({
        type: char === "E", // true for Elf, false for Goblin
        x,
        y,
        hp: START_HP,
      });
  }
}

var defaultUnits = JSON.stringify(units);

const compare = (a, b) => {
  if (a.y === b.y) return a.x - b.x;
  return a.y - b.y;
};

const getInRange = (units, type) => {
  var result = [];
  for (let unit of units) {
    if (type !== unit.type || unit.hp <= 0) continue;
    if (isEmpty(unit.x, unit.y - 1)) result.push({ x: unit.x, y: unit.y - 1 });
    if (isEmpty(unit.x - 1, unit.y)) result.push({ x: unit.x - 1, y: unit.y });
    if (isEmpty(unit.x + 1, unit.y)) result.push({ x: unit.x + 1, y: unit.y });
    if (isEmpty(unit.x, unit.y + 1)) result.push({ x: unit.x, y: unit.y + 1 });
  }
  return result;
};

const isEmpty = (x, y, startX, startY) => {
  if (x === startX && y === startY) return true;
  if (air.indexOf(x + "," + y) === -1) return false;
  for (let unit of units) if (unit.hp > 0 && x === unit.x && y === unit.y) return false;
  return true;
};

const getPossibleSteps = (x, y, startX, startY) => {
  var result = [];
  if (isEmpty(x, y - 1, startX, startY)) result.push(x + "," + (y - 1));
  if (isEmpty(x - 1, y, startX, startY)) result.push(x - 1 + "," + y);
  if (isEmpty(x + 1, y, startX, startY)) result.push(x + 1 + "," + y);
  if (isEmpty(x, y + 1, startX, startY)) result.push(x + "," + (y + 1));
  return result;
};

const getNearestPath = (startX, startY, finishX, finishY) => {
  var map = {};
  map[finishX + "," + finishY] = 0;
  for (var move = 1; map[startX + "," + startY] === undefined; move++) {
    var targets = Object.keys(Object.filter(map, (score) => score === move - 1));
    if (targets.length === 0) {
      move++;
      break;
    }
    for (let target of targets) {
      let possible = getPossibleSteps(...target.split(",").map((i) => parseInt(i)), startX, startY);
      for (let step of possible) if (map[step] === undefined) map[step] = move;
    }
  }
  if (map[startX + "," + (startY - 1)] === move - 2) var tile = { x: startX, y: startY - 1 };
  else if (map[startX - 1 + "," + startY] === move - 2) var tile = { x: startX - 1, y: startY };
  else if (map[startX + 1 + "," + startY] === move - 2) var tile = { x: startX + 1, y: startY };
  else if (map[startX + "," + (startY + 1)] === move - 2) var tile = { x: startX, y: startY + 1 };
  else var tile = false;
  return {
    moves: move - 2,
    tile,
  };
};

const getOpponent = (x, y, type) => {
  let opponents = [];
  for (let i = 0; i < units.length; i++) {
    let unit = units[i];
    if (
      Math.pow(unit.x - x, 2) + Math.pow(unit.y - y, 2) === 1 &&
      unit.type === type &&
      unit.hp > 0
    )
      opponents.push(unit);
  }
  if (opponents.length === 0) return;
  var minHitpoints = Math.min(...opponents.map((unit) => unit.hp));
  return opponents.sort(compare).find((unit) => unit.hp === minHitpoints);
};

const runRound = (ELF_DPH) => {
  var sortedUnits = units.sort(compare);
  for (let unit of sortedUnits) {
    if (unit.hp <= 0) continue;
    if (hasFinished()) return false;
    let opponent = getOpponent(unit.x, unit.y, !unit.type);
    if (!opponent) {
      let possibleTiles = getInRange(sortedUnits, !unit.type);
      let minMoves = Number.MAX_SAFE_INTEGER;
      let tile;
      let targetTile;
      for (let possibleTile of possibleTiles) {
        var nearestPath = getNearestPath(unit.x, unit.y, possibleTile.x, possibleTile.y);
        if (nearestPath.tile === false) continue;
        if (nearestPath.moves < minMoves) {
          minMoves = nearestPath.moves;
          tile = nearestPath.tile;
          targetTile = possibleTile;
        } else if (nearestPath.moves === minMoves)
          if (compare(possibleTile, targetTile) < 0) {
            tile = nearestPath.tile;
            targetTile = possibleTile;
          }
      }
      if (tile) {
        unit.x = tile.x;
        unit.y = tile.y;
        opponent = getOpponent(unit.x, unit.y, !unit.type);
      }
    }
    if (opponent) opponent.hp -= unit.type ? ELF_DPH : DPH;
  }
  return true;
};

const hasFinished = () => {
  var firstType;
  for (var i = 0; i < units.length; i++) {
    let unit = units[i];
    if (unit.hp <= 0) continue;
    if (firstType === undefined) firstType = unit.type;
    else if (firstType !== unit.type) return false;
  }
  return true;
};

const removeDeadUnits = () => {
  var elfDied = false;
  for (var i = 0; i < units.length; i++) {
    let unit = units[i];
    if (unit.hp <= 0) {
      if (unit.type === true) elfDied = true;
      units.splice(i, 1);
      i--;
    }
  }
  return elfDied;
};

var round = 0;

const runGame = (ELF_DPH) => {
  units = JSON.parse(defaultUnits);
  round = 0;
  while (runRound(ELF_DPH)) {
    if (removeDeadUnits() === true) return false;
    round++;
  }
  return !removeDeadUnits();
};

var ELF_DPH = 3;
while (!runGame(ELF_DPH)) ELF_DPH++;

var remainingHP = 0;
for (let unit of units) remainingHP += unit.hp;

var result = round * remainingHP;

console.log("The answer is:");
console.log(result);
console.timeEnd("15-part2");
