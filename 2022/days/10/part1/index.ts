import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");

  const cycles: number[] = [];

  for (const line of lines) {
    const currentRegisterValue = cycles.at(-1) ?? 1;
    if (line === "noop") {
      cycles.push(currentRegisterValue);
      continue;
    }
    const [, stringValue] = line.split(" ");

    cycles.push(
      currentRegisterValue,
      currentRegisterValue + Number(stringValue)
    );
  }

  const sum =
    20 * cycles[20 - 2] +
    60 * cycles[60 - 2] +
    100 * cycles[100 - 2] +
    140 * cycles[140 - 2] +
    180 * cycles[180 - 2] +
    220 * cycles[220 - 2];

  console.log(sum);
}

main();

// 13920
