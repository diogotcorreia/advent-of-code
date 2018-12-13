console.time("13-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt").split("\n");

// Track types:
// 0 -> Horizontal (-)
// 1 -> Vertical (|)
// 2 -> 90 DEG SOUTH EAST or 90 DEG NORTH WEST (/)
// 3 -> 90 DEG SOUTH WEST or 90 DEG NORTH EAST (\)
// 4 -> Intersection (+)
var track = [];
// Cart direction:
// 0 -> NORTH
// 1 -> EAST
// 2 -> SOUTH
// 3 -> WEST
var carts = [];

for (let i = 0; i < input.length; i++) {
  let line = input[i];
  if (line.length === 0) continue;
  track[i] = [];
  for (let k = 0; k < line.length; k++) {
    let char = line.charAt(k);
    if (char === "-") {
      track[i][k] = 0;
    } else if (char === "|") {
      track[i][k] = 1;
    } else if (char === "/") {
      track[i][k] = 2;
    } else if (char === "\\") {
      track[i][k] = 3;
    } else if (char === "+") {
      track[i][k] = 4;
    } else if (char === "^") {
      carts.push({ x: k, y: i, direction: 0, state: 0 });
      track[i][k] = 1;
    } else if (char === ">") {
      carts.push({ x: k, y: i, direction: 1, state: 0 });
      track[i][k] = 0;
    } else if (char === "v") {
      carts.push({ x: k, y: i, direction: 2, state: 0 });
      track[i][k] = 1;
    } else if (char === "<") {
      carts.push({ x: k, y: i, direction: 3, state: 0 });
      track[i][k] = 0;
    }
  }
}

const isCart = (x, y) => {
  for (let cart of carts) if (cart.x === x && cart.y === y) return true;
  return false;
};

const comparator = (a, b) => {
  if (a.y === b.y) return a.x - b.x;
  return a.y - b.y;
};

whileLoop: while (true) {
  carts = carts.sort(comparator);
  for (let cart of carts) {
    if (cart.direction === 0) {
      if (isCart(cart.x, cart.y - 1)) {
        var result = cart.x + "," + (cart.y - 1);
        break whileLoop;
      }
      cart.y--;
      let tile = track[cart.y][cart.x];
      if (tile === 2) {
        // 90 DEG (/)
        cart.direction = 1; // EAST
      } else if (tile === 3) {
        // 90 DEG (\)
        cart.direction = 3; // WEST
      } else if (tile === 4) {
        // Intersection (+)
        if (cart.state === 0) {
          cart.direction = 3; // WEST
          cart.state++;
        } else if (cart.state === 1) {
          // Keep direction
          cart.state++;
        } else if (cart.state === 2) {
          cart.direction = 1; // EAST
          cart.state = 0;
        } else console.error("Invalid cart state: " + cart.state);
      } else if (tile !== 1) console.error("Invalid tile for NORTH direction: " + tile);
    } else if (cart.direction === 1) {
      if (isCart(cart.x + 1, cart.y)) {
        var result = cart.x + 1 + "," + cart.y;
        break whileLoop;
      }
      cart.x++;
      let tile = track[cart.y][cart.x];
      if (tile === 2) {
        // 90 DEG (/)
        cart.direction = 0; // NORTH
      } else if (tile === 3) {
        // 90 DEG (\)
        cart.direction = 2; // SOUTH
      } else if (tile === 4) {
        // Intersection (+)
        if (cart.state === 0) {
          cart.direction = 0; // NORTH
          cart.state++;
        } else if (cart.state === 1) {
          // Keep direction
          cart.state++;
        } else if (cart.state === 2) {
          cart.direction = 2; // SOUTH
          cart.state = 0;
        } else console.error("Invalid cart state: " + cart.state);
      } else if (tile !== 0) console.error("Invalid tile for EAST direction: " + tile);
    } else if (cart.direction === 2) {
      if (isCart(cart.x, cart.y + 1)) {
        var result = cart.x + "," + (cart.y + 1);
        break whileLoop;
      }
      cart.y++;
      let tile = track[cart.y][cart.x];
      if (tile === 2) {
        // 90 DEG (/)
        cart.direction = 3; // WEST
      } else if (tile === 3) {
        // 90 DEG (\)
        cart.direction = 1; // EAST
      } else if (tile === 4) {
        // Intersection (+)
        if (cart.state === 0) {
          cart.direction = 1; // EAST
          cart.state++;
        } else if (cart.state === 1) {
          // Keep direction
          cart.state++;
        } else if (cart.state === 2) {
          cart.direction = 3; // WEST
          cart.state = 0;
        } else console.error("Invalid cart state: " + cart.state);
      } else if (tile !== 1) console.error("Invalid tile for SOUTH direction: " + tile);
    } else if (cart.direction === 3) {
      if (isCart(cart.x - 1, cart.y)) {
        var result = cart.x - 1 + "," + cart.y;
        break whileLoop;
      }
      cart.x--;
      let tile = track[cart.y][cart.x];
      if (tile === 2) {
        // 90 DEG (/)
        cart.direction = 2; // SOUTH
      } else if (tile === 3) {
        // 90 DEG (\)
        cart.direction = 0; // NORTH
      } else if (tile === 4) {
        // Intersection (+)
        if (cart.state === 0) {
          cart.direction = 2; // SOUTH
          cart.state++;
        } else if (cart.state === 1) {
          // Keep direction
          cart.state++;
        } else if (cart.state === 2) {
          cart.direction = 0; // NORTH
          cart.state = 0;
        } else console.error("Invalid cart state: " + cart.state);
      } else if (tile !== 0) console.error("Invalid tile for WEST direction: " + tile);
    } else console.error("Invalid cart direction: " + cart.direction);
  }
}

console.log("The answer is:");
console.log(result);
console.timeEnd("13-part1");
