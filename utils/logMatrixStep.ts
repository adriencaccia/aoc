import logUpdate from "log-update";

async function logMatrixStep<Item>(mat: Item[][], timeout = 100) {
  logUpdate(mat.map((row) => row.join("")).join("\n"));
  await new Promise((resolve) => setTimeout(resolve, timeout));
}

export default logMatrixStep;
