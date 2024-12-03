use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MIN_DIFFERENCE: i32 = 1;
const MAX_DIFFERENCE: i32 = 3;
const ALLOWED_ERROR_COUNT: i32 = 1;

fn main() -> io::Result<()> {
    // Open the file.
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut amount_of_ok_levels = 0;
    let mut amount_of_not_ok_levels = 0;

    for line in reader.lines() {
        let line = line?; // Unwrap each line

        // Split the line by whitespace into an iterator of values
        let values: Result<Vec<i32>, _> = line.split_whitespace().map(str::parse).collect();

        // Skip lines with parsing errors
        let values = match values {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Error parsing line: {}", line);
                continue;
            }
        };

        // Validate level
        if validate_level(&values) {
            amount_of_ok_levels += 1;
        } else {
            amount_of_not_ok_levels += 1;
        }
    }

    println!("Amount of ok levels: {}", amount_of_ok_levels);
    println!("Amount of not ok levels: {}", amount_of_not_ok_levels);

    Ok(())
}

fn validate_level(values: &[i32]) -> bool {
    if values.len() < 2 {
        return false; // Single value or empty levels are not considered OK
    }

    let mut is_increasing: Option<bool> = None;
    let mut error_count = 0;

    for pair in values.windows(2) {
        let (first, second) = (pair[0], pair[1]);
        let mut pair_had_error = false;

        if first == second {
            pair_had_error = true; // Consecutive values cannot be equal
        }

        let diff = (first - second).abs();

        if diff < MIN_DIFFERENCE || diff > MAX_DIFFERENCE {
            pair_had_error = true; // Difference out of bounds
        }

        match is_increasing {
            Some(true) if second < first => {
                pair_had_error = true; // Decreasing in an increasing sequence
            }
            Some(false) if second > first => {
                pair_had_error = true; // Increasing in a decreasing sequence
            }
            None => {
                if first != second {
                    is_increasing = Some(second > first)
                }
            } // Initialize direction
            _ => {} // No change needed if direction matches
        }

        if pair_had_error {
            error_count += 1;
        }

        if error_count > ALLOWED_ERROR_COUNT {
            return false; // More than one error is not allowed
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_values() {
        let values: Vec<i32> = vec![];
        assert!(!validate_level(&values), "Empty values should not be valid");
    }

    #[test]
    fn test_single_value() {
        let values = vec![42];
        assert!(!validate_level(&values), "Single value should not be valid");
    }

    #[test]
    fn test_all_ok_values_increasing() {
        let values = vec![1, 2, 3];
        assert!(
            validate_level(&values),
            "Valid increasing sequence should be valid"
        );
    }

    #[test]
    fn test_all_ok_values_decreasing() {
        let values = vec![3, 2, 1];
        assert!(
            validate_level(&values),
            "Valid decreasing sequence should be valid"
        );
    }

    #[test]
    fn test_mixed_sequence_valid() {
        let values = vec![1, 3, 2];
        assert!(
            validate_level(&values),
            "Mixed increasing and decreasing sequence with one error should be valid"
        );
    }

    #[test]
    fn test_mixed_sequence_invalid() {
        let values = vec![1, 4, 3, 2];
        assert!(
            !validate_level(&values),
            "Mixed increasing and decreasing sequence with more than one error should not be valid"
        );
    }

    #[test]
    fn test_equal_consecutive_values() {
        let values = vec![1, 1, 2];
        assert!(
            validate_level(&values),
            "Sequence with consecutive equal values should not be valid"
        );
    }

    #[test]
    fn test_equal_consecutive_values_invalid() {
        let values = vec![1, 1, 1, 2];
        assert!(
            !validate_level(&values),
            "Sequence with consecutive equal values should not be valid"
        );
    }

    #[test]
    fn test_difference_too_small() {
        let values = vec![1, 1 + MIN_DIFFERENCE - 1];
        assert!(
            validate_level(&values),
            "Difference below MIN_DIFFERENCE should be valid"
        );
    }

    #[test]
    fn test_difference_too_small_invalid() {
        let values = vec![1, 1 + MIN_DIFFERENCE - 1, 1 + MIN_DIFFERENCE - 1];
        assert!(
            !validate_level(&values),
            "Difference below MIN_DIFFERENCE twice should not be valid"
        );
    }

    #[test]
    fn test_difference_too_large() {
        let values = vec![1, 1 + MAX_DIFFERENCE + 1];
        assert!(
            validate_level(&values),
            "Difference above MAX_DIFFERENCE should be valid"
        );
    }

    #[test]
    fn test_difference_too_large_invalid() {
        let values = vec![1, 1 + MAX_DIFFERENCE + 1, 1 + MAX_DIFFERENCE + 1];
        assert!(
            !validate_level(&values),
            "Difference above MAX_DIFFERENCE twice should not be valid"
        );
    }

    #[test]
    fn test_valid_min_difference() {
        let values = vec![1, 1 + MIN_DIFFERENCE];
        assert!(
            validate_level(&values),
            "Sequence with exact MIN_DIFFERENCE should be valid"
        );
    }

    #[test]
    fn test_valid_max_difference() {
        let values = vec![1, 1 + MAX_DIFFERENCE];
        assert!(
            validate_level(&values),
            "Sequence with exact MAX_DIFFERENCE should be valid"
        );
    }

    #[test]
    fn test_part_2_a() {
        let values = vec![7, 6, 4, 2, 1];
        assert!(validate_level(&values), "7,6,4,2,1 should be valid");
    }

    #[test]
    fn test_part_2_b() {
        let values = vec![1, 2, 7, 8, 9];
        assert!(validate_level(&values), "1,2,7,8,9 should not be valid");
    }

    #[test]
    fn test_part_2_c() {
        let values = vec![9, 7, 6, 2, 1];
        assert!(validate_level(&values), "9,7,6,2,1 should not be valid");
    }

    #[test]
    fn test_part_2_d() {
        let values = vec![1, 3, 2, 4, 5];
        assert!(validate_level(&values), "1,3,2,4,5 should be valid");
    }

    #[test]
    fn test_part_2_e() {
        let values = vec![8, 6, 4, 4, 1];
        assert!(validate_level(&values), "8,6,4,4,1 should be valid");
    }

    #[test]
    fn test_part_2_f() {
        let values = vec![1, 3, 6, 7, 9];
        assert!(validate_level(&values), "1,3,6,7,9 should be valid");
    }

    #[test]
    fn test_starting_numbers_equal() {
        let values = vec![1, 1, 3, 6, 7, 9];
        assert!(validate_level(&values), "1,1,3,6,7,9 should be valid");
    }
}
