import { readFileSync } from "fs";
import { join } from "path";

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");
  lines.pop();
  const matrix = lines.map((line) => line.split("").map(Number));

  for (let y = 0; y < matrix.length; y++) {
    for (let x = 0; x < matrix[y].length; x++) {
      if (x === 0 && y === 0) {
        matrix[y][x] = 0;
        continue;
      }
      const top = y === 0 ? Number.POSITIVE_INFINITY : matrix[y - 1][x];
      const left = x === 0 ? Number.POSITIVE_INFINITY : matrix[y][x - 1];
      matrix[y][x] += Math.min(top, left);
    }
  }

  const answer = matrix[matrix.length - 1][matrix[0].length - 1];
  console.log(answer);
}

main();

// 592
