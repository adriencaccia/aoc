import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function parseInput(input: string) {
  return input.split("\n").map((line) => line.split("x").map(Number));
}

function goA(input: string) {
  const parsedInput = parseInput(input);

  let answer = 0;
  for (const [l, w, h] of parsedInput) {
    answer += 2 * l * w + 2 * w * h + 2 * h * l;
    answer += Math.min(l * w, w * h, h * l);
  }

  return answer;
}

function goB(input: string) {
  const parsedInput = parseInput(input);

  let answer = 0;
  for (const [l, w, h] of parsedInput) {
    answer += Math.min(2 * (l + w), 2 * (w + h), 2 * (h + l));
    answer += l * w * h;
  }

  return answer;
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

// 1588178
// ???
