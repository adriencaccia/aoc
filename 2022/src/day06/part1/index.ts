import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");

  let answer = 0;
  for (let i = 3; i < input.length; i++) {
    const charactersSet = new Set(input.substring(i - 3, i + 1));
    if (charactersSet.size === 4) {
      answer = i + 1;
      break;
    }
  }

  console.log(answer);
}

main();

// 1651
