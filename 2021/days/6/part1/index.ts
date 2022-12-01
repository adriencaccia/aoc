import { readFileSync } from "fs";
import { join } from "path";

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n").slice(0);
  const inputs = lines[0]
    .split(",")
    .map((number) => ({ value: parseInt(number, 10), isNew: false }));

  for (let i = 0; i < 80; i++) {
    let newNumbers = 0;
    inputs.forEach((element) => {
      if (element.value === 0) {
        newNumbers++;
        element.value = 6;

        return;
      }
      element.value--;
    });
    for (let index = 0; index < newNumbers; index++) {
      inputs.push({ value: 8, isNew: true });
    }
  }

  const answer = inputs.length;
  console.log(answer);
}

main();

// 356190
