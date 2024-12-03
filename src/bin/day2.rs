use aoc2024::fetch_or_load_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 2;
    let input = fetch_or_load_input(day)?;


    let number_of_safe_reports = calculate_safe_reports_part_one(&input)?;
    println!("Number of safe reports: {}", number_of_safe_reports);

    let number_of_safe_reports = calculate_safe_reports_part_two(&input)?;
    println!("Number of safe reports with up to one deletion: {}", number_of_safe_reports);

    Ok(())
}

fn calculate_safe_reports_part_one(input: &str) -> Result<usize, Box<dyn Error>> {
    let rows = parse_input(input)?;
    let valid_count = rows.iter()
        .map(|row| is_valid_sequence(row))
        .filter(|&is_valid| is_valid).count();

    Ok(valid_count)
}


fn calculate_safe_reports_part_two(input: &str) -> Result<usize, Box<dyn Error>> {
    let rows = parse_input(input)?;
    let valid_count = rows.iter()
        .map(|row| is_valid_or_can_be_made_valid(row))
        .filter(|&is_valid| is_valid).count();

    Ok(valid_count)
}


fn parse_input(input: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut rows = Vec::new();

    for (line_number, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue; // Skip empty lines
        }

        let numbers: Result<Vec<i32>, _> = line
            .split_whitespace()
            .map(str::parse)
            .collect();

        match numbers {
            Ok(nums) => rows.push(nums),
            Err(e) => {
                return Err(format!(
                    "Error parsing line {}: {}",
                    line_number + 1,
                    e
                )
                .into())
            }
        }
    }

    Ok(rows)
}

/// Determines if a sequence is valid or can be made valid by removing one element.
fn is_valid_or_can_be_made_valid(row: &[i32]) -> bool {
    // First, check if the original sequence is valid
    if is_valid_sequence(row) {
        return true;
    }

    // Try removing each element one at a time
    for i in 0..row.len() {
        let mut modified_row = row.to_vec();
        modified_row.remove(i);
        if is_valid_sequence(&modified_row) {
            return true;
        }
    }

    // Sequence cannot be made valid by removing one element
    false
}

/// Checks if a sequence is strictly increasing or decreasing with valid differences.
fn is_valid_sequence(row: &[i32]) -> bool {
    if row.len() < 2 {
        return true;
    }

    let mut trend = None; // None: trend not determined yet; Some(true): increasing; Some(false): decreasing

    for window in row.windows(2) {
        let diff = window[1] - window[0];

        if diff == 0 {
            return false; // Equal values are not allowed
        }

        let abs_diff = diff.abs();

        // Check if the absolute difference is between 1 and 3
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }

        let current_trend = diff > 0; // True for increasing, false for decreasing

        if let Some(t) = trend {
            if current_trend != t {
                return false; // Trend changed
            }
        } else {
            trend = Some(current_trend); // Set the initial trend
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_calculate_safe_reports() {
        let result = calculate_safe_reports_part_one(TEST_DATA).unwrap();
        assert_eq!(result, 2); 
    }

    #[test]
    fn test_calculate_safe_reports_with_up_to_one_deletion() {
        let result = calculate_safe_reports_part_two(TEST_DATA).unwrap();
        assert_eq!(result, 4); 
    }

}