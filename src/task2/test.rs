#[allow(unused_imports)]
use crate::task2::entry::{task1};

#[test]
fn basic() {
   assert_eq!(task1("./src/task2/input/basic_task2".to_string()), 150);
}

#[test]
fn challenge() {
   assert_eq!(task1("./src/task2/input/challenge_task2".to_string()), 2073315);
}
