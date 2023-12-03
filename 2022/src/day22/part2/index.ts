import { readFileSync } from "fs";
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

  return { board, instructions };
}

function getFaceNumber(x: number, y: number) {
  if (0 <= x && x < 50 && 50 <= y && y < 100) {
    return 1;
  }
  if (0 <= x && x < 50 && 100 <= y && y < 150) {
    return 2;
  }
  if (50 <= x && x < 100 && 50 <= y && y < 100) {
    return 3;
  }
  if (100 <= x && x < 150 && 0 <= y && y < 50) {
    return 4;
  }
  if (100 <= x && x < 150 && 50 <= y && y < 100) {
    return 5;
  }
  if (150 <= x && x < 200 && 0 <= y && y < 50) {
    return 6;
  }
  throw new Error(`x: ${x}, y: ${y} out of bounds`);
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const { board, instructions } = prepareInput(input);

  let x = 0;
  let y = board[0].findIndex((item) => item === "⬜");

  function move(
    x: number,
    y: number,
    direction: Direction
  ):
    | {
        newX: number;
        newY: number;
        newDirection: Direction;
      }
    | undefined {
    const { newX, newY, newDirection } = getNewTile(x, y, direction);
    if (board[newX][newY] === "🟧") {
      return undefined;
    }
    return { newX, newY, newDirection };
  }

  function getNewTile(
    x: number,
    y: number,
    direction: Direction
  ): {
    newX: number;
    newY: number;
    newDirection: Direction;
  } {
    const faceNumber = getFaceNumber(x, y);

    let newX = x;
    let newY = y;
    if (direction === "👉") {
      newY += 1;
      if (newY === board[0].length || board[newX][newY] === "⬛") {
        if (faceNumber === 2) {
          return { newX: 149 - newX, newY: 99, newDirection: "👈" };
        }
        if (faceNumber === 3) {
          return { newX: 49, newY: newX + 50, newDirection: "👆" };
        }
        if (faceNumber === 5) {
          return { newX: 149 - newX, newY: 149, newDirection: "👈" };
        }
        if (faceNumber === 6) {
          return { newX: 149, newY: newX - 100, newDirection: "👆" };
        }
      }
    }
    if (direction === "👇") {
      newX += 1;
      if (newX === board.length || board[newX][newY] === "⬛") {
        if (faceNumber === 2) {
          return { newX: newY - 50, newY: 99, newDirection: "👈" };
        }
        if (faceNumber === 5) {
          return { newX: newY + 100, newY: 49, newDirection: "👈" };
        }
        if (faceNumber === 6) {
          return { newX: 0, newY: newY + 100, newDirection: "👇" };
        }
      }
    }
    if (direction === "👈") {
      newY -= 1;
      if (newY === -1 || board[newX][newY] === "⬛") {
        if (faceNumber === 1) {
          return { newX: 149 - newX, newY: 0, newDirection: "👉" };
        }
        if (faceNumber === 3) {
          return { newX: 100, newY: newX - 50, newDirection: "👇" };
        }
        if (faceNumber === 4) {
          return { newX: 149 - newX, newY: 50, newDirection: "👉" };
        }
        if (faceNumber === 6) {
          return { newX: 0, newY: newX - 100, newDirection: "👇" };
        }
      }
    }
    if (direction === "👆") {
      newX -= 1;
      if (newX === -1 || board[newX][newY] === "⬛") {
        if (faceNumber === 1) {
          return { newX: 100 + newY, newY: 0, newDirection: "👉" };
        }
        if (faceNumber === 2) {
          return { newX: 199, newY: newY - 100, newDirection: "👆" };
        }
        if (faceNumber === 4) {
          return { newX: newY + 50, newY: 50, newDirection: "👉" };
        }
      }
    }
    return { newX, newY, newDirection: direction };
  }

  // first turn is left, so it will start going right with direction down
  let direction: Direction = "👇";
  let counter = 0;
  for (let i = 0; i < instructions.length; i++) {
    let [turn, length] = instructions[i];
    direction = getCurrentDirection(direction, turn);
    for (let moveIndex = 0; moveIndex < length; moveIndex++) {
      board[x][y] = direction;
      const moveResult = move(x, y, direction);
      if (moveResult === undefined) {
        break;
      }
      x = moveResult.newX;
      y = moveResult.newY;
      direction = moveResult.newDirection;
      board[x][y] = direction;
      counter++;
    }
  }

  // logBoard(board);
  const answer = 1000 * (x + 1) + 4 * (y + 1) + getFacingScore(board[x][y]);
  console.log(answer);
}

main();

// 110400
