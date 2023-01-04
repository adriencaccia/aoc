import { execSync } from "child_process";
import shelljs from "shelljs";

const [, , , year, , day] = process.argv;

const dayPath = `${year}/days/${day}`;

console.log(`Creating file structure for year ${year} day ${day}...`);
shelljs.mkdir("-p", dayPath);
shelljs.cp("-r", "template/*", dayPath);

execSync(
  `aoc download --year ${year} --day ${day} --overwrite \
      --input-file=${dayPath}/input.txt --puzzle-file=${dayPath}/example.txt -q`
);
