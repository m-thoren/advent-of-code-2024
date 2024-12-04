use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut part_one_sum: i32 = 0;
    let mut part_two_sum: i32 = 0;

    for line in reader.lines() {
        let line = line?; // Unwrap each line

        part_one_sum += part_one(&line);
        part_two_sum += part_two(&line);
    }

    println!("The sum for part one is {}", part_one_sum);
    println!("The sum for part two is {}", part_two_sum);

    Ok(())
}

fn part_one(line: &str) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut sum = 0;

    for caps in re.find_iter(&line) {
        let re2 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        if let Some(caps) = re2.captures(caps.into()) {
            // Extract the two captured groups and parse them as integers
            let a: i32 = caps[1].parse().unwrap();
            let b: i32 = caps[2].parse().unwrap();

            sum += a * b;
        }
    }

    sum
}

fn part_two(line: &str) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    // Remove parts of the string between every `don't()` and `do()`
    let cleaned_input = remove_between_dont_and_do(&line);
    println!("cleaned_input {}", &cleaned_input);

    let mut sum = 0;

    for mat in re.find_iter(&cleaned_input) {
        let matched = mat.as_str();
        let re2 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        if let Some(matched) = re2.captures(matched.into()) {
            // Extract the two captured groups and parse them as integers
            let a: i32 = matched[1].parse().unwrap();
            let b: i32 = matched[2].parse().unwrap();
            sum += a * b;
        }
    }
    println!("Summing {}", sum);

    sum
}

fn remove_between_dont_and_do(input: &str) -> String {
    let mut output = String::new();
    let mut start = 0;
    let mut search_start = 0;
    let mut unmatched_dont = false;

    while let Some(dont_start) = input[search_start..].find("don't()").map(|idx| search_start + idx) {
        // Add everything before this `don't()`
        output.push_str(&input[start..dont_start]);

        if let Some(do_start) = input[dont_start..].find("do()").map(|idx| dont_start + idx) {
            // Skip over the part between `don't()` and `do()`
            start = do_start + "do()".len();
            search_start = start; // Continue searching from after `do()`
            unmatched_dont = false; // Reset unmatched status
        } else {
            // No `do()` found, mark as unmatched and break
            unmatched_dont = true;
            break;
        }
    }

    // If there is an unmatched `don't()`, truncate the rest of the string
    if unmatched_dont {
        return output;
    }

    // Add the remainder of the string
    output.push_str(&input[start..]);

    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let value = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert!(part_one(&value) == 161, "Example should be valid");
    }

    #[test]
    fn test_part_two_example() {
        let value = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert!(part_two(&value) == 48, "Example should be valid");
    }

    #[test]
    fn test_part_two_multiple_donts() {
        let value = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert!(part_two(&value) == 96, "Example twice should be valid");
    }

    #[test]
    fn test_part_two_multiple_donts_no_end_do() {
        let value = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)un?mul(8,5))";
        assert!(part_two(&value) == 56, "Example twice should be valid");
    }
}
