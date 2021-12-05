pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new() -> Position {
        Position {
            x: 0,
            y: 0,
        }
    }

    pub fn move_horizontal(&mut self, steps: i32) {
        self.x += steps
    }

    pub fn move_vertical(&mut self, steps: i32) {
        self.y += steps
    }

    pub fn get_absolute_position(&self) -> i32 {
        return i32::abs(self.x) * i32::abs(self.y)
    }
}