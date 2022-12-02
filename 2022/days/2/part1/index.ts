import { readFileSync } from "fs";
import { join } from "path";

function getPlayerSignScore(sign: string) {
  switch (sign) {
    case "X":
      return 1;
    case "Y":
      return 2;
    case "Z":
      return 3;
  }

  return 0;
}

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
            return 3;
          case "Y":
            return 6;
          case "Z":
            return 0;
        }
      }

      return 0;

    case "B":
      {
        switch (playerSign) {
          case "X":
            return 0;
          case "Y":
            return 3;
          case "Z":
            return 6;
        }
      }

      return 0;

    case "C":
      {
        switch (playerSign) {
          case "X":
            return 6;
          case "Y":
            return 0;
          case "Z":
            return 3;
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
    return (
      currentScore +
      getRoundResult({ elfSign, playerSign }) +
      getPlayerSignScore(playerSign)
    );
  }, 0);

  console.log(score);
}

main();

// 10595
