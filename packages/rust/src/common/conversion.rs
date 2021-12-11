pub fn covert_vec(vector: Vec<String>) -> Vec<i32> {
    return vector.iter().map(str_to_i32).collect()
}

fn str_to_i32(x: &String) -> i32 {
    return (*x).parse::<i32>().ok().unwrap();
}