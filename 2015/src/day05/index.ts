import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function parseInput(input: string) {
  return input.split("\n");
}

function goA(parsedInput: ReturnType<typeof parseInput>) {
  let counter = 0;
  for (const line of parsedInput) {
    if ((line.match(/(a|e|i|o|u)/g) ?? []).length < 3) {
      continue;
    }
    let letterAppearsTwice = false;
    let forbiddenPairAppears = false;
    for (let i = 0; i < line.length - 1; i++) {
      if (line.at(i) === line.at(i + 1)) {
        letterAppearsTwice = true;
      }
      const pair = line.substring(i, i + 2);
      if (["ab", "cd", "pq", "xy"].includes(pair)) {
        forbiddenPairAppears = true;
        continue;
      }
    }
    if (letterAppearsTwice === false || forbiddenPairAppears === true) {
      continue;
    }
    counter++;
  }

  return counter;
}

function goB(parsedInput: ReturnType<typeof parseInput>) {
  let counter = 0;

  for (const line of parsedInput) {
    let hasDuplicatePair = false;
    for (let i = 0; i < line.length - 1; i++) {
      const pair = line.substring(i, i + 2);
      for (let j = 0; j < line.length - 1; j++) {
        if (i - 1 <= j && j <= i + 1) {
          continue;
        }
        const pairToTest = line.substring(j, j + 2);
        if (pair === pairToTest) {
          hasDuplicatePair = true;
          break;
        }
      }
      if (hasDuplicatePair === true) {
        break;
      }
    }
    if (hasDuplicatePair === false) {
      continue;
    }

    for (let i = 0; i < line.length - 2; i++) {
      if (line.at(i) === line.at(i + 2)) {
        counter++;
        break;
      }
    }
  }
  return counter;
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

// 236
// 51
