import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");

  const [stacksInput, instructionsInput] = input
    .split("\n\n")
    .map((group) => group.split("\n"));

  const numberOfStacks = Number(stacksInput.at(-1)?.slice(-2)) ?? 0;
  stacksInput.pop();

  const reversedStacks = Array(numberOfStacks)
    .fill(0)
    .map((_) => "");

  for (const stackInput of stacksInput) {
    for (let i = 0; i < numberOfStacks; i++) {
      const crate = stackInput.at(i * 4 + 1);
      if (crate === " ") continue;
      reversedStacks[i] += crate;
    }
  }

  const stacks = reversedStacks.map((stack) => stack.split("").reverse());

  const instructions = instructionsInput.map((instruction) => {
    return instruction
      .split(" ")
      .map(Number)
      .filter((item) => !isNaN(item));
  });

  for (const [
    numberOfCratesToMove,
    stackToMoveFrom,
    stackToMoveTo,
  ] of instructions) {
    const stackToMoveFromLength = stacks[stackToMoveFrom - 1].length;
    const cratesToMove = stacks[stackToMoveFrom - 1].splice(
      stackToMoveFromLength - numberOfCratesToMove,
      numberOfCratesToMove
    );

    const stackToMoveToLength = stacks[stackToMoveTo - 1].length;
    stacks[stackToMoveTo - 1].splice(stackToMoveToLength, 0, ...cratesToMove);
  }

  const answer = stacks.map((stack) => stack.at(-1)).join("");
  console.log(answer);
}

main();

// WDLPFNNNB
