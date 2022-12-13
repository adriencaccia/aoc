import { deepEqual, deepStrictEqual } from "assert";
import { readFileSync } from "fs";
import { isEqual } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

type Signal = (number | Signal)[];

function compareSignals(
  signal1: Signal,
  signal2: Signal,
  depth = 0
): 1 | 0 | -1 {
  for (let i = 0; i < signal1.length; i++) {
    const firstElement = signal1[i];
    if (signal2[i] === undefined) {
      return 1;
    }
    const secondElement = signal2[i];

    if (typeof firstElement === "number" && typeof secondElement === "number") {
      if (firstElement < secondElement) {
        return -1;
      }
      if (firstElement > secondElement) {
        return 1;
      }
      continue;
    }
    if (typeof firstElement === "object" && typeof secondElement === "object") {
      const comparaison = compareSignals(
        firstElement,
        secondElement,
        depth + 1
      );
      if (comparaison === 0) {
        continue;
      }
      return comparaison;
    }

    if (typeof firstElement === "number") {
      const comparaison = compareSignals(
        [firstElement],
        secondElement as Signal,
        depth + 1
      );
      if (comparaison === 0) {
        continue;
      }
      return comparaison;
    }

    const comparaison = compareSignals(
      firstElement,
      [secondElement],
      depth + 1
    );
    if (comparaison === 0) {
      continue;
    }
    return comparaison;
  }
  if (signal1.length === signal2.length) {
    return depth === 0 ? 1 : 0;
  }
  return -1;
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const signalPairs = input
    .split("\n")
    .filter((line) => line !== "")
    .map((line) => JSON.parse(line) as Signal);

  signalPairs.push([[2]], [[6]]);

  signalPairs.sort(compareSignals);

  let firstIndex = 0;
  let secondIndex = 0;
  signalPairs.forEach((value, index) => {
    if (isEqual(value, [[2]])) {
      firstIndex = index + 1;
    }
    if (isEqual(value, [[6]])) {
      secondIndex = index + 1;
    }
  });

  console.log(firstIndex * secondIndex);
}

main();

// 22110
