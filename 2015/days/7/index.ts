import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

type Instruction =
  | {
      name: "wire-to-wire";
      source: string;
    }
  | {
      name: "value-to-wire";
      value: number;
    }
  | {
      name: "and-gate";
      sourceA: string;
      sourceB: string;
    }
  | {
      name: "or-gate";
      sourceA: string;
      sourceB: string;
    }
  | {
      name: "l-shift";
      source: string;
      shift: number;
    }
  | {
      name: "r-shift";
      source: string;
      shift: number;
    }
  | {
      name: "not-gate";
      source: string;
    };

function parseInput(input: string) {
  const lines = input.split("\n");
  const instructionsMap = new Map<string, Instruction>();

  for (const line of lines) {
    const [, wire] = line.match(/-> (\w+)/)!;

    const [, wireToWireSource] = line.match(/^([a-z]+) ->/) ?? [];
    if (wireToWireSource) {
      const instruction: Instruction = {
        name: "wire-to-wire",
        source: wireToWireSource,
      };
      instructionsMap.set(wire, instruction);
      continue;
    }
    const [, sourceValue] = line.match(/^(\d+) ->/) ?? [];
    if (sourceValue) {
      const instruction: Instruction = {
        name: "value-to-wire",
        value: Number(sourceValue),
      };
      instructionsMap.set(wire, instruction);
      continue;
    }

    const [, andGateSourceA, andGateSourceB] =
      line.match(/(\w+) AND (\w+)/) ?? [];
    if (andGateSourceA && andGateSourceB) {
      const instruction: Instruction = {
        name: "and-gate",
        sourceA: andGateSourceA,
        sourceB: andGateSourceB,
      };
      instructionsMap.set(wire, instruction);
      continue;
    }

    const [, orGateSourceA, orGateSourceB] = line.match(/(\w+) OR (\w+)/) ?? [];
    if (orGateSourceA && orGateSourceB) {
      const instruction: Instruction = {
        name: "or-gate",
        sourceA: orGateSourceA,
        sourceB: orGateSourceB,
      };
      instructionsMap.set(wire, instruction);
      continue;
    }

    const [, lShiftSource, lShift] = line.match(/(\w+) LSHIFT (\d+)/) ?? [];
    if (lShiftSource && lShift) {
      const instruction: Instruction = {
        name: "l-shift",
        source: lShiftSource,
        shift: Number(lShift),
      };
      instructionsMap.set(wire, instruction);
      continue;
    }

    const [, rShiftSource, rShift] = line.match(/(\w+) RSHIFT (\d+)/) ?? [];
    if (rShiftSource && rShift) {
      const instruction: Instruction = {
        name: "r-shift",
        source: rShiftSource,
        shift: Number(rShift),
      };
      instructionsMap.set(wire, instruction);
      continue;
    }

    const [, notSource] = line.match(/NOT (\w+)/) ?? [];
    if (notSource) {
      const instruction: Instruction = {
        name: "not-gate",
        source: notSource,
      };
      instructionsMap.set(wire, instruction);
      continue;
    }
  }

  console.log(instructionsMap);
  return instructionsMap;
}

function traverseInstructions(
  instructionsMap: ReturnType<typeof parseInput>,
  wire = "a"
): number {
  const instruction = instructionsMap.get(wire);
  if (instruction === undefined) {
    throw new Error(`${wire} instruction not defined`);
  }
  if (instruction.name === "value-to-wire") {
    return instruction.value;
  }
  if (instruction.name === "wire-to-wire") {
    return traverseInstructions(instructionsMap, instruction.source);
  }
  if (instruction.name === "not-gate") {
    return ~traverseInstructions(instructionsMap, instruction.source);
  }
  if (instruction.name === "l-shift") {
    return (
      traverseInstructions(instructionsMap, instruction.source) <<
      instruction.shift
    );
  }
  if (instruction.name === "r-shift") {
    return (
      traverseInstructions(instructionsMap, instruction.source) <<
      instruction.shift
    );
  }
  if (instruction.name === "and-gate") {
    return (
      traverseInstructions(instructionsMap, instruction.sourceA) &
      traverseInstructions(instructionsMap, instruction.sourceB)
    );
  }
  if (instruction.name === "or-gate") {
    return (
      traverseInstructions(instructionsMap, instruction.sourceA) |
      traverseInstructions(instructionsMap, instruction.sourceB)
    );
  }
  return 0;
}

function goA(parsedInput: ReturnType<typeof parseInput>) {
  return traverseInstructions(parsedInput);
}

function goB(parsedInput: ReturnType<typeof parseInput>) {
  return parsedInput;
}

function main() {
  const input = readFileSync(join(__dirname, "input.txt"), "utf8").trim();
  const parsedInput = parseInput(input);

  const answerA = goA(parsedInput);
  console.log("Part 1");
  console.log(answerA);

  const answerB = goB(parsedInput);
  console.log("Part 2");
  console.log(answerB);
}

main();

// ???
// ???
