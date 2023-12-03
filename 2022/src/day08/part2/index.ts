import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function calculateScenicScore(
  treeMatrix: number[][],
  iTree: number,
  jTree: number
) {
  const tree = treeMatrix[iTree][jTree];
  // left viewing distance
  let leftViewingDistance = 0;
  for (let i = iTree - 1; i >= 0; i--) {
    const currentTree = treeMatrix[i][jTree];
    if (currentTree < tree) {
      leftViewingDistance += 1;
      continue;
    }
    if (currentTree === tree) {
      leftViewingDistance += 1;
      break;
    }
    break;
  }
  // right viewing distance
  let rightViewingDistance = 0;
  for (let i = iTree + 1; i < treeMatrix.length; i++) {
    const currentTree = treeMatrix[i][jTree];
    if (currentTree < tree) {
      rightViewingDistance += 1;
      continue;
    }
    if (currentTree === tree) {
      rightViewingDistance += 1;
      break;
    }
    break;
  }
  // top viewing distance
  let topViewingDistance = 0;
  for (let j = jTree - 1; j >= 0; j--) {
    const currentTree = treeMatrix[iTree][j];
    if (currentTree < tree) {
      topViewingDistance += 1;
      continue;
    }
    if (currentTree === tree) {
      topViewingDistance += 1;
      break;
    }
    break;
  }

  // bottom viewing distance
  let bottomViewingDistance = 0;
  for (let j = jTree + 1; j < treeMatrix.length; j++) {
    const currentTree = treeMatrix[iTree][j];
    if (currentTree < tree) {
      bottomViewingDistance += 1;
      continue;
    }
    if (currentTree === tree) {
      bottomViewingDistance += 1;
      break;
    }
    break;
  }

  return (
    leftViewingDistance *
    rightViewingDistance *
    topViewingDistance *
    bottomViewingDistance
  );
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const treeMatrix = input
    .split("\n")
    .map((line) => line.split("").map(Number));

  let highestScenicScore = 0;

  for (let i = 1; i < treeMatrix.length - 1; i++) {
    for (let j = 1; j < treeMatrix[i].length - 1; j++) {
      const scenicScore = calculateScenicScore(treeMatrix, i, j);
      if (scenicScore > highestScenicScore) {
        highestScenicScore = scenicScore;
      }
    }
  }

  console.log(highestScenicScore);
}

main();

// 268800
