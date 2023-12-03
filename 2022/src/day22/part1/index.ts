import { readFileSync } from "fs";
import { clone } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function logBoard(board: string[][]) {
  for (const line of board) {
    console.log(line.join(""));
  }
}

type Turn = "L" | "R";
type Direction = "👉" | "👇" | "👈" | "👆";
type BoardItem = "⬜" | "🟧" | "⬛" | Direction;

function getCurrentDirection(
  previousDirection: Direction,
  turn: Turn
): Direction {
  switch (previousDirection) {
    case "👆":
      return turn === "L" ? "👈" : "👉";
    case "👉":
      return turn === "L" ? "👆" : "👇";
    case "👇":
      return turn === "L" ? "👉" : "👈";
    case "👈":
      return turn === "L" ? "👇" : "👆";
  }
}

function getFacingScore(direction: BoardItem) {
  switch (direction) {
    case "👉":
      return 0;
    case "👇":
      return 1;
    case "👈":
      return 2;
    case "👆":
      return 3;
    default:
      return Number.MAX_SAFE_INTEGER;
  }
}

function prepareInput(input: string) {
  const [boardString, instructionsString] = input.split("\n\n");
  const lines = boardString.split("\n");
  const boardWidth = Math.max(...lines.map((line) => line.length));
  const board: BoardItem[][] = lines
    .map((line) => line.padEnd(boardWidth).split(""))
    .map((line) =>
      line
        .map((item) => {
          switch (item) {
            case ".":
              return "⬜";
            case "#":
              return "🟧";
            default:
              return "⬛";
          }
        })
        .fill("⬛", line.length, boardWidth)
    );
  let instructions = (instructionsString.match(/(R|L)\d+/g) ?? []).map(
    (instruction) =>
      [instruction[0] as Turn, Number(instruction.slice(1))] as const
  );
  instructions.unshift(["L", Number(instructionsString.match(/\d+/)?.[0])]);

  const directions: [Direction, number][] = [["👉", instructions[0][1]]];

  for (let i = 1; i < instructions.length; i++) {
    const [previousDirection] = directions[i - 1];
    const [turn, length] = instructions[i];
    directions.push([getCurrentDirection(previousDirection, turn), length]);
  }

  return { board, directions };
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const { board, directions } = prepareInput(input);

  let x = 0;
  let y = board[0].findIndex((item) => item === "⬜");
  board[x][y] = "👉";

  function move(direction: Direction) {
    let newX = x;
    let newY = y;
    if (direction === "👉") {
      newY += 1;
      if (newY === board[0].length || board[x][newY] === "⬛") {
        newY = board[x].findIndex((item) => item !== "⬛");
      }
    }
    if (direction === "👇") {
      newX += 1;
      if (newX === board.length || board[newX][y] === "⬛") {
        newX = board.map((line) => line[y]).findIndex((item) => item !== "⬛");
      }
    }
    if (direction === "👈") {
      newY -= 1;
      if (newY === -1 || board[x][newY] === "⬛") {
        newY =
          board[0].length -
          1 -
          clone(board[x])
            .reverse()
            .findIndex((item) => item !== "⬛");
      }
    }
    if (direction === "👆") {
      newX -= 1;
      if (newX === -1 || board[newX][y] === "⬛") {
        newX =
          board.length -
          1 -
          clone(board.map((line) => line[y]))
            .reverse()
            .findIndex((item) => item !== "⬛");
      }
    }
    if (board[newX][newY] === "🟧") {
      return undefined;
    }
    return { newX, newY };
  }

  for (let i = 0; i < directions.length; i++) {
    const [direction, length] = directions[i];
    for (let moveIndex = 0; moveIndex < length; moveIndex++) {
      board[x][y] = direction;
      const moveResult = move(direction);
      if (moveResult === undefined) {
        break;
      }
      x = moveResult.newX;
      y = moveResult.newY;
      board[x][y] = direction;
    }
  }

  // logBoard(board);
  const answer = 1000 * (x + 1) + 4 * (y + 1) + getFacingScore(board[x][y]);
  console.log(answer);
}

main();

// 26558
