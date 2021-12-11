export class Position {
  constructor() {
    this.horizontal = 0;
    this.depth = 0;
    this.aim = 0;
  }

  moveAim(movement) {
    if (movement.direction === "down") {
      this.aim += movement.steps;
    } else {
      this.aim -= movement.steps;
    }
  }

  move(movement) {
    console.log(this, movement.steps);
    if (["up", "down"].includes(movement.direction)) {
      this.moveAim(movement);
    } else {
      this.horizontal += movement.steps;
      this.depth += movement.steps * this.aim;
    }
  }

  position() {
    return Math.abs(this.depth) * Math.abs(this.horizontal);
  }
}
