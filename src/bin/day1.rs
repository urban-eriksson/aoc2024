use aoc2024::fetch_or_load_input;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 1;
    let input = fetch_or_load_input(day)?;

    let total_difference = calculate_total_difference(&input)?;
    println!("Total sum of absolute differences: {}", total_difference);

    let similarity_score = calculate_similarity_score(&input)?;
    println!("Similarity score: {}", similarity_score);

    Ok(())
}

fn calculate_total_difference(input: &str) -> Result<usize, Box<dyn Error>> {
    let (mut column1, mut column2) = parse_input(input)?;
    column1.sort();
    column2.sort();
    let total_difference = column1
        .iter()
        .zip(column2.iter())
        .map(|(&num1, &num2)| (num1 - num2).abs() as usize)
        .sum();
    Ok(total_difference)
}

fn parse_input(input: &str)  -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let nums: Vec<&str> = line.split_whitespace().collect();
        if nums.len() != 2 {
            return Err(format!("Invalid input line: '{}'", line).into());
        }

        let num1: i32 = nums[0].parse()?;
        let num2: i32 = nums[1].parse()?;

        column1.push(num1);
        column2.push(num2);
    }

    Ok((column1, column2))
}

fn calculate_similarity_score(input: &str) -> Result<usize, Box<dyn Error>> {
    let (column1, column2) = parse_input(input)?;

    // Build frequency map for the second column
    let mut freq_map = HashMap::new();
    for &num in &column2 {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    let similarity_score = column1
        .iter()
        .map(|&num| num as usize * freq_map.get(&num).cloned().unwrap_or(0))
        .sum();

    Ok(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_calculate_total_difference() {
        let result = calculate_total_difference(TEST_DATA).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_calculate_similarity_score() {
        let result = calculate_similarity_score(TEST_DATA).unwrap();
        assert_eq!(result, 31);
    }
}