import { execSync } from "child_process";
import shelljs from "shelljs";

// usage:
// init the current day
// node scripts/init.js rust|ts

// init any day
// node scripts/init.js 2022 19 rust|ts

let [, , year, day, lang] = process.argv;

if (["rust", "ts"].includes(year)) {
  lang = year;
  const now = new Date();
  year = now.getFullYear();
  day = now.getDate().toString();
}

const dayPath = `${year}/src/day${day}`;

console.log(
  `Creating file structure for year ${year} day ${day} in ${lang}...`
);
shelljs.mkdir("-p", dayPath);
shelljs.cp("-r", `template/${lang}/*`, dayPath);

execSync(
  `aoc download --year ${year} --day ${day} --overwrite \
      --input-only --input-file=${dayPath}/input.txt`
);
