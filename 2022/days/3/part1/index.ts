import { readFileSync } from "fs";
import { chunk, intersection } from "lodash";
import { join } from "path";

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const rucksacks = input.split("\n");
  rucksacks.pop();

  const answer = rucksacks.reduce((sum, rucksack) => {
    const compartments = chunk(rucksack, rucksack.length / 2);
    const [itemPresentInBothCompartment] = intersection(...compartments);

    const itemCharCode = itemPresentInBothCompartment.charCodeAt(0);

    return sum + (itemCharCode >= 97 ? itemCharCode - 96 : itemCharCode - 38);
  }, 0);

  console.log(answer);
}

main();

// 8053
