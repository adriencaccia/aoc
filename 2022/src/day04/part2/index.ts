import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");

  const elvesPairs = input
    .split("\n")
    .map((line) =>
      line.split(",").map((cleaningLine) => cleaningLine.split("-").map(Number))
    );
  elvesPairs.pop();

  const overlappingPairs = elvesPairs.filter(([firstElf, secondElf]) => {
    const firstOverlapsWithSecond =
      (secondElf[0] <= firstElf[0] && firstElf[0] <= secondElf[1]) ||
      (secondElf[0] <= firstElf[1] && firstElf[1] <= secondElf[1]);

    const secondOverlapsWithFirst =
      (firstElf[0] <= secondElf[0] && secondElf[0] <= firstElf[1]) ||
      (firstElf[0] <= secondElf[1] && secondElf[1] <= firstElf[1]);

    return firstOverlapsWithSecond || secondOverlapsWithFirst;
  });

  console.log(overlappingPairs.length);
}

main();

// 854
