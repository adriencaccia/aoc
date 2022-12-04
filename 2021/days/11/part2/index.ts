import { readFileSync } from "fs";
import { every, flatten } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function getCleanFlashMatrix(lines: string[]) {
  return lines.map((line) => line.split("").map(() => false));
}

function flash(
  matrix: number[][],
  flashMatrix: boolean[][],
  x: number,
  y: number
) {
  for (let i = x - 1; i <= x + 1; i++) {
    for (let j = y - 1; j <= y + 1; j++) {
      if (i < 0 || i >= matrix.length || j < 0 || j >= matrix[i].length) {
        continue;
      }
      matrix[i][j] += 1;
      if (matrix[i][j] >= 10) {
        if (flashMatrix[i][j]) {
          continue;
        }
        flashMatrix[i][j] = true;
        flash(matrix, flashMatrix, i, j);
      }
    }
  }

  return;
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");
  lines.pop();
  const matrix = lines.map((line) => line.split("").map(Number));

  let answer = 0;

  let step = 1;
  while (answer === 0) {
    const flashMatrix = getCleanFlashMatrix(lines);

    for (let x = 0; x < matrix.length; x++) {
      for (let y = 0; y < matrix[x].length; y++) {
        if (flashMatrix[x][y]) {
          continue;
        }
        matrix[x][y] += 1;
        if (matrix[x][y] >= 10) {
          flashMatrix[x][y] = true;
          flash(matrix, flashMatrix, x, y);
        }
      }
    }

    for (let x = 0; x < matrix.length; x++) {
      for (let y = 0; y < matrix[x].length; y++) {
        if (!flashMatrix[x][y]) {
          continue;
        }
        matrix[x][y] = 0;
      }
    }

    const allFlashed = every(flatten(flashMatrix));

    if (allFlashed) {
      answer = step;
      break;
    }
    step++;
  }

  console.log(answer);
}

main();

// ???
