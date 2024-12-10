use aoc2024::fetch_or_load_input;
use std::collections::HashSet;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 10;
    let input = fetch_or_load_input(day)?;
    let grid = parse_input(&input);

    // Compute both types of scores
    let (score_part_one, score_part_two) = find_all_trails_score(&grid);

    println!("Score (part one): {}", score_part_one);
    println!("Score (part two): {}", score_part_two);

    Ok(())
}

/// Parse the input lines into a grid of digits.
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.trim().bytes().map(|b| b - b'0').collect())
        .collect()
}

#[derive(Clone)]
struct StateResult {
    total_paths: i64,
    endpoints: HashSet<(usize, usize)>,
}

impl StateResult {
    fn new() -> Self {
        Self {
            total_paths: 0,
            endpoints: HashSet::new(),
        }
    }

    fn single_endpoint(r: usize, c: usize) -> Self {
        let mut s = Self::new();
        s.total_paths = 1;
        s.endpoints.insert((r, c));
        s
    }

    fn merge(&mut self, other: &StateResult) {
        self.total_paths += other.total_paths;
        for &e in &other.endpoints {
            self.endpoints.insert(e);
        }
    }
}

/// Find all trails.
/// Returns a tuple: (score_part_one, score_part_two).
/// score_part_one = sum of endpoints count from each '0' cell
/// score_part_two = sum of total_paths from each '0' cell
fn find_all_trails_score(grid: &[Vec<u8>]) -> (usize, i64) {
    let rows = grid.len();
    let cols = if rows == 0 { 0 } else { grid[0].len() };

    // memo[row][col][digit]: Option<StateResult>
    let mut memo = vec![vec![vec![None; 10]; cols]; rows];

    let mut score_part_one = 0usize;
    let mut score_part_two = 0i64;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 {
                // Get StateResult
                let res = dfs(grid, r, c, 0, &mut memo);
                score_part_one += res.endpoints.len();
                score_part_two += res.total_paths;
            }
        }
    }

    (score_part_one, score_part_two)
}

fn dfs(
    grid: &[Vec<u8>],
    r: usize,
    c: usize,
    digit: u8,
    memo: &mut Vec<Vec<Vec<Option<StateResult>>>>,
) -> StateResult {
    if let Some(ref res) = memo[r][c][digit as usize] {
        return res.clone();
    }

    let mut res = StateResult::new();

    if grid[r][c] != digit {
        // mismatch: no paths
        memo[r][c][digit as usize] = Some(res.clone());
        return res;
    }

    if digit == 9 {
        // endpoint
        res = StateResult::single_endpoint(r, c);
        memo[r][c][9] = Some(res.clone());
        return res;
    }

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let rows = grid.len();
    let cols = grid[0].len();
    let next_digit = digit + 1;

    for &(dr, dc) in &directions {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] == next_digit {
                let subres = dfs(grid, nr, nc, next_digit, memo);
                res.merge(&subres);
            }
        }
    }

    memo[r][c][digit as usize] = Some(res.clone());
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_one_and_part_two() {
        let grid = parse_input(TEST_DATA);
        let (score_part_one, score_part_two) = find_all_trails_score(&grid);

        let expected_score_part_one = 36;
        assert_eq!(score_part_one, expected_score_part_one);

        let expected_score_part_two = 81;
        assert_eq!(score_part_two, expected_score_part_two);
    }
}
