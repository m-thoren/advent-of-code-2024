use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Open the file.
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Create vectors to hold the left and right columns.
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();

    // Read the file line by line.
    for line in reader.lines() {
        let line = line?; // Unwrap each line

        // Split the line by whitespace.
        let mut split = line.split_whitespace();

        // Parse the left and right numbers.
        if let (Some(left), Some(right)) = (split.next(), split.next()) {
            let left_num: i32 = left.parse().unwrap();
            let right_num: i32 = right.parse().unwrap();

            // Add the numbers to the respective vectors.
            left_column.push(left_num);
            right_column.push(right_num);
        }
    }

    absolute_differences(&left_column, &right_column);
    occurrences(&left_column, &right_column);

    Ok(())
}

fn absolute_differences(left_column_input: &[i32], right_column_input: &[i32]) -> () {
    let mut left_column: Vec<i32> = left_column_input.to_vec();
    let mut right_column: Vec<i32> = right_column_input.to_vec();

    left_column.sort();
    right_column.sort();

    // Initialize a variable to sum the differences
    let mut sum_of_differences = 0;

    // Loop through the vectors and sum the differences
    for i in 0..left_column.len() {
        let difference: i32 = (left_column[i] - right_column[i]).abs();
        sum_of_differences += difference;
    }

    println!("Difference: {:?}", sum_of_differences);
}

fn occurrences(left_column_input: &[i32], right_column_input: &[i32]) -> () {
    let mut right_column: Vec<i32> = right_column_input.to_vec();
    let mut similarity_score = 0;

    for left_value in left_column_input {
        // Count occurrences of `left_value` in the right column
        let count: i32 = right_column.iter().filter(|&&x| x == *left_value).count().try_into().unwrap();

        similarity_score += left_value * count;

        // Remove the occurrences from the right column
        right_column.retain(|&x| x != *left_value);
    }

    println!("Similarity score: {:?}", similarity_score);
}