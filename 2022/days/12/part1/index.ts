import { readFileSync } from "fs";
import { cloneDeep } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
// import logMatrix from "../../../../utils/logMatrix.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");

  const start = { i: 0, j: 0 };
  const destination = { i: 0, j: 0 };

  const heightMap = input.split("\n").map((line, i) =>
    line.split("").map((location, j) => {
      if (location === "S") {
        start.i = i;
        start.j = j;
        return 1;
      }
      if (location === "E") {
        destination.i = i;
        destination.j = j;
        return 26;
      }
      return location.charCodeAt(0) - 96;
    })
  );

  const fastestWayMap = cloneDeep(heightMap).map((line) =>
    line.map((_) => "🟩" as "🟩" | number)
  );

  function visitNeighbors(i: number, j: number, distance = 0) {
    const height = heightMap[i][j];
    const currentDistance = fastestWayMap[i][j];
    if (currentDistance !== "🟩" && distance >= currentDistance) {
      return;
    }
    fastestWayMap[i][j] = distance;
    if (i > 0 && heightMap[i - 1][j] <= height + 1) {
      visitNeighbors(i - 1, j, distance + 1);
    }

    if (j < heightMap[0].length - 1 && heightMap[i][j + 1] <= height + 1) {
      visitNeighbors(i, j + 1, distance + 1);
    }
    if (i < heightMap.length - 1 && heightMap[i + 1][j] <= height + 1) {
      visitNeighbors(i + 1, j, distance + 1);
    }
    if (j > 0 && heightMap[i][j - 1] <= height + 1) {
      visitNeighbors(i, j - 1, distance + 1);
    }
  }
  visitNeighbors(start.i, start.j);

  // logMatrix(fastestWayMap);

  console.log(fastestWayMap[destination.i][destination.j]);
}

main();

// 361
