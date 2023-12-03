import fs from "fs";
import path from "path";

const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf8");

const sum = input.split("\n").reduce((acc: number, line: string) => {
  const [, data] = line.split(":");

  const cubeMap = new Map([
    ["red", 0],
    ["green", 0],
    ["blue", 0],
  ]);

  const sets = data.split(";").map((set) => set.trim());

  sets.forEach((set) => {
    const cubes = set.split(",").map((cube) => cube.trim());

    cubes.forEach((cube) => {
      const [count, colour] = cube.split(" ");
      cubeMap.set(colour, Math.max(cubeMap.get(colour)!, parseInt(count)));
    });
  });

  return acc + (cubeMap.get("red")! * cubeMap.get("green")! * cubeMap.get("blue")!);
}, 0);

console.log(sum);

export {};
