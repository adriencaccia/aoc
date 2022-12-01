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
      x1,
      y1,
      x2,
      y2,
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

  const axialLines = coords.filter((c) => c.x1 === c.x2 || c.y1 === c.y2);

  function drawAxialLine(line: Coords) {
    const x1 = Math.min(line.x1, line.x2);
    const y1 = Math.min(line.y1, line.y2);
    const x2 = Math.max(line.x1, line.x2);
    const y2 = Math.max(line.y1, line.y2);
    for (let x = x1; x <= x2; x++) {
      for (let y = y1; y <= y2; y++) {
        ventsGrid[y][x] += 1;
      }
    }
  }

  axialLines.forEach(drawAxialLine);

  const diagonalLines = coords.filter((c) => c.x1 !== c.x2 && c.y1 !== c.y2);

  function drawDiagonalLine(line: Coords) {
    const xDirectionRight = line.x1 < line.x2 ? true : false;
    const yDirectionBottom = line.y1 < line.y2 ? true : false;

    for (let index = 0; index <= Math.abs(line.x1 - line.x2); index++) {
      const x = xDirectionRight ? line.x1 + index : line.x1 - index;
      const y = yDirectionBottom ? line.y1 + index : line.y1 - index;
      ventsGrid[y][x] += 1;
    }
  }

  diagonalLines.forEach(drawDiagonalLine);

  const answer = flatten(ventsGrid).filter((v) => v > 1).length;
  console.log(answer);
}

main();

// 17717
