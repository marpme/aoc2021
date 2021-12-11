use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_text(file_name: String) -> Vec<String> {
    let mut string_lines = vec![];

    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(ip) = line {
                string_lines.push(ip.to_string())
            }
        }
    }

    return string_lines
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
