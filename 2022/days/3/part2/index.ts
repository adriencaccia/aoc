import { readFileSync } from "fs";
import { chunk, intersection } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const rucksacks = input.split("\n").map((rucksack) => rucksack.split(""));
  rucksacks.pop();

  const threeRucksacksGroups = chunk(rucksacks, 3);

  const answer = threeRucksacksGroups.reduce((sum, threeRucksacks) => {
    const [itemPresentInAllRucksacks] = intersection(...threeRucksacks);

    const itemCharCode = itemPresentInAllRucksacks.charCodeAt(0);

    return sum + (itemCharCode >= 97 ? itemCharCode - 96 : itemCharCode - 38);
  }, 0);

  console.log(answer);
}

main();

// 2425
