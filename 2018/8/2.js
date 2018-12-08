console.time("8-part1");
var input = require("../../utils/readTxt")(__dirname + "/input.txt")
  .split("\n")[0]
  .trim()
  .split(" ")
  .map((i) => parseInt(i));

const addMetadata = (path, metadata) => {
  var target = tree;
  for (let i of path) {
    if (!target.children) target.children = [];
    if (!target.children[i]) target.children[i] = {};
    target = target.children[i];
  }
  if (!target.metadata) target.metadata = [];
  target.metadata.push(metadata);
};

var tree = {};

const parseNode = (start, nodeCount, path) => {
  if (nodeCount === 0) return 0;
  var metadataRemaining = -1;
  var childId = 0;
  for (let i = start; i < input.length; i++) {
    let value = input[i];
    if (metadataRemaining === -1) {
      metadataRemaining = input[i + 1];
      i += parseNode(i + 2, value, [...path, childId]);
      i++;
      continue;
    }
    addMetadata([...path, childId], value);
    metadataRemaining--;
    if (metadataRemaining === 0) {
      nodeCount--;
      childId++;
      if (nodeCount === 0) return i - start + 1;
      else metadataRemaining = -1;
    }
  }
};

parseNode(0, 1, []);

const getNodeValue = (node) => {
  if (node === undefined) return 0;
  let value = 0;
  if (!node.children || node.children.length === 0) {
    for (let metadata of node.metadata || []) value += metadata;
  } else {
    for (let metadata of node.metadata || []) {
      value += getNodeValue(node.children[metadata - 1]);
    }
  }
  return value;
};

var result = getNodeValue(tree.children[0]);

console.log("The answer is:");
console.log(result);
console.timeEnd("8-part1");
