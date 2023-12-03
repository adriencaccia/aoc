import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const caloriesList = input.split("\n").map(Number);
  caloriesList.pop();

  const elvesCalories: number[][] = [[]];
  for (const calories of caloriesList) {
    if (calories === 0) {
      elvesCalories.push([]);
      continue;
    }

    elvesCalories[elvesCalories.length - 1].push(calories);
  }

  const caloriesByElf = elvesCalories.map((elfCalories) =>
    elfCalories.reduce((a, b) => a + b)
  );

  caloriesByElf.sort((a, b) => a - b).reverse();

  console.log(caloriesByElf[0] + caloriesByElf[1] + caloriesByElf[2]);
}

main();

// 197400
