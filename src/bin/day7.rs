use aoc2024::fetch_or_load_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 7;
    let input = fetch_or_load_input(day)?;

    let use_concat = false;
    let sum = solve(&input, use_concat)?;
    println!("Total sum part one: {}", sum);

    let use_concat = true;
    let sum = solve(&input, use_concat)?;
    println!("Total sum part two: {}", sum);

    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Plus,
    Mult,
    Concat,
}

fn parse_line(line: &str) -> Result<(i64, Vec<i64>), Box<dyn Error>> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid line format".into());
    }

    let result_str = parts[0].trim();
    let result_val: i64 = result_str.parse()?;

    let nums_str = parts[1].trim();
    let nums: Result<Vec<i64>, _> = nums_str.split_whitespace().map(|s| s.parse()).collect();
    let nums = nums?;

    Ok((result_val, nums))
}

fn can_form_result(result: i64, nums: &[i64], use_concat: bool) -> bool {
    if nums.is_empty() {
        return false;
    }
    if nums.len() == 1 {
        return nums[0] == result;
    }

    fn backtrack(nums: &[i64], result: i64, idx: usize, val: i64, use_concat: bool) -> bool {
        if idx == nums.len() {
            // All numbers used, check result
            return val == result;
        }

        // Prune if already exceeded result
        if val > result {
            return false;
        }

        let next_num = nums[idx];

        // Try '+'
        {
            let new_val = apply_op(val, next_num, Op::Plus);
            if backtrack(nums, result, idx + 1, new_val, use_concat) {
                return true;
            }
        }

        // Try '*'
        {
            let new_val = apply_op(val, next_num, Op::Mult);
            if backtrack(nums, result, idx + 1, new_val, use_concat) {
                return true;
            }
        }

        // Try '|' (concatenation)
        if use_concat {
            let new_val = apply_op(val, next_num, Op::Concat);
            if backtrack(nums, result, idx + 1, new_val, use_concat) {
                return true;
            }
        }

        false
    }

    backtrack(nums, result, 1, nums[0], use_concat)
}

fn apply_op(val: i64, operand: i64, op: Op) -> i64 {
    match op {
        Op::Plus => val + operand,
        Op::Mult => val * operand,
        Op::Concat => {
            let digits = operand.checked_ilog10().unwrap_or(0) + 1;
            val * 10_i64.pow(digits) + operand
        }
    }
}

fn solve(input: &str, use_concat: bool) -> Result<i64, Box<dyn Error>> {
    let mut total_sum = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (result_val, nums) = parse_line(line)?;
        if can_form_result(result_val, &nums, use_concat) {
            total_sum += result_val;
        }
    }
    Ok(total_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_one() {
        let use_concat = false;
        let sum = solve(TEST_DATA, use_concat).unwrap();

        let expected_sum = 3749;
        assert_eq!(sum, expected_sum);
    }

    #[test]
    fn test_part_two() {
        let use_concat = true;
        let sum = solve(TEST_DATA, use_concat).unwrap();

        let expected_sum = 11387;
        assert_eq!(sum, expected_sum);
    }
}
