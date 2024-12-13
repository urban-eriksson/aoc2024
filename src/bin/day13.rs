use aoc2024::fetch_or_load_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 13;
    let input = fetch_or_load_input(day)?;

    let mut items = parse_items(&input);

    let mut total_tokens_part_one = 0;
    for item in items.iter() {
        if let Some(tokens) = solve_item(&item) {
            total_tokens_part_one += tokens;
        }
    }
    println!("Total tokens part one: {}", total_tokens_part_one);

    let offset = 10000000000000;

    let mut total_tokens_part_two = 0;
    for item in items.iter_mut() {
        item.xp += offset;
        item.yp += offset;
        if let Some(tokens) = solve_item(&item) {
            total_tokens_part_two += tokens;
        }
    }
    println!("Total tokens part two: {}", total_tokens_part_two);

    Ok(())
}

/// Each Item consists of:
/// item[0]: "Button A: X+<XA>, Y+<YA>"
/// item[1]: "Button B: X+<XB>, Y+<YB>"
/// item[2]: "Prize: X=<XP>, Y=<YP>"
struct Item {
    xa: i64,
    ya: i64,
    xb: i64,
    yb: i64,
    xp: i64,
    yp: i64,
}

/// Parse the input lines into a vector of items.
/// Items are separated by a blank line.
fn parse_items(input: &str) -> Vec<Item> {
    let mut items = Vec::new();
    let mut current = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            // blank line, process current if 3 lines collected
            if current.len() == 3 {
                items.push(parse_item(&current));
            }
            current.clear();
        } else {
            current.push(line.to_string());
        }
    }

    // After loop, if last item is present
    if current.len() == 3 {
        items.push(parse_item(&current));
    }

    items
}

/// Parse a single item (3 lines)
fn parse_item(lines: &[String]) -> Item {
    // Button A line: "Button A: X+64, Y+22"
    // Extract XA, YA
    let (xa, ya) = parse_button_line(&lines[0]);

    // Button B line
    let (xb, yb) = parse_button_line(&lines[1]);

    // Prize line: "Prize: X=13478, Y=442"
    let (xp, yp) = parse_prize_line(&lines[2]);

    Item {
        xa,
        ya,
        xb,
        yb,
        xp,
        yp,
    }
}

/// Parse a button line like "Button A: X+64, Y+22"
fn parse_button_line(line: &str) -> (i64, i64) {
    // Example format: "Button A: X+64, Y+22"
    // We can split by ":" first
    let parts: Vec<&str> = line.split(':').collect();
    let coords_part = parts[1].trim(); // "X+64, Y+22"
                                       // Split by comma
    let parts2: Vec<&str> = coords_part.split(',').collect();
    let x_part = parts2[0].trim(); // "X+64"
    let y_part = parts2[1].trim(); // "Y+22"

    let xa = parse_coord(x_part);
    let ya = parse_coord(y_part);
    (xa, ya)
}

/// Parse something like "X+64" or "X-10"
fn parse_coord(s: &str) -> i64 {
    // s starts with X or Y and then sign and number
    // e.g. "X+64"
    let sign_number = &s[1..]; // skip 'X' or 'Y'
    sign_number.parse::<i64>().unwrap()
}

/// Parse prize line: "Prize: X=13478, Y=442"
fn parse_prize_line(line: &str) -> (i64, i64) {
    // Split by ":"
    let parts: Vec<&str> = line.split(':').collect();
    let coords_part = parts[1].trim(); // "X=13478, Y=442"

    let parts2: Vec<&str> = coords_part.split(',').collect();
    let x_part = parts2[0].trim(); // "X=13478"
    let y_part = parts2[1].trim(); // "Y=442"

    let xp = parse_prize_coord(x_part);
    let yp = parse_prize_coord(y_part);
    (xp, yp)
}

/// Parse "X=13478"
fn parse_prize_coord(s: &str) -> i64 {
    let eq_idx = s.find('=').unwrap();
    let num_str = &s[eq_idx + 1..];
    num_str.parse::<i64>().unwrap()
}

/// Solve
/// a*(xa,ya) + b*(xb,yb) = (xp,yp) =>
/// where a and b are the number of button pushes for A and B
/// This can be written as
/// xa*a + xb*b = xp
/// ya*a + yb*b = yp
/// Solving the matrix equation without taking care of edge cases gives the answer
fn solve_item(item: &Item) -> Option<usize> {
    let det = item.xa * item.yb - item.ya * item.xb;

    let numerator_a = item.xp * item.yb - item.yp * item.xb;
    let numerator_b = item.xa * item.yp - item.ya * item.xp;

    let (a, remainder_a) = (numerator_a / det, numerator_a % det);
    let (b, remainder_b) = (numerator_b / det, numerator_b % det);

    // There must be an integer number of button pushes for a solution
    if remainder_a != 0 || remainder_b != 0 {
        return None;
    }

    // No case when button A and B have the same translation so just calculate the cost
    let cost = 3 * (a as usize) + (b as usize);
    Some(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_one() {
        let items = parse_items(TEST_DATA);

        let mut total_tokens = 0;
        for item in items {
            if let Some(tokens) = solve_item(&item) {
                total_tokens += tokens;
            }
        }

        let expected_tokens = 480;
        assert_eq!(total_tokens, expected_tokens);
    }

    #[test]
    fn test_part_two() {
        let mut items = parse_items(TEST_DATA);

        let offset = 10000000000000;

        let mut claws = Vec::new();
        for (i, item) in items.iter_mut().enumerate() {
            item.xp += offset;
            item.yp += offset;
            if solve_item(&item).is_some() {
                claws.push(i + 1)
            }
        }

        let expected_claws = [2, 4];
        assert_eq!(claws, expected_claws);
    }
}
