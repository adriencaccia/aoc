import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

const CLOSING_CHARACTERS_MAP = {
  ")": { score: 3, openingCharacter: "(" },
  "]": { score: 57, openingCharacter: "[" },
  "}": { score: 1197, openingCharacter: "{" },
  ">": { score: 25137, openingCharacter: "<" },
} as const;

const OPENING_CHARACTERS_LIST = ["(", "[", "{", "<"] as const;

const CLOSING_CHARACTER_LIST = [")", "]", "}", ">"] as const;

const isOpeningCharacter = (
  character: Character
): character is OpeningCharacter =>
  OPENING_CHARACTERS_LIST.includes(character as OpeningCharacter);

type OpeningCharacter = typeof OPENING_CHARACTERS_LIST[number];
type ClosingCharacter = typeof CLOSING_CHARACTER_LIST[number];

type Character = OpeningCharacter | ClosingCharacter;

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");
  lines.pop();
  const characterMatrix = lines.map((line) =>
    line.split("").map((char) => char as Character)
  );

  let answer = 0;
  for (const characterLine of characterMatrix) {
    const characterQueue = new Array<Character>();
    for (const character of characterLine) {
      if (isOpeningCharacter(character)) {
        characterQueue.push(character);
        continue;
      }
      const openingCharacter = characterQueue.pop();
      if (openingCharacter === undefined) {
        throw new Error("No opening character found");
      }
      if (!isOpeningCharacter(openingCharacter)) {
        throw new Error("Invalid character found");
      }
      if (
        openingCharacter !== CLOSING_CHARACTERS_MAP[character].openingCharacter
      ) {
        answer += CLOSING_CHARACTERS_MAP[character].score;
      }
    }
  }

  console.log(answer);
}

main();

// 390993
