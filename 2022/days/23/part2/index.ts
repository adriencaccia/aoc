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
  const mapSize = board.length * 4;
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

  let newPositions: [[number, number], [number, number]][] = [
    [
      [1000, 2],
      [1000, 2],
    ],
  ];

  let round = 0;
  while (newPositions.length !== 0) {
    newPositions = [];
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

    round++;
  }

  console.log(round);
}

main();

// 978
