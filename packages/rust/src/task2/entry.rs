use crate::common::parse_text::parse_text;
use crate::task2::position::{Position};
use crate::task2::movement::Movement;

pub fn task1(file_name: String) -> i32 {
    let text = parse_text(file_name.to_string());
    let movements: Vec<Movement> = text.iter().map(|x| Movement::new(x)).collect();

    let mut position: Position = Position::new();
    for movement in movements {
        if movement.is_vertical() {
            position.move_vertical(movement.get_steps())
        } else {
            position.move_horizontal(movement.get_steps())
        }
    }

    println!("Position after movement: {}", position.get_absolute_position());
    return position.get_absolute_position();
}