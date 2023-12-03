import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const depths = input.split("\n").map(Number);
  const answer = depths.reduce((acc, depth, index) => {
    if (index === 0) {
      return 0;
    }

    return depth > depths[index - 1] ? acc + 1 : acc;
  }, 0);
  console.log(answer);
}

main();

// 1766
