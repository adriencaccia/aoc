import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

type Operation = ["old", "+" | "*", string];

function calculateNewWorryLevel(
  worryLevel: number,
  [, symbol, operand]: ["old", "+" | "*", string]
) {
  const newOperand = operand === "old" ? worryLevel : Number(operand);
  if (symbol === "*") {
    return worryLevel * newOperand;
  }
  return worryLevel + newOperand;
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const monkeysInput = input
    .split("\n\n")
    .map((monkeyLines) => monkeyLines.split("\n"));

  const monkeys = monkeysInput.map((monkey) => {
    monkey.shift();
    const items = monkey[0].slice(18).split(", ").map(Number);
    const operation = monkey[1].slice(19).split(" ") as Operation;
    const divisibleByTest = Number(monkey[2].slice(21));
    const throwToMonkey = {
      true: Number(monkey[3].slice(29)),
      false: Number(monkey[4].slice(30)),
    };

    return {
      items,
      operation,
      divisibleByTest,
      throwToMonkey,
    };
  });

  const leastCommonMultiple = monkeys.reduce(
    (factor, { divisibleByTest }) => factor * divisibleByTest,
    1
  );

  const inspectedItemsByMonkeys = Array(monkeys.length)
    .fill(0)
    .map((_) => 0);

  for (let round = 0; round < 10000; round++) {
    monkeys.forEach(
      ({ divisibleByTest, items, operation, throwToMonkey }, monkeyIndex) => {
        while (items.length !== 0) {
          const itemToThrow = items.shift() ?? 0;
          const newWorryLevel =
            calculateNewWorryLevel(itemToThrow, operation) %
            leastCommonMultiple;
          const isDivisibleByTest = newWorryLevel % divisibleByTest === 0;
          monkeys[
            isDivisibleByTest ? throwToMonkey.true : throwToMonkey.false
          ].items.push(newWorryLevel);
          inspectedItemsByMonkeys[monkeyIndex]++;
        }
      }
    );
  }

  inspectedItemsByMonkeys.sort((a, b) => b - a);

  const monkeyBusiness =
    inspectedItemsByMonkeys[0] * inspectedItemsByMonkeys[1];
  console.log(monkeyBusiness);
}

main();

// 30616425600
