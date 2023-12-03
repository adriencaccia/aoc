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
    .map((line) => (line.match(/(-|\d)+/g) ?? []).map(Number))
    .map(([xSensor, ySensor, xBeacon, yBeacon]) => {
      return {
        xSensor,
        ySensor,
        yBeacon,
        xBeacon,
        distance: computeManhattanDistance(xSensor, ySensor, xBeacon, yBeacon),
      };
    });

  function isBeaconPossiblyPresent(x: number, y: number) {
    for (const { distance, xSensor, ySensor, xBeacon, yBeacon } of sensors) {
      if (x === xBeacon && y === yBeacon) {
        // there is already a beacon here
        return false;
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
      return false;
    }
    return true;
  }

  const searchSize = 4_000_000;

  for (const { distance, xSensor, ySensor } of sensors) {
    let currentYAbove = ySensor;
    let currentYBelow = ySensor;
    for (
      let currentX = xSensor - distance - 1;
      currentX <= xSensor + distance + 1;
      currentX++
    ) {
      if (
        currentX < 0 ||
        currentYAbove < 0 ||
        currentX > searchSize ||
        currentYAbove > searchSize
      ) {
        currentYAbove -= Math.sign(xSensor - currentX);
        currentYBelow += Math.sign(xSensor - currentX);
        continue;
      }
      const isBeaconPossiblyPresentAbove = isBeaconPossiblyPresent(
        currentX,
        currentYAbove
      );
      if (isBeaconPossiblyPresentAbove) {
        const tuningFrequency = currentX * 4_000_000 + currentYAbove;
        console.log(tuningFrequency);
        return;
      }
      currentYBelow = ySensor + Math.abs(currentX - (xSensor + distance));

      if (
        currentX < 0 ||
        currentYBelow < 0 ||
        currentX > searchSize ||
        currentYBelow > searchSize
      ) {
        currentYAbove -= Math.sign(xSensor - currentX);
        currentYBelow += Math.sign(xSensor - currentX);
        continue;
      }
      const isBeaconPossiblyPresentBelow = isBeaconPossiblyPresent(
        currentX,
        currentYBelow
      );

      if (isBeaconPossiblyPresentBelow) {
        const tuningFrequency = currentX * 4_000_000 + currentYBelow;
        console.log(tuningFrequency);
        return;
      }
      currentYAbove -= Math.sign(xSensor - currentX);
      currentYBelow += Math.sign(xSensor - currentX);
    }
  }
}

main();

// 13639962836448
