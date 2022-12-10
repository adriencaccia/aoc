import { readFileSync } from "fs";
import { chunk } from "lodash-es";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function addPixel(
  pixels: string[],
  cycleNumber: number,
  currentRegisterValue: number
) {
  const pixelBeingDrawn = cycleNumber % 40;
  if (
    [
      currentRegisterValue,
      currentRegisterValue + 1,
      currentRegisterValue + 2,
    ].includes(pixelBeingDrawn)
  ) {
    pixels.push("⬜");
    return;
  }
  return pixels.push("⬛");
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");

  const cycles: number[] = [1];
  const pixels: string[] = ["⬜"];
  let cycleNumber = 1;

  for (const line of lines) {
    const currentRegisterValue = cycles.at(-1) ?? 1;
    if (line === "noop") {
      cycles.push(currentRegisterValue);
      cycleNumber += 1;
      addPixel(pixels, cycleNumber, currentRegisterValue);
      continue;
    }
    const [, stringValue] = line.split(" ");

    cycles.push(currentRegisterValue);
    cycleNumber += 1;
    addPixel(pixels, cycleNumber, currentRegisterValue);
    cycles.push(currentRegisterValue + Number(stringValue));
    cycleNumber += 1;
    addPixel(pixels, cycleNumber, currentRegisterValue + Number(stringValue));
  }
  pixels.pop();
  const rows = chunk(pixels, 40);

  for (const row of rows) {
    console.log(row.join(""));
  }
}

main();

// EGLHBLFJ
