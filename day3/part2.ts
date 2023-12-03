const input = Bun.file(import.meta.dir + "/input.txt");
const testInput = (await input.text()).replaceAll("\n", "");
const starRegex = /\*/g;
const numberRegex = /\d+/g;

let sum = 0;

let starMatch;
while ((starMatch = starRegex.exec(testInput)) !== null) {
  const gearNumbers = [];

  if (!isNaN(parseInt(testInput[starMatch.index - 1]))) {
    const numberMatches = [...testInput.slice(starMatch.index - 3, starMatch.index).matchAll(numberRegex)];
    gearNumbers.push(parseInt(numberMatches[numberMatches.length - 1][0]));
  }

  if (!isNaN(parseInt(testInput[starMatch.index + 1]))) {
    const numberMatches = [...testInput.slice(starMatch.index + 1, starMatch.index + 4).matchAll(numberRegex)];
    gearNumbers.push(parseInt(numberMatches[0][0]));
  }

  let above = testInput.slice(starMatch.index - 143, starMatch.index - 136);
  above.split("").forEach((char, index) => {
    if (isNaN(parseInt(above[index]))) {
      if (index < 3) {
        Array.from({ length: index + 1 }).forEach((value, charIndex) => {
          above = above.substring(0, charIndex) +  "." + above.substring(charIndex + 1);
        });
      } else if (index > 3) {
        Array.from({ length: above.length - index }).forEach((value, charIndex) => {
          above = above.substring(0, (above.length - 1) - charIndex) +  "." + above.substring((above.length - 1) - charIndex + 1);
        });
      }
    }
  });
  const aboveMatches = [...above.matchAll(numberRegex)];
  aboveMatches.forEach((match) => gearNumbers.push(parseInt(match[0])));

  let below = testInput.slice(starMatch.index + 137, starMatch.index + 144);
  below.split("").forEach((char, index) => {
    if (isNaN(parseInt(below[index]))) {
      if (index < 3) {
        Array.from({ length: index + 1 }).forEach((value, charIndex) => {
          below = below.substring(0, charIndex) +  "." + below.substring(charIndex + 1);
        });
      } else if (index > 3) {
        Array.from({ length: below.length - index }).forEach((value, charIndex) => {
          below = below.substring(0, (below.length - 1) - charIndex) +  "." + below.substring((below.length - 1) - charIndex + 1);
        });
      }
    }
  });
  const belowMatches = [...below.matchAll(numberRegex)];
  belowMatches.forEach((match) => gearNumbers.push(parseInt(match[0])));

  if (gearNumbers.length >= 2) {
    sum +=  gearNumbers.reduce((acc, curr) => acc * curr, 1);
  }
}

console.log(sum);
