use crate::common::parse_text::parse_text;
use crate::common::conversion::covert_vec;

pub fn task1(file_name: String) -> i32 {
    let text = parse_text(file_name.to_string());
    let numbers: Vec<i32> = covert_vec(text);

    let mut count = 0;
    for i in 1..numbers.len() {
        let prev = numbers.get(i - 1).unwrap();
        let curr = numbers.get(i).unwrap();
        count += if has_number_increased(prev, curr) { 1 } else { 0 };
    }

    println!("Numbers got increased {} times", count);
    return count;
}

pub fn task1_2(file_name: String) -> i32 {
    let text = parse_text(file_name.to_string());
    let numbers: Vec<i32> = covert_vec(text);

    let mut count = 0;
    for i in 4..numbers.len() + 1 {
        let prev = get_sliding_window(&numbers, i - 1);
        let curr = get_sliding_window(&numbers, i);
        count += if has_number_increased(&prev, &curr) { 1 } else { 0 };
    }

    println!("Numbers got increased {} times", count);
    return count;
}

fn get_sliding_window(numbers: &Vec<i32>, index: usize) -> i32 {
    let mut sum: i32 = 0;
    for i in (index - 3)..index {
        sum += (*numbers).get(i).unwrap();
    }
    return sum;
}

fn has_number_increased(number_prev: &i32, number_curr: &i32) -> bool {
    return (*number_curr) > (*number_prev)
}