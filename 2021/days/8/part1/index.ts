import { readFileSync } from "fs";
import { flatten } from "lodash";
import { join } from "path";

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");
  lines.pop();
  const outputValues = flatten(
    lines.map((line) => line.split(" | ")[1].split(" "))
  );

  const digitLengths = [2, 4, 3, 7];

  const answer = outputValues.reduce(
    (acc, curr) => acc + (digitLengths.includes(curr.length) ? 1 : 0),
    0
  );

  console.log(answer);
}

main();

// 303
