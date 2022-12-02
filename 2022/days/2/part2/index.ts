import { readFileSync } from "fs";
import { join } from "path";

function getRoundResult({
  elfSign,
  playerSign,
}: {
  elfSign: string;
  playerSign: string;
}) {
  switch (elfSign) {
    case "A":
      {
        switch (playerSign) {
          case "X":
            return 0 + 3;
          case "Y":
            return 3 + 1;
          case "Z":
            return 6 + 2;
        }
      }

      return 0;

    case "B":
      {
        switch (playerSign) {
          case "X":
            return 0 + 1;
          case "Y":
            return 3 + 2;
          case "Z":
            return 6 + 3;
        }
      }

      return 0;

    case "C":
      {
        switch (playerSign) {
          case "X":
            return 0 + 2;
          case "Y":
            return 3 + 3;
          case "Z":
            return 6 + 1;
        }
      }

      return 0;
  }

  return 0;
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const rounds = input.split("\n").map((line) => {
    const [elfSign, playerSign] = line.split(" ");

    return { elfSign, playerSign };
  });

  const score = rounds.reduce((currentScore, { elfSign, playerSign }) => {
    return currentScore + getRoundResult({ elfSign, playerSign });
  }, 0);

  console.log(score);
}

main();

// 9541
