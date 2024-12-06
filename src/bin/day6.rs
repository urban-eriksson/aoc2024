use aoc2024::fetch_or_load_input;
use std::collections::HashSet;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 6;
    let input = fetch_or_load_input(day)?;

    let (grid, guard_pos, guard_dir) = parse_input(&input)?;
    let (_final_grid, count, _has_loop) = simulate(grid.clone(), guard_pos, guard_dir);
    println!("Total count part one: {}", count);

    let loop_count = get_loop_count(&grid, guard_pos, guard_dir);
    println!("Total count of loops: {}", loop_count);

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn to_delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn parse_input(input: &str) -> Result<(Vec<Vec<char>>, (usize, usize), Direction), Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut guard_pos = None;
    for (r, line) in lines.iter().enumerate() {
        let row: Vec<char> = line.chars().collect();
        for (c, &ch) in row.iter().enumerate() {
            if ch == '^' {
                guard_pos = Some((r, c));
            }
        }
        grid.push(row);
    }

    let guard_pos = guard_pos.ok_or("No guard found in input")?;
    // According to the problem, '^' means the guard is facing upwards.
    let guard_dir = Direction::Up;

    Ok((grid, guard_pos, guard_dir))
}

fn simulate(
    mut grid: Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_dir: Direction,
) -> (Vec<Vec<char>>, usize, bool) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut direction = start_dir;
    let mut position = start_pos;

    // Keep track of visited positions in a set
    let mut visited = HashSet::new();
    visited.insert(position);

    // dir_grid: tracks the direction of the guard when it visited a cell last time
    // None means never visited, Some(Direction) means visited with that direction
    let mut dir_grid = vec![vec![None; cols]; rows];

    let mut has_loop = false;

    // Mark the starting position direction
    dir_grid[position.0][position.1] = Some(direction);

    // Function to check if a position is not an obstacle
    let is_free = |r: isize, c: isize| {
        let cell = grid[r as usize][c as usize];
        if cell == '#' {
            None // Obstacle
        } else {
            Some((r as usize, c as usize))
        }
    };

    loop {
        // Try to move forward in the current direction
        let (dr, dc) = direction.to_delta();
        let next_r = position.0 as isize + dr;
        let next_c = position.1 as isize + dc;

        if next_r < 0 || next_r >= rows as isize || next_c < 0 || next_c >= cols as isize {
            // Next step would leave the map, stop the simulation
            break;
        }

        match is_free(next_r, next_c) {
            Some(next_pos) => {
                // The next position is free; move there
                position = next_pos;

                // Check for loop:
                // If we've been here before, check the direction pattern
                let prev_dir = dir_grid[position.0][position.1];
                if let Some(prev_dir) = prev_dir {
                    // Tagging into a previous track
                    if direction == prev_dir {
                        has_loop = true;
                        break;
                    }
                }

                // Update direction grid for this cell
                dir_grid[position.0][position.1] = Some(direction);

                visited.insert(position);
            }
            None => {
                // Turn right
                direction = direction.turn_right();

                continue;
            }
        }
    }

    // Mark visited positions with 'X' on the grid
    for &(r, c) in &visited {
        grid[r][c] = 'X';
    }

    (grid, visited.len(), has_loop)
}

fn get_loop_count(grid: &Vec<Vec<char>>, guard_pos: (usize, usize), guard_dir: Direction) -> usize {
    let mut loop_count = 0;
    for r in 0..grid.len() {
        println!(
            "Processing row {} of {}, loop_count={}",
            r,
            grid.len(),
            loop_count
        );
        for c in 0..grid[0].len() {
            if !(r == guard_pos.0 && c == guard_pos.1) {
                let mut test_grid = grid.clone();
                test_grid[r][c] = '#';
                let (_final_grid, _count, has_loop) = simulate(test_grid, guard_pos, guard_dir);
                if has_loop {
                    loop_count += 1;
                }
            }
        }
    }
    loop_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        let (grid, guard_pos, guard_dir) = parse_input(TEST_DATA).unwrap();
        let (_final_grid, count, has_loop) = simulate(grid, guard_pos, guard_dir);

        let expected_count = 41;
        assert_eq!(count, expected_count);

        let expected_has_loop = false;
        assert_eq!(has_loop, expected_has_loop);
    }

    #[test]
    fn test_part_twp() {
        let (grid, guard_pos, guard_dir) = parse_input(TEST_DATA).unwrap();
        let loop_count = get_loop_count(&grid, guard_pos, guard_dir);

        let expected_loop_count = 6;
        assert_eq!(loop_count, expected_loop_count);
    }
}
