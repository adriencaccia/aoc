/**
 * Pretty print a 2D string matrix.
 *
 * Taken from https://gist.github.com/lbn/3d6963731261f76330af
 * @param mat `string[][]`
 */
function logMatrix(mat: string[][]) {
  let shape = [mat.length, mat[0].length];
  function col(mat: string[][], i: number) {
    return mat.map((row) => row[i]);
  }
  let colMaxes: number[] = [];
  for (let i = 0; i < shape[1]; i++) {
    colMaxes.push(
      Math.max.apply(
        null,
        col(mat, i).map((n) => n.toString().length)
      )
    );
  }

  mat.forEach((row) => {
    console.log.apply(
      null,
      row.map((val, j) => {
        return (
          new Array(colMaxes[j] - val.toString().length + 1).join(" ") +
          val.toString() +
          "  "
        );
      })
    );
  });
}

export default logMatrix;
