use aoc2024::fetch_or_load_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 5;
    let input = fetch_or_load_input(day)?;

    let (rules, print_jobs) = parse_input(&input)?;
    let validity = validate_print_jobs(&rules, &print_jobs);

    let valid_print_jobs: Vec<_> = print_jobs
        .clone()
        .into_iter()
        .zip(validity.iter())
        .filter(|(_, &is_valid)| is_valid)
        .map(|(job, _)| job)
        .collect();

    let total_sum = sum_middle_pages(&valid_print_jobs);
    println!("Total sum part one: {}", total_sum);

    let invalid_print_jobs: Vec<_> = print_jobs
        .into_iter()
        .zip(validity.iter())
        .filter(|(_, &is_valid)| !is_valid)
        .map(|(job, _)| job)
        .collect();

    let corrected_jobs = correct_invalid_print_jobs(&invalid_print_jobs, &rules);
    let total_sum_corrected = sum_middle_pages(&corrected_jobs);
    println!("Total sum part two: {}", total_sum_corrected);

    Ok(())
}

fn parse_input(input: &str) -> Result<(Vec<(u32, u32)>, Vec<Vec<u32>>), Box<dyn Error>> {
    let mut rules = Vec::new();
    let mut print_jobs = Vec::new();
    let mut is_rules_section = true;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            is_rules_section = false;
            continue;
        }

        if is_rules_section {
            // Parse rules
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid rule format: '{}'", line).into());
            }
            let left: u32 = parts[0].trim().parse()?;
            let right: u32 = parts[1].trim().parse()?;
            rules.push((left, right));
        } else {
            // Parse print jobs
            let pages: Result<Vec<u32>, _> = line.split(',').map(|s| s.trim().parse()).collect();
            match pages {
                Ok(pages) => print_jobs.push(pages),
                Err(e) => return Err(format!("Invalid page number: '{}'", e).into()),
            }
        }
    }

    Ok((rules, print_jobs))
}

/// Validates the print jobs based on the rules.
fn validate_print_jobs(rules: &[(u32, u32)], print_jobs: &[Vec<u32>]) -> Vec<bool> {
    print_jobs
        .iter()
        .map(|job| {
            let mut page_positions = std::collections::HashMap::new();
            for (pos, &page) in job.iter().enumerate() {
                page_positions.insert(page, pos);
            }

            for &(x, y) in rules {
                if let (Some(&pos_x), Some(&pos_y)) =
                    (page_positions.get(&x), page_positions.get(&y))
                {
                    if pos_y < pos_x {
                        // Rule violated: y appears before x
                        return false;
                    }
                }
            }
            true
        })
        .collect()
}

/// Calculates the total sum of middle page numbers from valid print jobs.
fn sum_middle_pages(print_jobs: &[Vec<u32>]) -> u32 {
    print_jobs
        .iter()
        .map(|job| {
            let middle_index = job.len() / 2;
            job[middle_index]
        })
        .sum()
}

fn correct_print_job(mut job: Vec<u32>, rules: &[(u32, u32)]) -> Vec<u32> {
    let mut changed = true;

    while changed {
        changed = false;

        let mut page_positions = std::collections::HashMap::new();
        for (pos, &page) in job.iter().enumerate() {
            page_positions.insert(page, pos);
        }

        for &(x, y) in rules {
            if let (Some(&pos_x), Some(&pos_y)) = (page_positions.get(&x), page_positions.get(&y)) {
                if pos_y < pos_x {
                    // Move x to be right before y
                    let x_page = job.remove(pos_x);
                    let pos_y = page_positions[&y]; // Update pos_y after removing x
                    job.insert(pos_y, x_page);
                    changed = true;
                    break; // Start over as positions have changed
                }
            }
        }
    }

    job
}

fn correct_invalid_print_jobs(print_jobs: &[Vec<u32>], rules: &Vec<(u32, u32)>) -> Vec<Vec<u32>> {
    print_jobs
        .iter()
        .map(|job| correct_print_job(job.clone(), rules))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_one() {
        let (rules, print_jobs) = parse_input(TEST_DATA).unwrap();
        let validity = validate_print_jobs(&rules, &print_jobs);

        let valid_print_jobs: Vec<_> = print_jobs
            .into_iter()
            .zip(validity.iter())
            .filter(|(_, &is_valid)| is_valid)
            .map(|(job, _)| job)
            .collect();

        let total_sum = sum_middle_pages(&valid_print_jobs);

        let expected_total = 143;
        assert_eq!(total_sum, expected_total);
    }

    #[test]
    fn test_part_two() {
        let (rules, print_jobs) = parse_input(TEST_DATA).unwrap();
        let validity = validate_print_jobs(&rules, &print_jobs);

        let invalid_print_jobs: Vec<_> = print_jobs
            .into_iter()
            .zip(validity.iter())
            .filter(|(_, &is_valid)| !is_valid)
            .map(|(job, _)| job)
            .collect();

        let corrected_jobs = correct_invalid_print_jobs(&invalid_print_jobs, &rules);
        let total_sum = sum_middle_pages(&corrected_jobs);

        let expected_total = 123;
        assert_eq!(total_sum, expected_total);
    }
}
