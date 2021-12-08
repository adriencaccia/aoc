import { readFileSync } from "fs";
import { difference, find, flatten, intersection, uniq } from "lodash";
import { join } from "path";

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");
  lines.pop();
  const values = flatten(
    lines.map((line) => {
      const [inputValues, outputValues] = line.split(" | ");

      return {
        inputValues: inputValues.split(" "),
        outputValues: outputValues.split(" "),
      };
    })
  );

  let answer = 0;
  for (const { inputValues, outputValues } of values) {
    const numbers: Record<number, string[]> = {};
    const sixSegments = [];

    for (const inputValue of inputValues) {
      const letters = inputValue.split("");
      if (inputValue.length === 2) {
        numbers[1] = letters;
        continue;
      }
      if (inputValue.length === 3) {
        numbers[7] = letters;
        continue;
      }
      if (inputValue.length === 4) {
        numbers[4] = letters;
        continue;
      }
      if (inputValue.length === 6) {
        sixSegments.push(letters);
        continue;
      }
      if (inputValue.length === 7) {
        numbers[8] = letters;
        continue;
      }
    }

    const firstSegment = difference(numbers[7], numbers[1]);
    const sixSegmentsIntersection = intersection(...sixSegments);
    const sixSegmentsMinusIntersection = uniq(
      flatten(
        sixSegments.map((segment) =>
          difference(segment, sixSegmentsIntersection)
        )
      )
    );
    const thirdSegment = intersection(sixSegmentsMinusIntersection, numbers[1]);
    const sixthSegment = difference(numbers[1], thirdSegment);
    const secondSegment = difference(
      intersection(numbers[4], sixSegmentsIntersection),
      numbers[1]
    );
    const fourthSegment = difference(
      numbers[4],
      secondSegment,
      thirdSegment,
      sixthSegment
    );
    const fifthSegment = difference(
      sixSegmentsMinusIntersection,
      thirdSegment,
      fourthSegment
    );
    const seventhSegment = difference(
      numbers[8],
      firstSegment,
      secondSegment,
      thirdSegment,
      fourthSegment,
      fifthSegment,
      sixthSegment
    );
    const segments = {
      1: firstSegment[0],
      2: secondSegment[0],
      3: thirdSegment[0],
      4: fourthSegment[0],
      5: fifthSegment[0],
      6: sixthSegment[0],
      7: seventhSegment[0],
    };
    const lettersToNumbers = {
      [segments[1] +
      segments[2] +
      segments[3] +
      segments[5] +
      segments[6] +
      segments[7]]: "0",
      [segments[3] + segments[6]]: "1",
      [segments[1] + segments[3] + segments[4] + segments[5] + segments[7]]:
        "2",
      [segments[1] + segments[3] + segments[4] + segments[6] + segments[7]]:
        "3",
      [segments[2] + segments[3] + segments[4] + segments[6]]: "4",
      [segments[1] + segments[2] + segments[4] + segments[6] + segments[7]]:
        "5",
      [segments[1] +
      segments[2] +
      segments[4] +
      segments[5] +
      segments[6] +
      segments[7]]: "6",
      [segments[1] + segments[3] + segments[6]]: "7",
      [segments[1] +
      segments[2] +
      segments[3] +
      segments[4] +
      segments[5] +
      segments[6] +
      segments[7]]: "8",
      [segments[1] +
      segments[2] +
      segments[3] +
      segments[4] +
      segments[6] +
      segments[7]]: "9",
    };
    const value = outputValues.reduce((acc, outputValue) => {
      const matchedNumber = find(
        lettersToNumbers,
        (_, key) =>
          difference(outputValue.split(""), key.split("")).length === 0 &&
          outputValue.length === key.length
      );

      return acc + (matchedNumber ?? "");
    }, "");

    answer += parseInt(value, 10);
  }

  console.log(answer);
}

main();

// 961734
