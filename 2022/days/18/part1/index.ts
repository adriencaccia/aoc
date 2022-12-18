import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const droplets = input
    .split("\n")
    .map((line) => line.split(",").map((coord) => Number(coord) + 1));

  const bigCubeSize = Math.max(...droplets.flat()) + 2;

  const bigCube = Array.from({ length: bigCubeSize }, () =>
    Array.from({ length: bigCubeSize }, () =>
      Array.from({ length: bigCubeSize }, () => 0)
    )
  );

  const surfaceArea = droplets.reduce((acc, [x, y, z]) => {
    const xCubes = bigCube[x - 1][y][z] + bigCube[x + 1][y][z];
    const yCubes = bigCube[x][y - 1][z] + bigCube[x][y + 1][z];
    const zCubes = bigCube[x][y][z - 1] + bigCube[x][y][z + 1];

    const dropletVisibleFaces = 6 - 2 * (xCubes + yCubes + zCubes);

    bigCube[x][y][z] = 1;
    return acc + dropletVisibleFaces;
  }, 0);

  console.log(surfaceArea);
}

main();

// 3412
