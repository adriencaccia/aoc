import { readFileSync } from "fs";
import { chunk, flatten, intersection, sum } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function invertBoard(board: number[][]) {
  return board[0].map((_, colIndex) => board.map((row) => row[colIndex]));
}

function getWinningBoard(numbersToDraw: number[], bingoBoards: number[][][]) {
  for (let draw = 4; draw < numbersToDraw.length; draw++) {
    const drawnNumbers = numbersToDraw.slice(0, draw + 1);
    const winningBoard = bingoBoards.find(
      (board) =>
        board.some((row) => intersection(drawnNumbers, row).length === 5) ||
        invertBoard(board).some(
          (row) => intersection(drawnNumbers, row).length === 5
        )
    );

    if (winningBoard) {
      return { winningBoard, drawnNumbers };
    }
  }
  throw new Error("No winning board found");
}

function getWinningBoardScore(
  winningBoard: number[][],
  drawnNumbers: number[]
) {
  const unmarkedNumbers = flatten(winningBoard).filter(
    (number) => !drawnNumbers.includes(number)
  );

  return sum(unmarkedNumbers) * (drawnNumbers.pop() ?? 0);
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");
  const numbersToDraw = lines[0].split(",").map(Number);
  const bingoBoards = chunk(
    lines
      .slice(1)
      .filter((line) => line !== "")
      .map((line) =>
        line.replaceAll("  ", " ").trimStart().split(" ").map(Number)
      ),
    5
  );

  const { drawnNumbers, winningBoard } = getWinningBoard(
    numbersToDraw,
    bingoBoards
  );

  const answer = getWinningBoardScore(winningBoard, drawnNumbers);
  console.log(answer);
}

main();

// 25023
