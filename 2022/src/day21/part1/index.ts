import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

interface Job {
  operation: "+" | "-" | "*" | "/";
  left: string;
  right: string;
}

type Monkeys = Record<string, number | Job>;

function prepareInput(input: string) {
  const monkeys: Monkeys = {};
  for (const line of input.split("\n")) {
    const matches = line.match(/(\w+): (((\w+) (\+|-|\*|\/) (\w+))|(\d+))/);
    if (matches === null) {
      continue;
    }
    if (matches[7] === undefined) {
      monkeys[matches[1]] = {
        operation: matches[5] as Job["operation"],
        left: matches[4],
        right: matches[6],
      };
      continue;
    }
    monkeys[matches[1]] = Number(matches[7]);
  }
  return monkeys;
}

function calculate(monkeys: Monkeys, name = "root"): number {
  const monkey = monkeys[name];
  if (typeof monkey === "number") {
    return monkey;
  }
  const { left, operation, right } = monkey;

  switch (operation) {
    case "*":
      return calculate(monkeys, left) * calculate(monkeys, right);
    case "+":
      return calculate(monkeys, left) + calculate(monkeys, right);
    case "-":
      return calculate(monkeys, left) - calculate(monkeys, right);
    case "/":
      return calculate(monkeys, left) / calculate(monkeys, right);
  }
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const monkeys = prepareInput(input);

  const answer = calculate(monkeys);

  console.log(answer);
}

main();

// 104272990112064
