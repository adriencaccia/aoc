import { readFileSync } from "fs";
import { join } from "path";

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");
  lines.pop();
  const heightMap = lines.map((line) => line.split("").map(Number));
  const visitedMap = lines.map((line) => line.split("").map(() => false));

  function computeBasinWeight(x: number, y: number) {
    if (visitedMap[x][y]) {
      return 0;
    }
    visitedMap[x][y] = true;

    let basinWeight = 0;

    if (x > 0 && heightMap[x - 1][y] !== 9) {
      basinWeight += computeBasinWeight(x - 1, y);
    }
    if (y > 0 && heightMap[x][y - 1] !== 9) {
      basinWeight += computeBasinWeight(x, y - 1);
    }
    if (x < heightMap.length - 1 && heightMap[x + 1][y] !== 9) {
      basinWeight += computeBasinWeight(x + 1, y);
    }
    if (y < heightMap[x].length - 1 && heightMap[x][y + 1] !== 9) {
      basinWeight += computeBasinWeight(x, y + 1);
    }

    return 1 + basinWeight;
  }

  const basinWeights: number[] = [];
  for (let x = 0; x < heightMap.length; x++) {
    for (let y = 0; y < heightMap[x].length; y++) {
      const current = heightMap[x][y];
      if (current === 9 || visitedMap[x][y]) {
        continue;
      }
      basinWeights.push(computeBasinWeight(x, y));
    }
  }

  const answer = basinWeights
    .sort((a, b) => b - a)
    .slice(0, 3)
    .reduce((a, b) => a * b);
  console.log(answer);
}

main();

// 949905
