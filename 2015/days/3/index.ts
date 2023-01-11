import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function parseInput(input: string) {
  return input;
}

function goA(input: string) {
  const parsedInput = parseInput(input);

  let x = 0;
  let y = 0;
  const houses: Record<string, number> = { x0y0: 1 };
  for (const direction of parsedInput) {
    switch (direction) {
      case "v":
        x++;
        break;
      case ">":
        y++;
        break;
      case "^":
        x--;
        break;
      case "<":
        y--;
        break;
    }
    if (houses[`x${x}y${y}`]) {
      houses[`x${x}y${y}`]++;
      continue;
    }
    houses[`x${x}y${y}`] = 1;
  }

  return Object.keys(houses).length;
}

function goB(input: string) {
  const parsedInput = parseInput(input);

  const santa = { x: 0, y: 0 };
  const robot = { x: 0, y: 0 };
  const houses: Record<string, number> = { x0y0: 1 };
  let index = 0;
  for (const direction of parsedInput) {
    switch (direction) {
      case "v":
        (index % 2 === 0 ? santa : robot).x++;
        break;
      case ">":
        (index % 2 === 0 ? santa : robot).y++;
        break;
      case "^":
        (index % 2 === 0 ? santa : robot).x--;
        break;
      case "<":
        (index % 2 === 0 ? santa : robot).y--;
        break;
    }
    const x = (index % 2 === 0 ? santa : robot).x;
    const y = (index % 2 === 0 ? santa : robot).y;
    index++;
    if (houses[`x${x}y${y}`]) {
      houses[`x${x}y${y}`]++;
      continue;
    }
    houses[`x${x}y${y}`] = 1;
  }

  return Object.keys(houses).length;
}

function main() {
  const input = readFileSync(join(__dirname, "input.txt"), "utf8");

  const answerA = goA(input);
  const answerB = goB(input);

  console.log("Part 1");
  console.log(answerA);
  console.log("Part 2");
  console.log(answerB);
}

main();

// 2081
// 2341
