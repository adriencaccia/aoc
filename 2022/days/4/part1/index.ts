import { readFileSync } from "fs";
import { join } from "path";

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");

  const elvesPairs = input
    .split("\n")
    .map((line) =>
      line.split(",").map((cleaningLine) => cleaningLine.split("-").map(Number))
    );
  elvesPairs.pop();

  const overlappingPairs = elvesPairs.filter(([firstElf, secondElf]) => {
    const firstIsContainedInSecond =
      firstElf[0] >= secondElf[0] && firstElf[1] <= secondElf[1];
    const secondIsContainedInFirst =
      firstElf[0] <= secondElf[0] && firstElf[1] >= secondElf[1];

    return firstIsContainedInSecond || secondIsContainedInFirst;
  });

  console.log(overlappingPairs.length);
}

main();

// 532
