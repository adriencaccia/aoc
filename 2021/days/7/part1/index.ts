import { readFileSync } from "fs";
import { join } from "path";

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n").slice(0);
  const inputs = lines[0].split(",").map(Number);

  const sortedInputs = inputs.sort((a, b) => a - b);

  const sortedInputsMedian = sortedInputs[Math.floor(sortedInputs.length / 2)];

  const answer = sortedInputs.reduce(
    (acc, curr) => acc + Math.abs(curr - sortedInputsMedian),
    0
  );
  console.log(answer);
}

main();

// 340987
