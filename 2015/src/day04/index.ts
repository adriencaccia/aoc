import { createHash, Hash } from "crypto";
import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function parseInput(input: string) {
  return input;
}

function findHash(input: string, zeros: number) {
  const parsedInput = parseInput(input);
  let testNumber = 1;

  while (true) {
    const stringToHash = parsedInput + testNumber.toString();
    const hash = createHash("md5").update(stringToHash).digest("hex");
    if (hash.startsWith("".padEnd(zeros, "0"))) {
      return testNumber;
    }
    testNumber++;
  }
}

function goA(input: string) {
  return findHash(input, 5);
}

function goB(input: string) {
  return findHash(input, 6);
}

function main() {
  const input = readFileSync(join(__dirname, "input.txt"), "utf8");

  const answerA = goA(input);
  console.log("Part 1");
  console.log(answerA);

  const answerB = goB(input);
  console.log("Part 2");
  console.log(answerB);
}

main();

// 117946
// 3938038
