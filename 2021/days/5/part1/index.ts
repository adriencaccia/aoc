import { readFileSync } from "fs";
import { flatten, maxBy } from "lodash";
import { join } from "path";

interface Coords {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
}

function getMaxXY(coords: Coords[]) {
  const maxXCoords = maxBy(coords, (c) => Math.max(c.x1, c.x2)) ?? {
    x1: 0,
    y1: 0,
    x2: 0,
    y2: 0,
  };
  const maxX = Math.max(maxXCoords.x1, maxXCoords.x2);
  const maxYCoords = maxBy(coords, (c) => Math.max(c.y1, c.y2)) ?? {
    x1: 0,
    y1: 0,
    x2: 0,
    y2: 0,
  };
  const maxY = Math.max(maxYCoords.y1, maxYCoords.y2);

  return { maxX, maxY };
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n").slice(0, -1);
  const coords: Coords[] = lines.map((line) => {
    const [x1, y1, x2, y2] = line.replace(" -> ", ",").split(",").map(Number);

    return {
      x1: Math.min(x1, x2),
      y1: Math.min(y1, y2),
      x2: Math.max(x1, x2),
      y2: Math.max(y1, y2),
    };
  });
  const { maxX, maxY } = getMaxXY(coords);

  const ventsGrid = Array(maxX + 1)
    .fill(0)
    .map(() =>
      Array(maxY + 1)
        .fill(0)
        .map(() => 0)
    );

  const horizontalLines = coords.filter((c) => c.y1 === c.y2);
  const verticalLines = coords.filter((c) => c.x1 === c.x2);

  function drawLine(line: Coords) {
    for (let x = line.x1; x <= line.x2; x++) {
      for (let y = line.y1; y <= line.y2; y++) {
        ventsGrid[y][x] += 1;
      }
    }
  }

  horizontalLines.forEach((line) => drawLine(line));
  verticalLines.forEach((line) => drawLine(line));

  const answer = flatten(ventsGrid).filter((v) => v > 1).length;
  console.log(answer);
}

main();

// 4728
