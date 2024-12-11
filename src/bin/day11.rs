use aoc2024::fetch_or_load_input;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 11;
    let input = fetch_or_load_input(day)?;
    let map = parse_input(&input)?;

    let count_part_one = evolve_sequence(&map, 25);
    println!("Total count part one: {}", count_part_one);

    let count_part_two = evolve_sequence(&map, 75);
    println!("Total count part two: {}", count_part_two);

    Ok(())
}

fn parse_input(line: &str) -> Result<HashMap<usize, usize>, Box<dyn Error>> {
    let mut map = HashMap::new();
    let numbers: Result<Vec<usize>, _> = line.split_whitespace().map(|s| s.parse()).collect();
    let numbers = numbers?;
    for num in numbers {
        *map.entry(num).or_insert(0) += 1;
    }
    Ok(map)
}

/// Evolve the sequence represented by a map of numbers and their counts
/// for `n` iterations.
fn evolve_sequence(initial: &HashMap<usize, usize>, n: usize) -> usize {
    let mut current = initial.clone();
    for _ in 0..n {
        let mut next = HashMap::new();
        for (num, count) in current {
            apply_rules_to_map(num, count, &mut next);
        }
        current = next;
    }
    let total: usize = current.values().sum();

    total
}

/// Apply the rules to a single number and add results to `out_map`.
/// `count` is how many times `num` appears.
fn apply_rules_to_map(num: usize, count: usize, out_map: &mut HashMap<usize, usize>) {
    if num == 0 {
        // Zero -> one
        *out_map.entry(1).or_insert(0) += count;
    } else {
        let digits = num.checked_ilog10().unwrap_or(0) + 1;
        if digits % 2 == 0 {
            // Even number of digits -> split
            let denominator = 10_u64.pow(digits / 2) as usize;
            let (quotient, remainder) = (num / denominator, num % denominator);
            *out_map.entry(quotient).or_insert(0) += count;
            *out_map.entry(remainder).or_insert(0) += count;
        } else {
            // Odd digits and not zero -> multiply by 2024
            let product = num * 2024;
            *out_map.entry(product).or_insert(0) += count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "125 17";

    #[test]
    fn test_part_one() {
        let initial_map = parse_input(TEST_DATA).unwrap();
        let count = evolve_sequence(&initial_map, 25);

        let expected_count = 55312;
        assert_eq!(count, expected_count);
    }
}
