console.time("14-part1");
var input = parseInt(require("../../utils/readTxt")(__dirname + "/input.txt").trim());

var firstRecipe;
var currentRecipe1;
var currentRecipe2;

var recipeAmount = 2;

const initialize = () => {
  currentRecipe1 = firstRecipe = { score: 3 };
  currentRecipe2 = { score: 7, prev: currentRecipe1, next: currentRecipe1 };
  currentRecipe1.prev = currentRecipe2;
  currentRecipe1.next = currentRecipe2;
};

initialize();

const addRecipe = (score) => {
  let newRecipe = { score, prev: firstRecipe.prev, next: firstRecipe };
  firstRecipe.prev.next = newRecipe;
  firstRecipe.prev = newRecipe;
  recipeAmount++;
};

const moveElf = (recipe) => {
  let toMove = recipe.score + 1;
  for (let i = 0; i < toMove; i++) recipe = recipe.next;
  return recipe;
};

const makeRecipe = () => {
  for (let newRecipe of (currentRecipe1.score + currentRecipe2.score)
    .toString()
    .split("")
    .map((i) => parseInt(i)))
    addRecipe(newRecipe);
  currentRecipe1 = moveElf(currentRecipe1);
  currentRecipe2 = moveElf(currentRecipe2);
};

var result = "";

while (recipeAmount < input + 10) {
  makeRecipe();
}

var resultRecipe = firstRecipe;
for (let i = 0; i < input; i++) resultRecipe = resultRecipe.next;
for (let i = 0; i < 10; i++) {
  result += resultRecipe.score;
  resultRecipe = resultRecipe.next;
}

console.log("The answer is:");
console.log(result);
console.timeEnd("14-part1");
