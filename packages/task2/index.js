import { assert } from "console";
import { readFileSync } from "fs";
import { Movement } from "./Movement.js";
import { Position } from "./Position.js";

const movements = readFileSync("./input2.txt")
  .toString()
  .split("\n")
  .filter(Boolean)
  .map((line) => {
    const [direction, steps] = line.split(" ");
    return new Movement(direction, parseInt(steps, 10));
  });

const position = new Position();

for (let movement of movements) {
  assert(!Number.isNaN(movement.steps), movements.indexOf(movement));
  position.move(movement);
}

console.log(position.position());
