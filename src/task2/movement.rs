#[derive(Debug)]
pub struct Movement {
    pub way: String,
    pub steps: i32,
}

impl Movement {
    pub fn new(key: &String) -> Movement {
        let parts = (*key).split_whitespace().collect::<Vec<&str>>();
        let way = parts.get(0).unwrap();
        let steps = parts.get(1).unwrap().parse::<i32>().unwrap();

        return Movement {
            way: way.to_string(),
            steps,
        }
    }

    pub fn is_vertical(&self) -> bool {
        return self.way == "up" || self.way == "down"
    }

    pub fn get_steps(&self) -> i32 {
        if self.way == "down" {
            return self.steps * -1
        }

        return self.steps
    }
}