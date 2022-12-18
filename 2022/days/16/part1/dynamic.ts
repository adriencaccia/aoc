import { readFileSync } from "fs";
import { clone } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

interface Valve {
  flowRate: number;
  siblingValves: string[];
}

type Valves = Record<string, Valve>;

type Solution = { openedValves: string[]; releasedPressure: number };

function main() {
  const input = readFileSync(join(__dirname, "../input-simple.txt"), "utf8");
  const valves = input.split("\n").reduce<Valves>((acc, line) => {
    const [, valveName, flowRateString, siblingValvesString] =
      line.match(
        /Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)/
      ) ?? [];
    acc[valveName] = {
      flowRate: Number(flowRateString),
      siblingValves: siblingValvesString.split(", "),
    };
    return acc;
  }, {});

  const initialObject = Object.keys(valves).reduce<Record<string, Solution>>(
    (acc, valveName) => {
      acc[valveName] = {
        openedValves: [],
        releasedPressure: 0,
      };

      return acc;
    },
    {}
  );

  const solutions = Array.from({ length: 30 }, (_) => clone(initialObject));
  console.log(valves);
  console.log(solutions);
}

main();

// 4907780
