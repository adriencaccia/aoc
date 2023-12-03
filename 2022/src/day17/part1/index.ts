import { readFileSync } from "fs";
import { clone } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

const ROCKS = [
  ["⬜⬜⬛⬛⬛⬛⬜"],
  ["⬜⬜⬜⬛⬜⬜⬜", "⬜⬜⬛⬛⬛⬜⬜", "⬜⬜⬜⬛⬜⬜⬜"],
  ["⬜⬜⬜⬜⬛⬜⬜", "⬜⬜⬜⬜⬛⬜⬜", "⬜⬜⬛⬛⬛⬜⬜"],
  ["⬜⬜⬛⬜⬜⬜⬜", "⬜⬜⬛⬜⬜⬜⬜", "⬜⬜⬛⬜⬜⬜⬜", "⬜⬜⬛⬜⬜⬜⬜"],
  ["⬜⬜⬛⬛⬜⬜⬜", "⬜⬜⬛⬛⬜⬜⬜"],
];

function logShape(shape: string[]) {
  shape.forEach((line) => console.log(line));
}

function logTower(tower: string[][]) {
  tower.forEach((line) => console.log(line.join("")));
}

function pushShape(
  shape: string[],
  jetDirection: ">" | "<",
  tower: string[][],
  currentBottomEdgeHeight: number
) {
  const edge = jetDirection === ">" ? -1 : 0;
  const isShapeBlockedByTowerEdge = shape.some(
    (line) => line.at(edge) === "⬛"
  );
  if (isShapeBlockedByTowerEdge) {
    return shape;
  }
  // push shape
  const pushedShape = shape.map((line) => {
    if (jetDirection === ">") {
      return "⬜" + line.substring(0, 6);
    }
    return line.substring(1) + "⬜";
  });

  for (let lineIndex = 0; lineIndex < pushedShape.length; lineIndex++) {
    const line = pushedShape[lineIndex];
    for (let columnIndex = 0; columnIndex < tower[0].length; columnIndex++) {
      if (line.at(columnIndex) === "⬜") {
        continue;
      }
      const currentHeight =
        currentBottomEdgeHeight - (lineIndex - (pushedShape.length - 1));

      // collision
      if (tower[TOWER_HEIGHT - currentHeight][columnIndex] === "⬛") {
        return shape;
      }
    }
  }

  return pushedShape;
}

function canLowerShape(
  shape: string[],
  tower: string[][],
  currentBottomEdgeHeight: number
) {
  for (let lineIndex = 0; lineIndex < shape.length; lineIndex++) {
    const line = shape[lineIndex];
    for (let columnIndex = 0; columnIndex < tower[0].length; columnIndex++) {
      if (line.at(columnIndex) === "⬜") {
        continue;
      }
      const currentHeight =
        currentBottomEdgeHeight - (lineIndex - (shape.length - 1));

      if (tower[TOWER_HEIGHT - (currentHeight - 1)][columnIndex] === "⬛") {
        return false;
      }
    }
  }
  return true;
}

function updateTower(
  tower: string[][],
  towerHeights: number[],
  shape: string[],
  currentBottomEdgeHeight: number
) {
  for (let columnIndex = 0; columnIndex < tower[0].length; columnIndex++) {
    for (let lineIndex = 0; lineIndex < shape.length; lineIndex++) {
      const line = shape[lineIndex];
      if (line.at(columnIndex) === "⬜") {
        continue;
      }
      const heightOfPixel = currentBottomEdgeHeight + shape.length - lineIndex;
      if (heightOfPixel - 1 > towerHeights[columnIndex]) {
        towerHeights[columnIndex] = heightOfPixel - 1;
      }

      tower[TOWER_HEIGHT - (heightOfPixel - 1)][columnIndex] = "⬛";
    }
  }
}

const TOWER_HEIGHT = 3230;

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const jetDirections = input.split("") as (">" | "<")[];

  const towerHeights = Array.from({ length: 7 }, () => 0);
  const tower = Array.from({ length: TOWER_HEIGHT + 1 }, (_, index) =>
    Array.from({ length: 7 }, () => (index === 0 ? "⬛" : "⬜"))
  );
  tower.reverse();

  let jetDirectionsIndex = 0;
  for (let rockIndex = 1; rockIndex <= 2022; rockIndex++) {
    let rockShape = clone(ROCKS[(rockIndex - 1) % 5]);
    const maxHeight = Math.max(...towerHeights);
    let blocked = false;
    let currentBottomEdgeHeight = maxHeight + 4;
    while (blocked === false) {
      const jetDirection = jetDirections[jetDirectionsIndex];
      rockShape = pushShape(
        rockShape,
        jetDirection,
        tower,
        currentBottomEdgeHeight
      );

      jetDirectionsIndex = (jetDirectionsIndex + 1) % jetDirections.length;
      if (!canLowerShape(rockShape, tower, currentBottomEdgeHeight)) {
        blocked = true;
        break;
      }
      currentBottomEdgeHeight--;
    }
    updateTower(tower, towerHeights, rockShape, currentBottomEdgeHeight);
  }

  // logTower(tower);
  console.log(Math.max(...towerHeights));
}

main();

// 3224
