import { readFileSync } from "fs";
import { join } from "path";

const ADJACENT_CAVES: Record<string, string[]> = {};

function isSmallCave(cave: string): boolean {
  return cave.toUpperCase() !== cave;
}

function visitCaves(cave: string, visited: Set<string>): number {
  if (isSmallCave(cave) && visited.has(cave)) {
    return 0;
  }
  visited.add(cave);

  return ADJACENT_CAVES[cave].reduce((acc, newCave) => {
    if (newCave === "end") {
      return acc + 1;
    }

    return acc + visitCaves(newCave, new Set(visited));
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

  const answer = visitCaves("start", new Set());

  console.log(answer);
}

main();

// 3298
