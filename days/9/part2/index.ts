import { readFileSync } from "fs";
import { cloneDeep, sortBy } from "lodash";
import { join } from "path";

interface Basin {
  x: number;
  y: number;
  height: number;
}

interface Height {
  height: number;
  leftIsHigher: boolean;
  topIsHigher: boolean;
  rightIsHigher: boolean;
  bottomIsHigher: boolean;
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");
  lines.pop();
  const heightMap: Height[][] = lines.map((line) =>
    line.split("").map((height) => ({
      height: Number(height),
      leftIsHigher: false,
      topIsHigher: false,
      rightIsHigher: false,
      bottomIsHigher: false,
    }))
  );

  const basins: Basin[] = [];
  for (let x = 0; x < heightMap.length; x++) {
    for (let y = 0; y < heightMap[x].length; y++) {
      const current = heightMap[x][y];
      const currentHeight = current.height;
      const topIsHigher =
        x > 0 ? currentHeight < heightMap[x - 1][y].height : true;
      const leftIsHigher =
        y > 0 ? currentHeight < heightMap[x][y - 1].height : true;
      const bottomIsHigher =
        x < heightMap.length - 1
          ? currentHeight < heightMap[x + 1][y].height
          : true;
      const rightIsHigher =
        y < heightMap[x].length - 1
          ? currentHeight < heightMap[x][y + 1].height
          : true;

      current.leftIsHigher = leftIsHigher;
      current.topIsHigher = topIsHigher;
      current.rightIsHigher = rightIsHigher;
      current.bottomIsHigher = bottomIsHigher;

      if (leftIsHigher && topIsHigher && rightIsHigher && bottomIsHigher) {
        basins.push({ x, y, height: currentHeight });
      }
    }
  }
  const visitedMap = cloneDeep(heightMap).map((row) => row.map((_) => false));
  function computeBasinWeight(x: number, y: number) {
    if (visitedMap[x][y]) {
      return 0;
    }
    visitedMap[x][y] = true;

    const { leftIsHigher, topIsHigher, rightIsHigher, bottomIsHigher } =
      heightMap[x][y];

    let basinWeight = 0;

    if (
      x > 0 &&
      topIsHigher &&
      // heightMap[x - 1][y].height === heightMap[x][y].height + 1 &&
      heightMap[x - 1][y].height !== 9
    ) {
      basinWeight += computeBasinWeight(x - 1, y);
    }
    if (
      y > 0 &&
      leftIsHigher &&
      // heightMap[x][y - 1].height === heightMap[x][y].height + 1 &&
      heightMap[x][y - 1].height !== 9
    ) {
      basinWeight += computeBasinWeight(x, y - 1);
    }
    if (
      x < heightMap.length - 1 &&
      bottomIsHigher &&
      // heightMap[x + 1][y].height === heightMap[x][y].height + 1 &&
      heightMap[x + 1][y].height !== 9
    ) {
      basinWeight += computeBasinWeight(x + 1, y);
    }
    if (
      y < heightMap[x].length - 1 &&
      rightIsHigher &&
      // heightMap[x][y + 1].height === heightMap[x][y].height + 1 &&
      heightMap[x][y + 1].height !== 9
    ) {
      basinWeight += computeBasinWeight(x, y + 1);
    }

    return 1 + basinWeight;
  }

  const basinWeights: number[] = [];
  for (const basin of basins) {
    basinWeights.push(computeBasinWeight(basin.x, basin.y));
  }

  const answer = basinWeights
    .sort((a, b) => b - a)
    .slice(0, 3)
    .reduce((a, b) => a * b);
  console.log(answer);
}

main();

// 949905
