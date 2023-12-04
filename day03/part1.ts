const input = Bun.file(import.meta.dir + "/input.txt");
const testInput = (await input.text()).replaceAll("\n", "");
const regex = /\d+/g;

let sum = 0;

let match;
while ((match = regex.exec(testInput)) !== null) {
  const matchIndexes = [];

  for (let i = match.index; i < testInput.length; i++) {
    if (!isNaN(parseInt(testInput[i]))) {
      matchIndexes.push(i);
    } else {
      break;
    }
  }

  const possibleCharacters = [];

  for (let i = 0; i < matchIndexes.length; i++) {
    if (matchIndexes[i] - 141 >= 0) {
      possibleCharacters.push(testInput[matchIndexes[i] - 141]);
    }

    if (matchIndexes[i] - 140 >= 0) {
      possibleCharacters.push(testInput[matchIndexes[i] - 140]);
    }

    if (matchIndexes[i] - 139 >= 0) {
      possibleCharacters.push(testInput[matchIndexes[i] - 139]);
    }

    if (matchIndexes[i] + 139 < testInput.length) {
      possibleCharacters.push(testInput[matchIndexes[i] + 139]);
    }

    if (matchIndexes[i] + 140 < testInput.length) {
      possibleCharacters.push(testInput[matchIndexes[i] + 140]);
    }

    if (matchIndexes[i] + 141 < testInput.length) {
      possibleCharacters.push(testInput[matchIndexes[i] + 141]);
    }
  }

  if (matchIndexes[0] - 1 >= 0) {
    possibleCharacters.push(testInput[matchIndexes[0] - 1]);
  }

  if (matchIndexes[matchIndexes.length - 1] + 1 < testInput.length) {
    possibleCharacters.push(testInput[matchIndexes[matchIndexes.length - 1] + 1]);
  }

  if (possibleCharacters.some((char) => char !== "." && isNaN(parseInt(char)))) {
    sum += parseInt(match);
  }
}

console.log(sum);
