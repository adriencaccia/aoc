import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

interface BitCount {
  zero: number;
  one: number;
}
function getBitCounts(diagnostics: string[]) {
  const diagnosticLength = diagnostics[0].length;

  const bitCounts = diagnostics.reduce(
    (acc, diagnostic) => {
      for (let i = 0; i < diagnosticLength; i++) {
        if (diagnostic[i] === "0") {
          acc[i].zero++;
        } else {
          acc[i].one++;
        }
      }

      return acc;
    },
    Array(diagnosticLength)
      .fill(0)
      .map(() => ({ zero: 0, one: 0 } as BitCount))
  );

  return bitCounts;
}

function getRating(
  currentDiagnostics: string[],
  criteria: "oxygen" | "co2",
  index = 0
): string {
  if (currentDiagnostics.length === 1) {
    return currentDiagnostics[0];
  }
  const bitCounts = getBitCounts(currentDiagnostics);
  const mostCommonBit =
    bitCounts[index].one >= bitCounts[index].zero ? "1" : "0";
  const criteriaBit =
    criteria === "oxygen"
      ? mostCommonBit
      : (1 - parseInt(mostCommonBit, 10)).toString();
  const filteredDiagnostics = currentDiagnostics.filter(
    (diagnostic) => diagnostic[index] === criteriaBit
  );

  return getRating(filteredDiagnostics, criteria, index + 1);
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const diagnostics = input.split("\n");

  const oxygenCriteriaRating = parseInt(getRating(diagnostics, "oxygen"), 2);
  const co2CriteriaRating = parseInt(getRating(diagnostics, "co2"), 2);

  const answer = oxygenCriteriaRating * co2CriteriaRating;

  console.log(answer);
}

main();

// 1800151
