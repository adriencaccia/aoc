import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const treeMatrix = input
    .split("\n")
    .map((line) => line.split("").map(Number));

  const visibilityMatrix = Array(treeMatrix.length)
    .fill(0)
    .map((_) =>
      Array(treeMatrix[0].length)
        .fill(0)
        .map((_) => false)
    );

  let visibleTrees = 0;

  // left to right
  for (let i = 0; i < treeMatrix.length; i++) {
    let currentTallestTree = 0;
    let currentVisibleTrees = 0;
    for (let j = 0; j < treeMatrix[i].length; j++) {
      const tree = treeMatrix[i][j];
      if (
        tree > currentTallestTree ||
        (currentTallestTree === 0 && currentVisibleTrees === 0)
      ) {
        currentTallestTree = tree;
        if (visibilityMatrix[i][j] === false) {
          visibilityMatrix[i][j] = true;
          currentVisibleTrees++;
        }
      }
    }
    visibleTrees += currentVisibleTrees;
  }

  // right to left
  for (let i = 0; i < treeMatrix.length; i++) {
    let currentTallestTree = 0;
    let currentVisibleTrees = 0;
    for (let j = treeMatrix[i].length - 1; j >= 0; j--) {
      const tree = treeMatrix[i][j];
      if (
        tree > currentTallestTree ||
        (currentTallestTree === 0 && currentVisibleTrees === 0)
      ) {
        currentTallestTree = tree;
        if (visibilityMatrix[i][j] === false) {
          visibilityMatrix[i][j] = true;
          currentVisibleTrees++;
        }
      }
    }
    visibleTrees += currentVisibleTrees;
  }

  // top to bottom
  for (let j = 0; j < treeMatrix[0].length; j++) {
    let currentTallestTree = 0;
    let currentVisibleTrees = 0;
    for (let i = 0; i < treeMatrix.length; i++) {
      const tree = treeMatrix[i][j];
      if (
        tree > currentTallestTree ||
        (currentTallestTree === 0 && currentVisibleTrees === 0)
      ) {
        currentTallestTree = tree;
        if (visibilityMatrix[i][j] === false) {
          visibilityMatrix[i][j] = true;
          currentVisibleTrees++;
        }
      }
    }
    visibleTrees += currentVisibleTrees;
  }

  // bottom to top
  for (let j = 0; j < treeMatrix[0].length; j++) {
    let currentTallestTree = 0;
    let currentVisibleTrees = 0;
    for (let i = treeMatrix.length - 1; i >= 0; i--) {
      const tree = treeMatrix[i][j];
      if (
        tree > currentTallestTree ||
        (currentTallestTree === 0 && currentVisibleTrees === 0)
      ) {
        currentTallestTree = tree;
        if (visibilityMatrix[i][j] === false) {
          visibilityMatrix[i][j] = true;
          currentVisibleTrees++;
        }
      }
    }
    visibleTrees += currentVisibleTrees;
  }

  console.log(visibleTrees);
}

main();

// 1736
