import { readFileSync } from "fs";
import { cloneDeep } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const droplets = input
    .split("\n")
    .map((line) => line.split(",").map((coord) => Number(coord) + 2));

  const bigCubeSize = Math.max(...droplets.flat()) + 3;

  const bigCube = Array.from({ length: bigCubeSize }, () =>
    Array.from({ length: bigCubeSize }, () =>
      Array.from({ length: bigCubeSize }, () => "⬜")
    )
  );

  droplets.forEach(([x, y, z]) => {
    bigCube[x][y][z] = "🟧";
  });

  function visitShell(x: number, y: number, z: number): number {
    if (
      bigCube[x][y][z] !== "⬜" ||
      x <= 0 ||
      y <= 0 ||
      z <= 0 ||
      x >= bigCubeSize - 1 ||
      y >= bigCubeSize - 1 ||
      z >= bigCubeSize - 1 ||
      [
        bigCube[x - 1][y][z],
        bigCube[x + 1][y][z],
        bigCube[x][y - 1][z],
        bigCube[x][y + 1][z],
        bigCube[x][y][z - 1],
        bigCube[x][y][z + 1],
        bigCube[x + 1][y][z + 1],
        bigCube[x][y + 1][z + 1],
        bigCube[x + 1][y + 1][z],
        bigCube[x + 1][y + 1][z + 1],
        bigCube[x - 1][y][z - 1],
        bigCube[x][y - 1][z - 1],
        bigCube[x - 1][y - 1][z],
        bigCube[x - 1][y - 1][z - 1],
      ].every((droplet) => droplet !== "🟧")
    ) {
      return 0;
    }
    bigCube[x][y][z] = "🟪";

    const surfaceContact =
      (bigCube[x - 1][y][z] === "🟧" ? 1 : 0) +
      (bigCube[x + 1][y][z] === "🟧" ? 1 : 0) +
      (bigCube[x][y - 1][z] === "🟧" ? 1 : 0) +
      (bigCube[x][y + 1][z] === "🟧" ? 1 : 0) +
      (bigCube[x][y][z - 1] === "🟧" ? 1 : 0) +
      (bigCube[x][y][z + 1] === "🟧" ? 1 : 0);

    return (
      surfaceContact +
      visitShell(x - 1, y, z) +
      visitShell(x + 1, y, z) +
      visitShell(x, y - 1, z) +
      visitShell(x, y + 1, z) +
      visitShell(x, y, z - 1) +
      visitShell(x, y, z + 1)
    );
  }

  const surfaceArea = visitShell(13, 6, 4);

  console.log(surfaceArea);

  // bigCube.map((slice) => slice.forEach((line) => console.log(line.join(""))));
}

main();

// 2018
