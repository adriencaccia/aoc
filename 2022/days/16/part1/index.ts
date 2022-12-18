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

// let maxPressure = 0;
// let maxPressurePath: [string, number][] = [];

function visitValve(
  valves: Valves,
  valveName: string,
  releasedPressure = 0,
  openedValves = new Set<string>(),
  valvesByMinute: [string, number][] = [],
  minute = 1
): number {
  const releasedPressureAtMinute = Array.from(openedValves).reduce(
    (acc, currentValve) => acc + valves[currentValve].flowRate,
    0
  );
  valvesByMinute.push([valveName, releasedPressureAtMinute]);
  // console.log(releasedPressureAtMinute);
  releasedPressure += releasedPressureAtMinute;
  if (minute === 30) {
    // console.log(valvesByMinute, releasedPressure);
    // if (releasedPressure > maxPressure) {
    //   maxPressure = releasedPressure;
    //   maxPressurePath = valvesByMinute;
    // }
    return releasedPressure;
  }

  const valve = valves[valveName];
  const siblingValvesToCheck = valve.siblingValves.filter(
    (siblingValve) => siblingValve !== valvesByMinute.at(-2)?.[0]
    // {
    //   for (
    //     let valveIndex = valvesByMinute.length - 1;
    //     valveIndex >= 1;
    //     valveIndex--
    //   ) {
    //     if (valvesByMinute[valveIndex][0] === valvesByMinute[valveIndex - 1][0]) {
    //       return true;
    //     }
    //     if (
    //       valveIndex !== valvesByMinute.length - 1 &&
    //       siblingValve === valvesByMinute[valveIndex][0]
    //     ) {
    //       return false;
    //     }
    //   }
    //   return true;
    // }
  );
  if (valve.flowRate === 0) {
    return Math.max(
      ...siblingValvesToCheck.map((siblingValveName) =>
        visitValve(
          valves,
          siblingValveName,
          releasedPressure,
          clone(openedValves),
          clone(valvesByMinute),
          minute + 1
        )
      )
    );
  }

  return Math.max(
    // open the valve if it is not already opened
    ...(openedValves.has(valveName)
      ? []
      : [
          visitValve(
            valves,
            valveName,
            releasedPressure,
            clone(clone(openedValves).add(valveName)),
            clone(valvesByMinute),
            minute + 1
          ),
        ]),
    // visit the other valves
    ...siblingValvesToCheck.map((siblingValveName) =>
      visitValve(
        valves,
        siblingValveName,
        releasedPressure,
        clone(openedValves),
        clone(valvesByMinute),
        minute + 1
      )
    )
  );
}

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

  const maxReleasedPressure = visitValve(valves, "AA");

  console.log(maxReleasedPressure);
  // console.log(maxPressure, maxPressurePath);
}

main();

// 4907780
