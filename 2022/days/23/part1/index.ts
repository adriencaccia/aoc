import { readFileSync } from "fs";
import { uniq } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function logMatrix(matrix: string[][]) {
  for (const line of matrix) {
    console.log(line.join(""));
  }
}

function prepareInput(input: string) {
  const board = input
    .split("\n")
    .map((line) =>
      line.replaceAll(/\./g, "⬜").replaceAll(/#/g, "⬛").split("")
    );
  const mapSize = board.length * 2;
  const map = Array.from({ length: mapSize }, () =>
    Array.from({ length: mapSize }, () => "⬜")
  );

  const offset = Math.floor(board.length / 2);
  for (let i = 0; i < board.length; i++) {
    for (let j = 0; j < board.length; j++) {
      map[i + offset][j + offset] = board[i][j];
    }
  }
  return map;
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const map = prepareInput(input);
  let directions = ["N", "S", "W", "E"];

  let newPositions: [[number, number], [number, number]][] = [];

  // console.log(`Initial state`);
  // logMatrix(map);

  const rounds = 10;
  for (let round = 1; round <= rounds; round++) {
    for (let i = 1; i < map.length - 1; i++) {
      for (let j = 1; j < map.length - 1; j++) {
        if (
          map[i][j] === "⬜" ||
          (map[i - 1][j - 1] === "⬜" &&
            map[i - 1][j] === "⬜" &&
            map[i - 1][j + 1] === "⬜" &&
            map[i][j - 1] === "⬜" &&
            map[i][j + 1] === "⬜" &&
            map[i + 1][j - 1] === "⬜" &&
            map[i + 1][j] === "⬜" &&
            map[i + 1][j + 1] === "⬜")
        ) {
          // do not move
          continue;
        }
        for (const direction of directions) {
          if (
            direction === "N" &&
            map[i - 1][j - 1] === "⬜" &&
            map[i - 1][j] === "⬜" &&
            map[i - 1][j + 1] === "⬜"
          ) {
            // move north
            newPositions.push([
              [i, j],
              [i - 1, j],
            ]);
            break;
          }
          if (
            direction === "S" &&
            map[i + 1][j - 1] === "⬜" &&
            map[i + 1][j] === "⬜" &&
            map[i + 1][j + 1] === "⬜"
          ) {
            // move south
            newPositions.push([
              [i, j],
              [i + 1, j],
            ]);
            break;
          }
          if (
            direction === "W" &&
            map[i - 1][j - 1] === "⬜" &&
            map[i][j - 1] === "⬜" &&
            map[i + 1][j - 1] === "⬜"
          ) {
            // move west
            newPositions.push([
              [i, j],
              [i, j - 1],
            ]);
            break;
          }
          if (
            direction === "E" &&
            map[i - 1][j + 1] === "⬜" &&
            map[i][j + 1] === "⬜" &&
            map[i + 1][j + 1] === "⬜"
          ) {
            // move east
            newPositions.push([
              [i, j],
              [i, j + 1],
            ]);
            break;
          }
        }
      }
    }

    const elvesToMove = new Set<number>();
    const duplicates = new Set<number>();

    for (const positions of newPositions) {
      const value = positions[1][0] * 1000 + positions[1][1];
      if (elvesToMove.has(value)) {
        duplicates.add(value);
        continue;
      }
      elvesToMove.add(value);
    }

    for (const [currentPosition, newPosition] of newPositions) {
      if (duplicates.has(newPosition[0] * 1000 + newPosition[1])) {
        continue;
      }
      map[currentPosition[0]][currentPosition[1]] = "⬜";
      map[newPosition[0]][newPosition[1]] = "⬛";
    }

    const [elementToCycle] = directions.splice(0, 1);
    directions.push(elementToCycle);

    newPositions = [];
    // console.log(`End of round ${round}`);
    // logMatrix(map);
  }

  let minI = Number.MAX_SAFE_INTEGER;
  let minJ = Number.MAX_SAFE_INTEGER;
  let maxI = 0;
  let maxJ = 0;
  for (let i = 1; i < map.length - 1; i++) {
    for (let j = 1; j < map.length - 1; j++) {
      if (map[i][j] !== "⬛") {
        continue;
      }
      if (i < minI) {
        minI = i;
      }
      if (j < minJ) {
        minJ = j;
      }
      if (maxI < i) {
        maxI = i;
      }
      if (maxJ < j) {
        maxJ = j;
      }
    }
  }

  let emptyTiles = 0;
  for (let i = minI; i <= maxI; i++) {
    for (let j = minJ; j <= maxJ; j++) {
      if (map[i][j] === "⬛") {
        continue;
      }
      emptyTiles++;
    }
  }
  console.log(emptyTiles);
}

main();

// 4052
