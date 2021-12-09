import { readFileSync } from "fs";
import { join } from "path";

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");
  lines.pop();
  const heightMap = lines.map((line) => line.split("").map(Number));

  let answer = 0;
  for (let x = 0; x < heightMap.length; x++) {
    for (let y = 0; y < heightMap[x].length; y++) {
      const current = heightMap[x][y];
      if (
        (x - 1 >= 0 ? heightMap[x - 1][y] > current : true) &&
        (x + 1 < heightMap.length ? heightMap[x + 1][y] > current : true) &&
        (y - 1 >= 0 ? heightMap[x][y - 1] > current : true) &&
        (y + 1 < heightMap.length ? heightMap[x][y + 1] > current : true)
      ) {
        answer += current + 1;
      }
    }
  }

  console.log(answer);
}

main();

// 518
