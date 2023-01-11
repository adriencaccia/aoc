import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function parseInput(input: string) {
  return input.split("\n").map((line) => {
    const [, action, xStart, yStart, xEnd, yEnd] = line.match(
      /(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)/
    )!;

    return {
      action: action as "turn on" | "turn off" | "toggle",
      xStart: Number(xStart),
      yStart: Number(yStart),
      xEnd: Number(xEnd),
      yEnd: Number(yEnd),
    };
  });
}

function goA(parsedInput: ReturnType<typeof parseInput>) {
  const grid = Array.from({ length: 1000 }, () =>
    Array.from({ length: 1000 }, () => false)
  );

  for (const { action, xEnd, xStart, yEnd, yStart } of parsedInput) {
    for (let x = xStart; x <= xEnd; x++) {
      for (let y = yStart; y <= yEnd; y++) {
        switch (action) {
          case "toggle":
            grid[x][y] = !grid[x][y];
            break;
          case "turn off":
            grid[x][y] = false;
            break;
          case "turn on":
            grid[x][y] = true;
            break;
        }
      }
    }
  }

  return grid.flat().filter((isLit) => isLit === true).length;
}

function goB(parsedInput: ReturnType<typeof parseInput>) {
  const grid = Array.from({ length: 1000 }, () =>
    Array.from({ length: 1000 }, () => 0)
  );

  for (const { action, xEnd, xStart, yEnd, yStart } of parsedInput) {
    for (let x = xStart; x <= xEnd; x++) {
      for (let y = yStart; y <= yEnd; y++) {
        switch (action) {
          case "toggle":
            grid[x][y] += 2;
            break;
          case "turn off":
            grid[x][y] = grid[x][y] === 0 ? 0 : grid[x][y] - 1;
            break;
          case "turn on":
            grid[x][y]++;
            break;
        }
      }
    }
  }

  return grid.flat().reduce((a, b) => a + b);
}

function main() {
  const input = readFileSync(join(__dirname, "input.txt"), "utf8").trim();
  const parsedInput = parseInput(input);

  const answerA = goA(parsedInput);
  console.log("Part 1");
  console.log(answerA);

  const answerB = goB(parsedInput);
  console.log("Part 2");
  console.log(answerB);
}

main();

// 569999
// 17836115
