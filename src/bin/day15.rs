use aoc2024::fetch_or_load_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 15;
    let input = fetch_or_load_input(day)?;

    let (map_part_one, moves) = parse_input(&input, false)?;

    let final_map_part_one = apply_moves(map_part_one, &moves)?;
    final_map_part_one.display();
    let score_part_one = final_map_part_one.compute_score();
    println!("Score part one: {}", score_part_one);

    let (map_part_two, moves) = parse_input(&input, true)?;
    let final_map_part_two = apply_moves(map_part_two, &moves)?;
    final_map_part_two.display();
    let score_part_two = final_map_part_two.compute_score();
    println!("Score part two: {}", score_part_two);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }

    fn from_char(c: char) -> Option<Direction> {
        match c {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Robot,
    Box,
    BoxLeft,
    BoxRight,
}

impl Cell {
    fn from_char(c: char) -> Option<Cell> {
        match c {
            '.' => Some(Cell::Empty),
            '#' => Some(Cell::Wall),
            '@' => Some(Cell::Robot),
            'O' => Some(Cell::Box),
            '[' => Some(Cell::BoxLeft),
            ']' => Some(Cell::BoxRight),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Robot => '@',
            Cell::Box => 'O',
            Cell::BoxLeft => '[',
            Cell::BoxRight => ']',
        }
    }
}

struct Map {
    grid: Vec<Vec<Cell>>,
}

impl Map {
    fn new(chars: &[Vec<char>]) -> Result<Self, &'static str> {
        let grid = chars
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| Cell::from_char(c).ok_or("Invalid cell character"))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Map { grid })
    }

    fn widen(&self) -> Self {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let mut widened = vec![vec![Cell::Empty; cols * 2]; rows];

        for r in 0..rows {
            for (c, &cell) in self.grid[r].iter().enumerate() {
                let (c1, c2) = match cell {
                    Cell::Wall => (Cell::Wall, Cell::Wall),
                    Cell::Empty => (Cell::Empty, Cell::Empty),
                    Cell::Box => (Cell::BoxLeft, Cell::BoxRight),
                    Cell::Robot => (Cell::Robot, Cell::Empty),
                    _ => (Cell::Empty, Cell::Empty),
                };
                widened[r][2 * c] = c1;
                widened[r][2 * c + 1] = c2;
            }
        }

        Map { grid: widened }
    }

    fn display(&self) {
        for row in &self.grid {
            println!(
                "{}",
                row.iter().map(|cell| cell.to_char()).collect::<String>()
            );
        }
    }

    fn find_robot(&self) -> Option<(usize, usize)> {
        for (r, row) in self.grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == Cell::Robot {
                    return Some((r, c));
                }
            }
        }
        None
    }

    fn get_cell(&self, row: isize, col: isize) -> Option<Cell> {
        if row < 0 || col < 0 {
            return None;
        }
        self.grid
            .get(row as usize)
            .and_then(|r| r.get(col as usize))
            .copied()
    }

    fn set_cell(&mut self, row: isize, col: isize, cell: Cell) -> bool {
        if row < 0 || col < 0 {
            return false;
        }
        if let Some(grid_cell) = self
            .grid
            .get_mut(row as usize)
            .and_then(|r| r.get_mut(col as usize))
        {
            *grid_cell = cell;
            true
        } else {
            false
        }
    }

    fn is_move_possible(&self, pos: (isize, isize), dir: Direction) -> bool {
        let (drow, dcol) = dir.to_offset();
        let (row, col) = pos;

        match self.get_cell(row, col) {
            Some(Cell::Robot) | Some(Cell::Box) => {
                self.is_move_possible((row + drow, col + dcol), dir)
            }
            Some(Cell::BoxRight) => match dir {
                Direction::Left => self.is_move_possible((row, col - 2), dir),
                Direction::Up | Direction::Down => {
                    self.is_move_possible((row + drow, col), dir)
                        && self.is_move_possible((row + drow, col - 1), dir)
                }
                _ => false,
            },
            Some(Cell::BoxLeft) => match dir {
                Direction::Right => self.is_move_possible((row, col + 2), dir),
                Direction::Up | Direction::Down => {
                    self.is_move_possible((row + drow, col), dir)
                        && self.is_move_possible((row + drow, col + 1), dir)
                }
                _ => false,
            },
            Some(Cell::Wall) => false,
            Some(Cell::Empty) => true,
            None => false,
        }
    }

    fn perform_move(&mut self, pos: (isize, isize), dir: Direction) {
        let (drow, dcol) = dir.to_offset();
        let (row, col) = pos;
        let next_pos = (row + drow, col + dcol);

        match self.get_cell(row, col) {
            Some(Cell::Robot) => {
                self.perform_move(next_pos, dir);
                self.set_cell(next_pos.0, next_pos.1, Cell::Robot);
                self.set_cell(row, col, Cell::Empty);
            }
            Some(Cell::Box) => {
                self.perform_move(next_pos, dir);
                self.set_cell(next_pos.0, next_pos.1, Cell::Box);
            }
            Some(Cell::BoxRight) => match dir {
                Direction::Left | Direction::Right => {
                    self.perform_move(next_pos, dir);
                    self.set_cell(next_pos.0, next_pos.1, Cell::BoxRight);
                }
                Direction::Up | Direction::Down => {
                    self.perform_move(next_pos, dir);
                    self.set_cell(next_pos.0, next_pos.1, Cell::BoxRight);
                    self.perform_move((next_pos.0, next_pos.1 - 1), dir);
                    self.set_cell(next_pos.0, next_pos.1 - 1, Cell::BoxLeft);
                    self.set_cell(row, col - 1, Cell::Empty);
                }
            },
            Some(Cell::BoxLeft) => match dir {
                Direction::Left | Direction::Right => {
                    self.perform_move(next_pos, dir);
                    self.set_cell(next_pos.0, next_pos.1, Cell::BoxLeft);
                }
                Direction::Up | Direction::Down => {
                    self.perform_move(next_pos, dir);
                    self.set_cell(next_pos.0, next_pos.1, Cell::BoxLeft);
                    self.perform_move((next_pos.0, next_pos.1 + 1), dir);
                    self.set_cell(next_pos.0, next_pos.1 + 1, Cell::BoxRight);
                    self.set_cell(row, col + 1, Cell::Empty);
                }
            },
            _ => (),
        }
    }

    /// Compute score for the current map state.
    /// Score for a box at (r,c): c + 100*r
    /// Returns the sum of all box scores.
    fn compute_score(&self) -> i64 {
        let mut score = 0;
        for r in 0..self.grid.len() {
            for c in 0..self.grid[r].len() {
                match self.grid[r][c] {
                    Cell::Box | Cell::BoxLeft => {
                        score += c as i64 + 100 * (r as i64);
                    }
                    _ => continue,
                }
            }
        }
        score
    }
}

