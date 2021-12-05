import { readFileSync } from "fs";
import { join } from "path";

interface BitCount {
  zero: number;
  one: number;
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const diagnostics = input.split("\n");
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

  const gammaRateBinary = bitCounts
    .map((bitCount) => (bitCount.one > bitCount.zero ? "1" : "0"))
    .join("");

  const epsilonRateBinary = bitCounts
    .map((bitCount) => (bitCount.one > bitCount.zero ? "0" : "1"))
    .join("");

  const gammaRate = parseInt(gammaRateBinary, 2);
  const epsilonRate = parseInt(epsilonRateBinary, 2);

  const answer = gammaRate * epsilonRate;

  console.log(answer);
}

main();

// 4139586
