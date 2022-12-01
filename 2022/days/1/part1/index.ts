import { readFileSync } from "fs";
import { join } from "path";

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

  const maxCalories = elvesCalories.reduce(
    (currentMaxCalories, elfCalories) => {
      const elfCaloriesSum = elfCalories.reduce((a, b) => a + b);

      return elfCaloriesSum > currentMaxCalories
        ? elfCaloriesSum
        : currentMaxCalories;
    },
    0
  );

  console.log(maxCalories);
}

main();

// 69206
