import { readFileSync } from "fs";
import { mean } from "lodash";
import { join } from "path";

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n").slice(0);
  const inputs = lines[0].split(",").map(Number);

  const inputsMean = Math.floor(mean(inputs));

  const answer = inputs.reduce(
    (acc, curr) =>
      acc +
      (Math.abs(curr - inputsMean) * (Math.abs(curr - inputsMean) + 1)) / 2,
    0
  );
  console.log(answer);
}

main();

// 96987874
