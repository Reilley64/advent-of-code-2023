const fs = require("fs");
const path = require("path");

const calibrationDocument = fs.readFileSync(path.join(__dirname, "input.txt"), "utf8");

const numberMap = new Map([
  ["one", "1"],
  ["two", "2"],
  ["three", "3"],
  ["four", "4"],
  ["five", "5"],
  ["six", "6"],
  ["seven", "7"],
  ["eight", "8"],
  ["nine", "9"],
]);

const regex = /one|two|three|four|five|six|seven|eight|nine/g;

const sum = calibrationDocument.split("\n").reduce((acc: number, line: string) => {
  function findNumber(str: string, reverse: boolean) {
    let result = null;
    let word = "";

    for (let i = reverse ? str.length - 1 : 0; reverse ? i >= 0 : i < str.length; reverse ? i-- : i++) {
      if (!isNaN(parseInt(str[i]))) {
        result = str[i];
      } else {
        word = reverse ? `${str[i]}${word}` : `${word}${str[i]}`;
        const match = word.match(regex);
        if (match) {
          result = numberMap.get(match[0]);
        }
      }

      if (result != null) {
        break;
      }
    }

    return result;
  }

  const first = findNumber(line, false);
  const last = findNumber(line, true);

  return acc + (first ? parseInt(`${first}${last}`) : 0);
}, 0);

console.log("sum:", sum);
