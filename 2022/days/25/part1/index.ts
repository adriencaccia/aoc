import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function prepareInput(input: string) {
  return input.split("\n").map((line) => line.split(""));
}

function snafuDigitToNumber(digit: string) {
  switch (digit) {
    case "-":
      return -1;
    case "=":
      return -2;
    default:
      return Number(digit);
  }
}

export function toNumber(snafu: string[]) {
  return snafu.reverse().reduce((acc, digit, index) => {
    return acc + snafuDigitToNumber(digit) * 5 ** index;
  }, 0);
}

function toSnafu(num: number): string[] {
  const digits = [];
  while (num > 0) {
    num += 2;
    digits.unshift(num % 5);
    num = Math.floor(num / 5);
  }
  return digits.map((digit) => "=-012".charAt(digit));
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const snafus = prepareInput(input);

  const sum = snafus.reduce((acc, snafu) => acc + toNumber(snafu), 0);

  console.log(toSnafu(sum).join(""));
}

main();

// 2--1=0=-210-1=00=-=1
