use aoc2024::fetch_or_load_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 4;
    let input = fetch_or_load_input(day)?;

    let grid = parse_grid(&input);

    let total_occurences = count_xmas_occurrences(&grid);
    println!("Total occurences part one: {}", total_occurences);

    let total_crossing_mas_occurences = count_crossing_mas_occurrences(&grid);
    println!("Total crossing mas occurrences part two: {}", total_crossing_mas_occurences);

    Ok(())
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty()) // Ignore empty lines
        .map(|line| line.chars().collect())
        .collect()
}

fn count_xmas_occurrences(grid: &[Vec<char>]) -> usize {
    let word = "XMAS";
    let word_len = word.len();
    let directions = [
        (-1, -1), // Up-Left
        (-1, 0),  // Up
        (-1, 1),  // Up-Right
        (0, -1),  // Left
        (0, 1),   // Right
        (1, -1),  // Down-Left
        (1, 0),   // Down
        (1, 1),   // Down-Right
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                let mut k = 0;
                let mut x = i as isize;
                let mut y = j as isize;

                while k < word_len {
                    if x < 0 || x >= rows as isize || y < 0 || y >= cols as isize {
                        break;
                    }
                    if grid[x as usize][y as usize] != word.chars().nth(k).unwrap() {
                        break;
                    }
                    x += dx;
                    y += dy;
                    k += 1;
                }

                if k == word_len {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_crossing_mas_occurrences(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    let mut count = 0;

    // Iterate over the grid, avoiding the borders
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if grid[i][j] == 'A' {
                if ((grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S')
                    || (grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M'))
                    && ((grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S')
                        || (grid[i - 1][j + 1] == 'S' && grid[i + 1][j - 1] == 'M'))
                {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_one() {
        let grid = parse_grid(TEST_DATA);
        let total_occurrences = count_xmas_occurrences(&grid);

        let expected_total = 18;
        assert_eq!(total_occurrences, expected_total);
    }

    #[test]
    fn test_part_two() {
        let grid = parse_grid(TEST_DATA);
        let total_occurrences = count_crossing_mas_occurrences(&grid);

        let expected_total = 9;
        assert_eq!(total_occurrences, expected_total);
    }
}
