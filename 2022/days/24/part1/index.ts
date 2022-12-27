import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
// import logMatrixStep from "../../../../utils/logMatrixStep.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

const DIRECTIONS = ["👉", "👇", "👈", "👆"] as const;
type Direction = typeof DIRECTIONS[number];
const WIND_TILES = ["🟨", ...DIRECTIONS] as const;
type WindTile = typeof WIND_TILES[number];
const TILES = ["⬛", "⬜", "🟦", ...WIND_TILES] as const;
type Tile = typeof TILES[number];
interface Position {
  i: number;
  j: number;
}
interface Wind extends Position {
  direction: Direction;
}

function prepareInput(input: string): { map: Tile[][]; winds: Wind[] } {
  const winds: Wind[] = [];
  const map = input.split("\n").map((line, i) =>
    line.split("").map((tile, j) => {
      switch (tile) {
        case "#":
          return "⬛";
        case ">":
          winds.push({ i, j, direction: "👉" });
          return "👉";
        case "v":
          winds.push({ i, j, direction: "👇" });
          return "👇";
        case "<":
          winds.push({ i, j, direction: "👈" });
          return "👈";
        case "^":
          winds.push({ i, j, direction: "👆" });
          return "👆";
        default:
          return "⬜";
      }
    })
  );
  return { map, winds };
}

async function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const { map, winds } = prepareInput(input);

  const iLength = map.length;
  const jLength = map[0].length;
  let reachablePositions: Position[] = [{ i: 0, j: 1 }];

  // await logMatrixStep(map);
  let minute = 0;
  while (map[iLength - 1][jLength - 2] !== "🟦") {
    // clear map and calculate new winds
    for (let windIndex = 0; windIndex < winds.length; windIndex++) {
      const { i, j, direction } = winds[windIndex];
      map[i][j] = "⬜";
      switch (direction) {
        case "👆":
          winds[windIndex].i = map[i - 1][j] === "⬛" ? iLength - 2 : i - 1;
          continue;
        case "👇":
          winds[windIndex].i = map[i + 1][j] === "⬛" ? 1 : i + 1;
          continue;
        case "👈":
          winds[windIndex].j = map[i][j - 1] === "⬛" ? jLength - 2 : j - 1;
          continue;
        case "👉":
          winds[windIndex].j = map[i][j + 1] === "⬛" ? 1 : j + 1;
          continue;
      }
    }

    // add new winds to map
    for (const { i, j, direction } of winds) {
      const tile = map[i][j];
      if (tile === "⬜") {
        map[i][j] = direction;
        continue;
      }
      map[i][j] = "🟨";
    }

    // add new reachable positions
    const newReachablePositions: Position[] = [];

    reachablePositions = reachablePositions.filter(({ i, j }) => {
      if (map[i - 1]?.[j] === "⬜") {
        map[i - 1][j] = "🟦";
        newReachablePositions.push({ i: i - 1, j });
      }
      if (map[i + 1]?.[j] === "⬜") {
        map[i + 1][j] = "🟦";
        newReachablePositions.push({ i: i + 1, j });
      }
      if (map[i][j - 1] === "⬜") {
        map[i][j - 1] = "🟦";
        newReachablePositions.push({ i, j: j - 1 });
      }
      if (map[i][j + 1] === "⬜") {
        map[i][j + 1] = "🟦";
        newReachablePositions.push({ i, j: j + 1 });
      }
      if (WIND_TILES.includes(map[i][j] as WindTile)) {
        return false;
      }
      map[i][j] = "🟦";
      return true;
    });

    reachablePositions.push(...newReachablePositions);

    minute++;
    // await logMatrixStep(map);
  }

  console.log(minute);
}

main();

// 269
