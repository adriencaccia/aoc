import { readFileSync } from "fs";
import { chunk, clone } from "lodash-es";
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

const TOWER_HEIGHT = 10000;
const FALLING_ROCKS = BigInt(1_000_000_000_000);

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const jetDirections = input.split("") as (">" | "<")[];

  const towerHeights = Array.from({ length: 7 }, () => 0);
  const tower = Array.from({ length: TOWER_HEIGHT + 1 }, (_, index) =>
    Array.from({ length: 7 }, () => (index === 0 ? "⬛" : "⬜"))
  );
  tower.reverse();

  const addedRocks: [number, number][] = [];

  let jetDirectionsIndex = 0;
  let maxHeight = 0;
  let bigHeight = BigInt(0);
  for (let rockIndex = 1; rockIndex <= FALLING_ROCKS; rockIndex++) {
    const rockShapeIndex = (rockIndex - 1) % 5;
    let rockShape = clone(ROCKS[rockShapeIndex]);
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
    // logTower(tower);
    const currentMaxHeight = Math.max(...towerHeights);
    const gainedHeight = currentMaxHeight - maxHeight;
    addedRocks.push([rockShapeIndex, gainedHeight]);
    if (currentMaxHeight > maxHeight) {
      maxHeight = currentMaxHeight;
    }
    const repeatedPatternResponse = findRepeatedPattern(addedRocks);
    if (repeatedPatternResponse === undefined) {
      continue;
    }
    const { patternLength, pattern, remainder, heightGainedByPattern } =
      repeatedPatternResponse;

    const numberOfTimesToAddPattern =
      (FALLING_ROCKS - BigInt(rockIndex)) / BigInt(patternLength);
    bigHeight =
      BigInt(maxHeight) +
      BigInt(heightGainedByPattern) * numberOfTimesToAddPattern;
    // maxHeight += heightGainedByPattern * numberOfTimesToAddPattern;
    const remainingRocks =
      (FALLING_ROCKS - BigInt(remainder)) % BigInt(patternLength);
    // maxHeight += pattern
    bigHeight += pattern
      .reverse()
      .slice(0, Number(remainingRocks))
      .reduce((acc, [, heightGained]) => acc + BigInt(heightGained), BigInt(0));
    break;
  }

  console.log(Number(bigHeight));
}

function findRepeatedPattern(addedRocks: [number, number][]) {
  const reversedAddedRocks = clone(addedRocks).reverse();
  for (
    let patternLength = 5;
    patternLength < Math.floor(addedRocks.length / 2);
    patternLength++
  ) {
    const patterns = chunk(reversedAddedRocks, patternLength);
    const remainder = addedRocks.length % patternLength;
    if (remainder !== 0) {
      patterns.pop();
    }

    if (arePatternsEqual(patterns)) {
      const heightGainedByPattern = patterns[0].reduce(
        (acc, [, heightGained]) => acc + heightGained,
        0
      );
      return {
        patternLength,
        remainder,
        heightGainedByPattern,
        pattern: patterns[0],
      };
    }
  }
}

function arePatternsEqual(patterns: [number, number][][]) {
  for (let index = 0; index < patterns[0].length; index++) {
    for (
      let patternIndex = 0;
      patternIndex < patterns.length - 1;
      patternIndex++
    ) {
      if (
        areAddedRocksEqual(
          patterns[patternIndex][index],
          patterns[patternIndex + 1][index]
        )
      ) {
        continue;
      }
      return false;
    }
  }
  return true;
}

function areAddedRocksEqual(
  addedRock1: [number, number],
  addedRock2: [number, number]
) {
  return addedRock1[0] === addedRock2[0] && addedRock1[1] === addedRock2[1];
}

main();

// 1595988538691
