import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

interface Job {
  operation: "+" | "-" | "*" | "/" | "=";
  left: string;
  right: string;
}

type Monkeys = Record<string, number | "humn" | Job>;

function prepareInput(input: string) {
  const monkeys: Monkeys = {};
  for (const line of input.split("\n")) {
    const matches = line.match(/(\w+): (((\w+) (\+|-|\*|\/) (\w+))|(\d+))/);
    if (matches === null) {
      continue;
    }
    const name = matches[1];
    if (name === "root") {
      monkeys[name] = {
        operation: "=",
        left: matches[4],
        right: matches[6],
      };
      continue;
    }
    if (name === "humn") {
      monkeys[name] = "humn";
      continue;
    }
    if (matches[7] === undefined) {
      monkeys[name] = {
        operation: matches[5] as Job["operation"],
        left: matches[4],
        right: matches[6],
      };
      continue;
    }
    monkeys[name] = Number(matches[7]);
  }
  return monkeys;
}

function calculate(monkeys: Monkeys, humnValue: number, name: string): number {
  const monkey = monkeys[name];
  if (typeof monkey === "number") {
    return monkey;
  }

  if (monkey === "humn") {
    return humnValue;
  }

  const { left, operation, right } = monkey;

  switch (operation) {
    case "*":
      return (
        calculate(monkeys, humnValue, left) *
        calculate(monkeys, humnValue, right)
      );
    case "+":
      return (
        calculate(monkeys, humnValue, left) +
        calculate(monkeys, humnValue, right)
      );
    case "-":
      return (
        calculate(monkeys, humnValue, left) -
        calculate(monkeys, humnValue, right)
      );
    case "/":
      return (
        calculate(monkeys, humnValue, left) /
        calculate(monkeys, humnValue, right)
      );
    default:
      return 0;
  }
}

function printEquation(monkeys: Monkeys, name = "root"): string {
  const monkey = monkeys[name];
  if (typeof monkey === "number") {
    return monkey.toString();
  }
  if (monkey === "humn") {
    return "humn";
  }
  return (
    printEquation(monkeys, monkey.left) +
    monkey.operation +
    printEquation(monkeys, monkey.right)
  );
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const monkeys = prepareInput(input);

  const equation = printEquation(monkeys);

  const equalSignIndex = equation.indexOf("=");
  const humnIndex = equation.indexOf("humn");
  const { left, right } = monkeys["root"] as Job;

  const sideWithoutHumnValue = calculate(
    monkeys,
    0,
    humnIndex < equalSignIndex ? right : left
  );

  let min = -Number.MAX_SAFE_INTEGER;
  let max = Number.MAX_SAFE_INTEGER;

  let sideWithHumnValue = calculate(
    monkeys,
    (min + max) / 2,
    humnIndex < equalSignIndex ? left : right
  );

  while (sideWithHumnValue !== sideWithoutHumnValue) {
    const newHumnValue = (min + max) / 2;
    const value = calculate(
      monkeys,
      newHumnValue,
      humnIndex < equalSignIndex ? left : right
    );
    if (value === sideWithoutHumnValue) {
      console.log(newHumnValue);
      break;
    }

    min = value > sideWithoutHumnValue ? newHumnValue : min;
    max = value > sideWithoutHumnValue ? max : newHumnValue;
  }
}

main();

// 3220993874133
