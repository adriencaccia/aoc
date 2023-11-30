import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function parseInput(input: string) {
  return input.split("\n");
}

function goA(parsedInput: ReturnType<typeof parseInput>) {
  return parsedInput.length;
}

function goB(parsedInput: ReturnType<typeof parseInput>) {
  return parsedInput.length;
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

// ???
// ???
