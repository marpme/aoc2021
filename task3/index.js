import { readFileSync } from "fs";

const binLines = readFileSync("./input2.txt").toString().split("\n");

const setLength = binLines.length;

const calculateRate = (binaries, rater) => {
  const binaryLength = binaries[0].length;
  let rate = new Array(binaryLength).fill(0);

  for (let column = 0; column < binaryLength; column++) {
    for (let bin of binaries) {
      const number = parseInt(bin, 2);
      rate[column] += Boolean(number & (1 << column)) ? 0 : 1;
    }
  }

  return parseInt(rate.reverse().map(rater).join(""), 2);
};

const gammaRate = calculateRate(binLines, (count) =>
  count / setLength > 0.5 ? 1 : 0
);
const epsilonRate = calculateRate(binLines, (count) =>
  count / setLength < 0.5 ? 1 : 0
);

console.log(gammaRate, epsilonRate, gammaRate * epsilonRate);
