import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

interface Valve {
  flowRate: number;
  siblingValves: string[];
}

type Valves = Record<string, Valve>;
type ValvesToIndex = Map<string, number>;

function prepareInput(input: string) {
  return input.split("\n").reduce<Valves>((acc, line) => {
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
}

function getValvesToIndexMap(valves: Valves) {
  const valvesToIndex = new Map<keyof Valves, number>();
  const valvesArray = Object.keys(valves);
  for (let valveIndex = 0; valveIndex < valvesArray.length; valveIndex++) {
    valvesToIndex.set(valvesArray[valveIndex], valveIndex);
  }
  return valvesToIndex;
}

function getNonZeroValves(valves: Valves) {
  return Object.entries(valves).reduce<string[]>(
    (acc, [valve, { flowRate }]) => {
      if (flowRate === 0) {
        return acc;
      }
      acc.push(valve);
      return acc;
    },
    []
  );
}

/**
 * Find distances between all valves using the Floyd-Warshall algorithm
 */
function getDistanceBetweenValves(
  valves: Valves,
  valvesToIndex: ValvesToIndex
) {
  const numberOfValves = Object.keys(valves).length;
  const dist = Array.from({ length: numberOfValves }, () =>
    Array.from({ length: numberOfValves }, () => 1000)
  );

  // initialize the edges and the valves
  for (const [valve, { siblingValves }] of Object.entries(valves)) {
    const valveIndex = valvesToIndex.get(valve)!;
    for (const siblingValve of siblingValves) {
      const siblingValveIndex = valvesToIndex.get(siblingValve)!;
      dist[valveIndex][siblingValveIndex] = 1;
    }
    dist[valveIndex][valveIndex] = 0;
  }

  for (let k = 0; k < numberOfValves; k++) {
    for (let i = 0; i < numberOfValves; i++) {
      for (let j = 0; j < numberOfValves; j++) {
        if (dist[i][j] > dist[i][k] + dist[k][j]) {
          dist[i][j] = dist[i][k] + dist[k][j];
        }
      }
    }
  }

  return dist;
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const valves = prepareInput(input);

  const valvesToIndex = getValvesToIndexMap(valves);
  const dist = getDistanceBetweenValves(valves, valvesToIndex);

  const nonZeroValves = getNonZeroValves(valves);

  // dp[i][k][j] = maximum amount of pressure released at minute i, standing at
  //               location k, with the valves marked in bitset j opened
  const dp = Array.from({ length: 31 }, () =>
    Array.from({ length: nonZeroValves.length }, () =>
      Array.from(
        { length: 1 << nonZeroValves.length },
        () => Number.MIN_SAFE_INTEGER
      )
    )
  );

  // initialize the solutions accounting that "AA" is 0, (representing the decision of which valve to go to and open from the starting location)
  for (let i = 0; i < nonZeroValves.length; i++) {
    const distance =
      dist[valvesToIndex.get("AA")!][valvesToIndex.get(nonZeroValves[i])!];
    dp[distance + 1][i][1 << i] = 0;
  }

  const getFlow = (mask: number) => {
    let ans = 0;

    for (let i = 0; i < nonZeroValves.length; i++) {
      if (((1 << i) & mask) !== 0) {
        ans += valves[nonZeroValves[i]].flowRate;
      }
    }

    return ans;
  };

  let ans = 0;

  for (let i = 1; i < 31; i++) {
    for (let j = 0; j < 1 << nonZeroValves.length; j++) {
      for (let k = 0; k < nonZeroValves.length; k++) {
        const flow = getFlow(j);

        const hold = dp[i - 1][k][j] + flow;
        if (hold > dp[i][k][j]) {
          dp[i][k][j] = hold;
        }

        ans = Math.max(ans, dp[i][k][j]);

        if (((1 << k) & j) === 0) {
          continue;
        }

        for (let l = 0; l < nonZeroValves.length; l++) {
          if (((1 << l) & j) !== 0) {
            continue;
          }

          const distance =
            dist[valvesToIndex.get(nonZeroValves[k])!][
              valvesToIndex.get(nonZeroValves[l])!
            ];

          if (i + distance + 1 >= 31) {
            continue;
          }
          const value = dp[i][k][j] + flow * (distance + 1);
          if (value > dp[i + distance + 1][l][j | (1 << l)]) {
            dp[i + distance + 1][l][j | (1 << l)] = value;
          }
        }
      }
    }
  }

  let ans2 = 0;

  for (let i = 0; i < 1 << nonZeroValves.length; i++) {
    for (let j = 0; j < 1 << nonZeroValves.length; j++) {
      if ((i & j) !== j) {
        continue;
      }

      let a = -99999999;
      let b = -99999999;

      for (let k = 0; k < nonZeroValves.length; k++) {
        a = Math.max(a, dp[26][k][j]);
      }

      for (let k = 0; k < nonZeroValves.length; k++) {
        b = Math.max(b, dp[26][k][i & ~j]);
      }

      ans2 = Math.max(ans2, a + b);
    }
  }

  console.log(ans2);
}

main();

// 2679
