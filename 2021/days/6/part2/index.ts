import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
import { clone } from "lodash-es";

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n").slice(0);
  const inputs = lines[0].split(",").map(Number);

  const numberCounter: Record<number, number> = {
    0: inputs.filter((n) => n === 0).length,
    1: inputs.filter((n) => n === 1).length,
    2: inputs.filter((n) => n === 2).length,
    3: inputs.filter((n) => n === 3).length,
    4: inputs.filter((n) => n === 4).length,
    5: inputs.filter((n) => n === 5).length,
    6: inputs.filter((n) => n === 6).length,
    7: inputs.filter((n) => n === 7).length,
    8: inputs.filter((n) => n === 8).length,
  };

  for (let i = 0; i < 256; i++) {
    const previousNumberCounter = clone(numberCounter);
    for (let j = 0; j < 9; j++) {
      if (j === 8) {
        numberCounter[6] += previousNumberCounter[0];
        numberCounter[8] = previousNumberCounter[0];
        continue;
      }
      numberCounter[j] = previousNumberCounter[j + 1];
    }
  }

  const answer = Object.values(numberCounter).reduce((a, b) => a + b);
  console.log(answer);
}

main();

// 1617359101538
