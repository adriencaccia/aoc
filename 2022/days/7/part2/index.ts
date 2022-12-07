import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

interface Dir {
  name: string;
  children: Record<string, TreeItem>;
}
interface File {
  name: string;
  size: number;
}

type TreeItem = File | Dir;

interface Directory {
  name: string;
  size: number;
  children: Record<string, Directory>;
  files: File[];
}

function calculateDirectoriesSizes(directory: Directory) {
  for (const childDirectory of Object.values(directory.children)) {
    directory.size += calculateDirectoriesSizes(childDirectory);
  }
  return directory.size;
}

function getNestedDirectoryByPath(directory: Directory, path: string[]) {
  if (path.length === 0) {
    return directory;
  }
  return path.reduce(
    (currentDirectory, currentPath) => currentDirectory.children[currentPath],
    directory
  );
}

function extractDirectoriesSize(directory: Directory): number[] {
  return [directory.size].concat(
    ...Object.values(directory.children).map(extractDirectoriesSize)
  );
}

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const lines = input.split("\n");
  lines.pop();

  let rootDirectory: Directory = {
    name: "/",
    children: {},
    files: [],
    size: 0,
  };

  const currentDirectoryPath = [];

  // stop at each ls and add it the directories list
  let i = 1;
  while (i < lines.length) {
    if (lines[i] === "$ cd ..") {
      currentDirectoryPath.pop();
      i++;
      continue;
    }
    if (lines[i].startsWith("$ cd")) {
      const directory = getNestedDirectoryByPath(
        rootDirectory,
        currentDirectoryPath
      );

      const nestedDirectoryName = lines[i].slice(5);
      directory.children[nestedDirectoryName] = {
        name: nestedDirectoryName,
        size: 0,
        children: {},
        files: [],
      };
      currentDirectoryPath.push(nestedDirectoryName);
      i++;
      continue;
    }

    // line is "$ ls"
    i++;
    const currentDirectoryName = currentDirectoryPath.at(-1) ?? "";

    const directory =
      currentDirectoryName === "/"
        ? rootDirectory
        : getNestedDirectoryByPath(rootDirectory, currentDirectoryPath);

    while (i < lines.length && !lines[i].startsWith("$")) {
      const line = lines[i];
      if (line.startsWith("dir")) {
        i++;
        continue;
      }
      const [sizeString, fileName] = line.split(" ");
      const file: File = { name: fileName, size: Number(sizeString) };
      directory.files.push(file);
      directory.size += file.size;
      i++;
    }
  }

  // recursively calculate the size of the
  calculateDirectoriesSizes(rootDirectory);

  const sizes = extractDirectoriesSize(rootDirectory).sort((a, b) => a - b);

  const spaceToDeleteMin = 30000000 - (70000000 - (sizes.at(-1) ?? 0));

  const answer = sizes.find((size) => size >= spaceToDeleteMin);

  console.log(answer);
}

main();

// 404395