/// Parse the input lines into a map and a sequence of moves.
fn parse_input(input: &str, widen_map: bool) -> Result<(Map, Vec<Direction>), Box<dyn Error>> {
    // Split input on blank line
    let mut parts = input.split("\n\n");

    // Parse map section
    let map_str = parts.next().ok_or("Missing map section")?;
    let map_lines: Vec<String> = map_str.lines().map(|l| l.to_string()).collect();
    let char_grid: Vec<Vec<char>> = map_lines.iter().map(|l| l.chars().collect()).collect();

    // Create map
    let mut map = Map::new(&char_grid)?;
    if widen_map {
        map = map.widen();
    }

    // Parse moves section
    let moves_str = parts.next().ok_or("Missing moves section")?;
    let moves: Vec<Direction> = moves_str
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter_map(Direction::from_char)
        .collect();

    Ok((map, moves))
}

fn apply_moves(mut map: Map, moves: &[Direction]) -> Result<Map, &'static str> {
    let mut robot_pos = map.find_robot().ok_or("Robot not found")?;

    for &dir in moves {
        let pos = (robot_pos.0 as isize, robot_pos.1 as isize);
        if map.is_move_possible(pos, dir) {
            map.perform_move(pos, dir);
            let (drow, dcol) = dir.to_offset();
            robot_pos = (
                (robot_pos.0 as isize + drow) as usize,
                (robot_pos.1 as isize + dcol) as usize,
            );
        }
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn test_small_example_part_one() {
        let (map, moves) = parse_input(TEST_DATA, false).unwrap();

        let final_map = apply_moves(map, &moves).unwrap();
        final_map.display();
        let score = final_map.compute_score();

        let expected_score = 2028;
        assert_eq!(score, expected_score);
    }

    const TEST_DATA_LARGE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_large_example_part_one() {
        let (map, moves) = parse_input(TEST_DATA_LARGE, false).unwrap();

        let final_map = apply_moves(map, &moves).unwrap();
        final_map.display();
        let score = final_map.compute_score();

        let expected_score = 10092;
        assert_eq!(score, expected_score);
    }

    #[test]
    fn test_large_example_part_two() {
        let (map, moves) = parse_input(TEST_DATA_LARGE, true).unwrap();

        let final_map = apply_moves(map, &moves).unwrap();
        final_map.display();
        let score = final_map.compute_score();

        let expected_score = 9021;
        assert_eq!(score, expected_score);
    }
}
