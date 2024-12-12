use aoc2024::fetch_or_load_input;
use std::collections::VecDeque;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 12;
    let input = fetch_or_load_input(day)?;

    let grid = parse_input(&input)?;

    let (original_score, sides_score) = compute_scores(&grid);

    println!("Total score part one: {}", original_score);
    println!("Total score part two: {}", sides_score);

    Ok(())
}

/// Parse input into a 2D grid of chars.
/// Each line must have the same length.
fn parse_input(input: &str) -> Result<Vec<Vec<char>>, &'static str> {
    let lines: Vec<&str> = input.lines().map(|l| l.trim()).collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut grid = Vec::with_capacity(rows);

    for &line in &lines {
        if line.len() != cols {
            return Err("All lines must have equal length");
        }
        grid.push(line.chars().collect());
    }

    Ok(grid)
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];
const VERTICAL_DIRECTIONS: [Direction; 2] = [Direction::Up, Direction::Down];
const HORIZONTAL_DIRECTIONS: [Direction; 2] = [Direction::Left, Direction::Right];

/// Represents a direction in a 2D grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const OFFSETS: [(isize, isize); 4] = [
        (-1, 0), // Up
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
    ];

    fn to_dr(self) -> isize {
        Self::OFFSETS[self as usize].0
    }

    fn to_dc(self) -> isize {
        Self::OFFSETS[self as usize].1
    }
}

/// Represents an edge of a region in the grid.
struct Edge {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Edge {
    fn new(row: usize, col: usize, direction: Direction) -> Self {
        Self {
            row,
            col,
            direction,
        }
    }
}

/// Computes two different scoring methods for regions in the grid.
/// Returns a tuple of (original_score, sides_score) where:
/// - original_score is the sum of area * perimeter for each region
/// - sides_score is the sum of area * number of contiguous sides for each region
fn compute_scores(grid: &[Vec<char>]) -> (i64, i64) {
    let rows = grid.len();
    if rows == 0 {
        return (0, 0);
    }
    let cols = grid[0].len();

    let mut visited = vec![vec![false; cols]; rows];
    let mut original_score = 0;
    let mut sides_score = 0;

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let (area, perimeter, edges) = explore_region(grid, r, c, &mut visited);
                original_score += (area as i64) * (perimeter as i64);

                let sides = count_sides(&edges, rows, cols);
                sides_score += (area as i64) * (sides as i64);
            }
        }
    }

    (original_score, sides_score)
}

/// Explore a single region starting from (r,c).
/// Returns (area, perimeter, edges).
fn explore_region(
    grid: &[Vec<char>],
    r: usize,
    c: usize,
    visited: &mut [Vec<bool>],
) -> (usize, usize, Vec<Edge>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let letter = grid[r][c];

    let mut area = 0;
    let mut perimeter = 0;
    let mut edges = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back((r, c));
    visited[r][c] = true;

    // Iterative approach using vecdequeue for the search
    while let Some((cr, cc)) = queue.pop_front() {
        area += 1;

        for &direction in &ALL_DIRECTIONS {
            let nr = cr as isize + direction.to_dr();
            let nc = cc as isize + direction.to_dc();

            if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                perimeter += 1;
                edges.push(Edge::new(cr, cc, direction));
            } else {
                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] != letter {
                    perimeter += 1;
                    edges.push(Edge::new(cr, cc, direction));
                } else {
                    if !visited[nr][nc] {
                        visited[nr][nc] = true;
                        queue.push_back((nr, nc));
                    }
                }
            }
        }
    }

    (area, perimeter, edges)
}

/// Counts the number of contiguous sequences in a sorted array.
/// A sequence is defined as consecutive numbers that differ by 1.
///
/// # Examples
/// ```
/// assert_eq!(number_of_sequences(&vec![1, 2, 3, 5, 6, 8]), 3);
/// assert_eq!(number_of_sequences(&vec![1, 3, 5]), 3);
/// assert_eq!(number_of_sequences(&vec![1, 2, 3]), 1);
/// ```
fn number_of_sequences(array: &[usize]) -> usize {
    if array.is_empty() {
        return 0;
    }

    1 + array
        .windows(2)
        .filter(|window| window[1] != window[0] + 1)
        .count()
}

// Counts the sides as contigous sequences on the same row/col with identical direction
fn count_sides(edges: &Vec<Edge>, rows: usize, cols: usize) -> usize {
    let mut sides = 0;

    for &direction in &VERTICAL_DIRECTIONS {
        for row in 0..rows {
            let mut edge_positions: Vec<usize> = edges
                .iter()
                .filter(|edge| edge.direction == direction && edge.row == row)
                .map(|edge| edge.col)
                .collect();

            edge_positions.sort();

            sides += number_of_sequences(&edge_positions);
        }
    }

    for &direction in &HORIZONTAL_DIRECTIONS {
        for col in 0..cols {
            let mut edge_positions: Vec<usize> = edges
                .iter()
                .filter(|edge| edge.direction == direction && edge.col == col)
                .map(|edge| edge.row)
                .collect();

            edge_positions.sort();

            sides += number_of_sequences(&edge_positions);
        }
    }

    sides
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part_one_and_part_two() {
        let grid = parse_input(TEST_DATA).unwrap();
        let (original_score, sides_score) = compute_scores(&grid);

        let expected_original_score = 1930;
        assert_eq!(original_score, expected_original_score);

        let expected_sides_score = 1206;
        assert_eq!(sides_score, expected_sides_score);
    }

    #[test]
    fn test_example_with_diagonal_cutouts() {
        let input: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
        let grid = parse_input(input).unwrap();
        let (_, sides_score) = compute_scores(&grid);

        let expected_sides_score = 368;
        assert_eq!(sides_score, expected_sides_score);
    }

    // #[test]
    // fn test_part_two() {
    //     let grid = parse_input(TEST_DATA);
    //     let count = solve(&grid, true);

    //     let expected_count = 34;
    //     assert_eq!(count, expected_count);
    // }
}
