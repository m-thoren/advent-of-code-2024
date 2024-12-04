use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?; // Unwrap each line

        lines.push(line)
    }

    let part_one_sum = part_one(&lines);
    let part_one_sum = 0;

    println!("lines {}", lines);
    println!("The sum for part one is {}", part_one_sum);
    println!("The sum for part two is {}", part_two_sum);

    Ok(())
}

fn part_one(line: &[str]) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut sum = 0;

    sum
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let value = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert!(part_one(&value) == 18, "Example should be valid");
    }
}