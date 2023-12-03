import { readFileSync } from "fs";
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
      return -1;
    }
    const secondElement = signal2[i];

    if (typeof firstElement === "number" && typeof secondElement === "number") {
      if (firstElement < secondElement) {
        return 1;
      }
      if (firstElement > secondElement) {
        return -1;
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
  return 1;
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const signalPairs = input.split("\n\n").map((lines) => {
    const [signal1, signal2] = lines.split("\n");
    return [JSON.parse(signal1), JSON.parse(signal2)] as [Signal, Signal];
  });

  let rightOrderSum = 0;

  for (
    let signalPairsIndex = 0;
    signalPairsIndex < signalPairs.length;
    signalPairsIndex++
  ) {
    const [signal1, signal2] = signalPairs[signalPairsIndex];

    const comparaison = compareSignals(signal1, signal2);
    if (comparaison === 1) {
      rightOrderSum += signalPairsIndex + 1;
    }
  }
  console.log(rightOrderSum);
}

main();

// 5659
