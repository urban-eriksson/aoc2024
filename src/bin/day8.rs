use aoc2024::fetch_or_load_input;
use std::collections::{HashMap, HashSet};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 8;
    let input = fetch_or_load_input(day)?;

    let grid = parse_input(&input);

    let count = solve(&grid, false);
    println!("Total count part one: {}", count);

    let count_part_two = solve(&grid, true);
    println!("Total count part two: {}", count_part_two);

    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve(grid: &[Vec<char>], part_two: bool) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    let (min_idx, max_idx) = if part_two {
        (0, rows.max(cols))
    } else {
        (1, 2)
    };

    // Map each character to its list of positions
    let mut positions_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for x in 0..rows {
        for y in 0..cols {
            let ch = grid[x][y];
            if ch != '.' {
                positions_map.entry(ch).or_default().push((x, y));
            }
        }
    }

    let mut antinodes = HashSet::new();

    // For each character, consider all pairs of distinct positions
    for (_, positions) in &positions_map {
        let len = positions.len();
        // Generate all pairs (p1,p2), p1 < p2 to avoid duplicates
        for i in 0..len {
            for j in i + 1..len {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                for n in min_idx..max_idx {
                    let ax = x2 as isize + n as isize * dx;
                    let ay = y2 as isize + n as isize * dy;
                    if ax >= 0 && ax < rows as isize && ay >= 0 && ay < cols as isize {
                        antinodes.insert((ax as usize, ay as usize));
                    } else {
                        break;
                    }
                }

                for n in min_idx..max_idx {
                    let ax = x1 as isize - n as isize * dx;
                    let ay = y1 as isize - n as isize * dy;
                    if ax >= 0 && ax < rows as isize && ay >= 0 && ay < cols as isize {
                        antinodes.insert((ax as usize, ay as usize));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_one() {
        let grid = parse_input(TEST_DATA);
        let count = solve(&grid, false);

        let expected_count = 14;
        assert_eq!(count, expected_count);
    }

    #[test]
    fn test_part_two() {
        let grid = parse_input(TEST_DATA);
        let count = solve(&grid, true);

        let expected_count = 34;
        assert_eq!(count, expected_count);
    }
}
