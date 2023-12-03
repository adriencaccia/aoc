import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function computeManhattanDistance(
  x1: number,
  y1: number,
  x2: number,
  y2: number
) {
  return Math.abs(x1 - x2) + Math.abs(y1 - y2);
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const sensors = input
    .split("\n")
    .map((line) => (line.match(/\d+/g) ?? []).map(Number))
    .map(([xSensor, ySensor, xBeacon, yBeacon]) => {
      return {
        xSensor,
        ySensor,
        yBeacon,
        xBeacon,
        distance: computeManhattanDistance(xSensor, ySensor, xBeacon, yBeacon),
      };
    });

  function isBeaconNotPossiblyPresent(x: number, y: number) {
    for (const { distance, xSensor, ySensor, xBeacon, yBeacon } of sensors) {
      if (x === xBeacon && y === yBeacon) {
        // there is already a beacon here
        return 0;
      }
      const sensorDistanceWithPossibleBeacon = computeManhattanDistance(
        x,
        y,
        xSensor,
        ySensor
      );

      if (sensorDistanceWithPossibleBeacon > distance) {
        continue;
      }
      return 1;
    }
    return 0;
  }

  let y = 2_000_000;
  let possibleBeaconPositions = 0;
  for (let x = -1_000_000; x < 5_000_000; x++) {
    possibleBeaconPositions += isBeaconNotPossiblyPresent(x, y);
  }

  console.log(possibleBeaconPositions);
}

main();

// 4907780
