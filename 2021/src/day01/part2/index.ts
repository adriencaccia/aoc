import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const depths = input.split("\n").map(Number);
  const slidingWindows: number[] = [];
  for (let index = 0; index < depths.length - 2; index++) {
    slidingWindows.push(depths[index] + depths[index + 1] + depths[index + 2]);
  }

  const answer = slidingWindows.reduce((acc, slidingWindow, index) => {
    if (index === 0) {
      return 0;
    }

    return slidingWindow > slidingWindows[index - 1] ? acc + 1 : acc;
  }, 0);
  console.log(answer);
}

main();

// 1797
