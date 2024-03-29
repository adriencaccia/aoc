import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

interface Instruction {
  command: "forward" | "down" | "up";
  value: number;
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const instructions = input.split("\n").map((instructionLine) => {
    const [command, value] = instructionLine.split(" ");

    const instruction: Instruction = {
      command: command as Instruction["command"],
      value: Number(value),
    };

    return instruction;
  });

  const { depth, horizontal } = instructions.reduce(
    (acc, instruction) => {
      switch (instruction.command) {
        case "down":
          acc.aim += instruction.value;
          break;
        case "up":
          acc.aim -= instruction.value;
          break;
        case "forward":
          acc.horizontal += instruction.value;
          acc.depth += acc.aim * instruction.value;
          break;
      }

      return acc;
    },
    { depth: 0, horizontal: 0, aim: 0 }
  );
  const answer = depth * horizontal;

  console.log(answer);
}

main();

// 1594785890
