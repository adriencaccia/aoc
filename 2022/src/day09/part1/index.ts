import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
// import logMatrix from "../../../../utils/logMatrix.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

interface Indices {
  i: number;
  j: number;
}

interface Rope {
  head: Indices;
  tail: Indices;
}

function moveTail(rope: Rope) {
  // head and tail touching
  if (
    Math.abs(rope.head.i - rope.tail.i) < 2 &&
    Math.abs(rope.head.j - rope.tail.j) < 2
  ) {
    return;
  }
  // head above or below the tail
  if (Math.abs(rope.head.i - rope.tail.i) == 2 && rope.head.j === rope.tail.j) {
    rope.tail.i = rope.tail.i + (rope.head.i - rope.tail.i) / 2;
    return;
  }
  // head left or right of tail
  if (rope.head.i === rope.tail.i && Math.abs(rope.head.j - rope.tail.j) == 2) {
    rope.tail.j = rope.tail.j + (rope.head.j - rope.tail.j) / 2;
    return;
  }
  // tail moves diagonally
  rope.tail.i = rope.tail.i + Math.sign(rope.head.i - rope.tail.i);
  rope.tail.j = rope.tail.j + Math.sign(rope.head.j - rope.tail.j);
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

  const rope = {
    head: { i: gridSize / 2, j: gridSize / 2 },
    tail: { i: gridSize / 2, j: gridSize / 2 },
  };

  let visitedPositions = 0;

  for (const [direction, numberOfMoves] of moves) {
    for (let move = 0; move < numberOfMoves; move++) {
      if (direction === "R") {
        rope.head.j += 1;
      }
      if (direction === "L") {
        rope.head.j -= 1;
      }
      if (direction === "U") {
        rope.head.i -= 1;
      }
      if (direction === "D") {
        rope.head.i += 1;
      }
      moveTail(rope);
      if (visitGrid[rope.tail.i][rope.tail.j] !== "#") {
        visitedPositions += 1;
      }
      visitGrid[rope.tail.i][rope.tail.j] = "#";
      // console.log(`move ${direction}`);
      // logMatrix(grid);
    }
  }

  // logMatrix(visitGrid);
  console.log(visitedPositions);
}

main();

// 6339
