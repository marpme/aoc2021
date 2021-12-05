mod task1;
mod common;
mod task2;

fn main() {
    task1::entry::task1("./src/task1/input/challenge_task1".to_string());
    task1::entry::task1_2("./src/task1/input/challenge_task1_2".to_string());
    task2::entry::task1("./src/task2/input/basic_task2".to_string());
}
