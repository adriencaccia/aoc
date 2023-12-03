import { readFileSync } from "fs";
import { cloneDeep } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
// import logMatrix from "../../../../utils/logMatrix.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");

  const destination = { i: 0, j: 0 };

  const heightMap = input.split("\n").map((line, i) =>
    line.split("").map((location, j) => {
      if (location === "S") {
        return 27 - 1;
      }
      if (location === "E") {
        destination.i = i;
        destination.j = j;
        return 27 - 26;
      }
      return 27 - (location.charCodeAt(0) - 96);
    })
  );

  const fastestWayMap = cloneDeep(heightMap).map((line) =>
    line.map((_) => "🟩" as "🟩" | number)
  );

  let smallestDistance = Number.MAX_SAFE_INTEGER;

  function visitNeighbors(i: number, j: number, distance = 0) {
    const height = heightMap[i][j];
    const currentDistance = fastestWayMap[i][j];
    if (height === 26 && distance < smallestDistance) {
      smallestDistance = distance;
    }
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
  visitNeighbors(destination.i, destination.j);

  // logMatrix(fastestWayMap);

  console.log(smallestDistance);
}

main();

// 354
