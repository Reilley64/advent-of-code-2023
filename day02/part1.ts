const input = Bun.file(import.meta.dir + "/input.txt.txt");

const cubeMap = new Map([
  ["red", 12],
  ["green", 13],
  ["blue", 14],
]);

const sum = (await input.text()).split("\n").reduce((acc: number, line: string) => {
  const [identifier, data] = line.split(":");

  const id = parseInt(identifier.slice(5, identifier.length));

  const sets = data.split(";").map((set) => set.trim());

  const invalid = sets.some((set) => {
    const cubes = set.split(",").map((cube) => cube.trim());
    return cubes.some((cube) => {
      const [count, colour] = cube.split(" ");
      return parseInt(count) > cubeMap.get(colour)!;
    });
  });

  if (invalid) {
    return acc;
  }

  return acc + id;
}, 0);

console.log(sum);
