import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
// import logMatrix from "../../../../utils/logMatrix.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const paths = input
    .split("\n")
    .map((path) =>
      path.split(" -> ").map((line) => line.split(",").map(Number))
    );

  const gridSize = 1000;
  const offset = 0;
  const cave = Array(gridSize)
    .fill(0)
    .map((_) =>
      Array(gridSize)
        .fill(0)
        .map((_) => "⬛")
    );

  for (const path of paths) {
    for (let pointIndex = 0; pointIndex < path.length - 1; pointIndex++) {
      const [jStart, iStart] = path[pointIndex];
      const [jEnd, iEnd] = path[pointIndex + 1];
      if (iStart === iEnd) {
        if (jEnd > jStart) {
          for (let j = jStart; j <= jEnd; j++) {
            cave[iStart][j - offset] = "⬜";
          }
          continue;
        }
        for (let j = jEnd; j <= jStart; j++) {
          cave[iStart][j - offset] = "⬜";
        }
        continue;
      }
      if (iEnd > iStart) {
        for (let i = iStart; i <= iEnd; i++) {
          cave[i][jStart - offset] = "⬜";
        }
        continue;
      }

      for (let i = iEnd; i <= iStart; i++) {
        cave[i][jStart - offset] = "⬜";
      }
      continue;
    }
  }
  // logMatrix(cave);

  let unitsOfSand = 0;
  let depth = 0;
  while (depth < gridSize) {
    let blocked = false;
    let jCurrent = 500 - offset;
    depth = 0;
    while (blocked === false && depth < gridSize) {
      if (cave[depth + 1] === undefined) {
        depth = gridSize * 2;
        break;
      }
      if (cave[depth + 1][jCurrent] === "⬛") {
        depth++;
        continue;
      }
      // sand is blocked
      if (["🟧", "⬜"].includes(cave[depth + 1][jCurrent])) {
        if (["🟧", "⬜"].includes(cave[depth + 1][jCurrent - 1])) {
          if (["🟧", "⬜"].includes(cave[depth + 1][jCurrent + 1])) {
            cave[depth][jCurrent] = "🟧";
            blocked = true;
            continue;
          }
        }
      }

      // sand can go left
      if (["🟧", "⬜"].includes(cave[depth + 1][jCurrent])) {
        if (cave[depth + 1][jCurrent - 1] === "⬛") {
          jCurrent -= 1;
          continue;
        }
      }

      // sand can go right
      if (["🟧", "⬜"].includes(cave[depth + 1][jCurrent])) {
        if (["🟧", "⬜"].includes(cave[depth + 1][jCurrent - 1])) {
          jCurrent += 1;
          continue;
        }
      }

      depth++;
    }

    unitsOfSand++;
  }

  // logMatrix(cave);

  console.log(unitsOfSand - 1);
}

main();

// 737
