console.time("14-part2");
var input = parseInt(require("../../utils/readTxt")(__dirname + "/input.txt").trim())
  .toString()
  .split("")
  .map((i) => parseInt(i));

var firstRecipe;
var currentRecipe1;
var currentRecipe2;

var inputProgress = 0;

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
  return newRecipe;
};

const moveElf = (recipe) => {
  let toMove = recipe.score + 1;
  for (let i = 0; i < toMove; i++) recipe = recipe.next;
  return recipe;
};

const makeRecipe = () => {
  var newRecipes = [];
  for (let newRecipe of (currentRecipe1.score + currentRecipe2.score)
    .toString()
    .split("")
    .map((i) => parseInt(i)))
    newRecipes.push(addRecipe(newRecipe));
  currentRecipe1 = moveElf(currentRecipe1);
  currentRecipe2 = moveElf(currentRecipe2);
  return newRecipes;
};

whileLoop: while (true) {
  var newRecipes = makeRecipe();
  for (let recipe of newRecipes) {
    if (recipe.score !== input[inputProgress]) {
      inputProgress = 0;
      continue;
    }
    inputProgress++;
    if (inputProgress === input.length) {
      var targetRecipe = recipe;
      break whileLoop;
    }
  }
}

for (let i = 0; i < input.length - 1; i++) targetRecipe = targetRecipe.prev;

var result = 0;
while (targetRecipe !== firstRecipe) {
  result++;
  targetRecipe = targetRecipe.prev;
}

console.log("The answer is:");
console.log(result);
console.timeEnd("14-part2");
