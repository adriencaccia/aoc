import { readFileSync } from "fs";
import { chunk, flatten, intersection, keys, pickBy, sum } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function invertBoard(board: number[][]) {
  return board[0].map((_, colIndex) => board.map((row) => row[colIndex]));
}

function getLastWinningBoard(
  numbersToDraw: number[],
  bingoBoards: number[][][]
) {
  const winningBoardIndexes: number[] = [];
  let lastWinningDrawnNumbers: number[] = [];

  for (let draw = 4; draw < numbersToDraw.length; draw++) {
    const drawnNumbers = numbersToDraw.slice(0, draw + 1);
    const currentWinningBoardIndexes = keys(
      pickBy(bingoBoards, (board, index) => {
        if (winningBoardIndexes.includes(parseInt(index, 10))) {
          return false;
        }

        return (
          board.some((row) => intersection(drawnNumbers, row).length === 5) ||
          invertBoard(board).some(
            (row) => intersection(drawnNumbers, row).length === 5
          )
        );
      })
    ).map(Number);

    if (currentWinningBoardIndexes.length !== 0) {
      winningBoardIndexes.push(...currentWinningBoardIndexes);
      lastWinningDrawnNumbers = drawnNumbers;
    }
  }

  return {
    board: bingoBoards[winningBoardIndexes[winningBoardIndexes.length - 1]],
    drawnNumbers: lastWinningDrawnNumbers,
  };
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

  const { drawnNumbers, board } = getLastWinningBoard(
    numbersToDraw,
    bingoBoards
  );

  const answer = getWinningBoardScore(board, drawnNumbers);
  console.log(answer);
}

main();

// 2634
