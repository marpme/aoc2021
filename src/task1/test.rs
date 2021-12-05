#[allow(unused_imports)]
use crate::task1::entry::{task1, task1_2};

#[test]
fn basic() {
   assert_eq!(task1("./src/task1/input/basic_task1".to_string()), 7);
}

#[test]
fn challenge() {
   assert_eq!(task1("./src/task1/input/challenge_task1".to_string()), 1121);
}

#[test]
fn basic2() {
   assert_eq!(task1_2("./src/task1/input/basic_task1".to_string()), 5);
}

#[test]
fn challenge2() {
   assert_eq!(task1_2("./src/task1/input/challenge_task1_2".to_string()), 1065);
}