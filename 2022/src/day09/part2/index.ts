import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
// import logMatrix from "../../../../utils/logMatrix.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

interface Indices {
  i: number;
  j: number;
}

type Rope = Indices[];

function moveKnot(rope: Rope, knotIndex: number) {
  // head and tail touching
  if (
    Math.abs(rope[knotIndex - 1].i - rope[knotIndex].i) < 2 &&
    Math.abs(rope[knotIndex - 1].j - rope[knotIndex].j) < 2
  ) {
    return;
  }
  // head above or below the tail
  if (
    Math.abs(rope[knotIndex - 1].i - rope[knotIndex].i) == 2 &&
    rope[knotIndex - 1].j === rope[knotIndex].j
  ) {
    rope[knotIndex].i =
      rope[knotIndex].i + (rope[knotIndex - 1].i - rope[knotIndex].i) / 2;
    return;
  }
  // head left or right of tail
  if (
    rope[knotIndex - 1].i === rope[knotIndex].i &&
    Math.abs(rope[knotIndex - 1].j - rope[knotIndex].j) == 2
  ) {
    rope[knotIndex].j =
      rope[knotIndex].j + (rope[knotIndex - 1].j - rope[knotIndex].j) / 2;
    return;
  }
  // tail moves diagonally
  rope[knotIndex].i =
    rope[knotIndex].i + Math.sign(rope[knotIndex - 1].i - rope[knotIndex].i);
  rope[knotIndex].j =
    rope[knotIndex].j + Math.sign(rope[knotIndex - 1].j - rope[knotIndex].j);
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const moves = input.split("\n").map((line) => {
    const [direction, numberOfMoves] = line.split(" ");
    return [direction, Number(numberOfMoves)] as const;
  });

  const gridSize = 300;

  const visitGrid = Array(gridSize)
    .fill(0)
    .map((_) =>
      Array(gridSize)
        .fill(0)
        .map((_) => ".")
    );

  const rope = Array(10)
    .fill(0)
    .map((_) => ({ i: gridSize / 2, j: gridSize / 2 }));

  let visitedPositions = 0;

  for (const [direction, numberOfMoves] of moves) {
    for (let move = 0; move < numberOfMoves; move++) {
      if (direction === "R") {
        rope[0].j += 1;
      }
      if (direction === "L") {
        rope[0].j -= 1;
      }
      if (direction === "U") {
        rope[0].i -= 1;
      }
      if (direction === "D") {
        rope[0].i += 1;
      }
      for (let knotIndex = 1; knotIndex < 10; knotIndex++) {
        moveKnot(rope, knotIndex);
      }

      if (visitGrid[rope.at(-1)?.i ?? 0][rope.at(-1)?.j ?? 0] !== "#") {
        visitedPositions += 1;
      }
      visitGrid[rope.at(-1)?.i ?? 0][rope.at(-1)?.j ?? 0] = "#";
      // console.log(`move ${direction}`);
      // logMatrix(grid);
    }
  }

  // logMatrix(visitGrid);
  console.log(visitedPositions);
}

main();

// 2541
