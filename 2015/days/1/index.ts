import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function parseInput(input: string) {
  return input;
}

function goA(input: string) {
  const parsedInput = parseInput(input);
  let answer = 0;
  for (const char of parsedInput) {
    answer += char === "(" ? 1 : -1;
  }

  return answer;
}

function goB(input: string) {
  const parsedInput = parseInput(input);
  let answer = 0;
  let position = 1;
  for (const char of parsedInput) {
    answer += char === "(" ? 1 : -1;
    if (answer === -1) {
      return position;
    }
    position++;
  }
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

// 232
// 1783
