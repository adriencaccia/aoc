import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

const ADJACENT_CAVES: Record<string, string[]> = {};

function isSmallCave(cave: string): boolean {
  return cave.toUpperCase() !== cave;
}

function visitCaves(
  cave: string,
  visitedSmallCavesMap: Map<string, number>
): number {
  if (
    (visitedSmallCavesMap.get(cave) ?? 0) > 1 ||
    (visitedSmallCavesMap.get(cave) === 1 &&
      Array.from(visitedSmallCavesMap.values()).includes(2))
  ) {
    return 0;
  }

  if (isSmallCave(cave)) {
    visitedSmallCavesMap.set(cave, (visitedSmallCavesMap.get(cave) ?? 0) + 1);
  }

  return ADJACENT_CAVES[cave].reduce((acc, newCave) => {
    if (newCave === "end") {
      return acc + 1;
    }

    return acc + visitCaves(newCave, new Map(visitedSmallCavesMap));
  }, 0);
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");
  lines.pop();
  lines.forEach((line) => {
    const [leftCave, rightCave] = line.split("-");
    if (leftCave !== "end" && rightCave !== "start") {
      ADJACENT_CAVES[leftCave] = [
        // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
        ...(ADJACENT_CAVES[leftCave] ?? []),
        rightCave,
      ];
    }
    if (rightCave !== "end" && leftCave !== "start") {
      ADJACENT_CAVES[rightCave] = [
        // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
        ...(ADJACENT_CAVES[rightCave] ?? []),
        leftCave,
      ];
    }
  });

  const answer = visitCaves("start", new Map());

  console.log(answer);
}

main();

// 93572
