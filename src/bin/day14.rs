use aoc2024::fetch_or_load_input;
use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let day = 14;
    let input = fetch_or_load_input(day)?;

    let width = 101;
    let height = 103;

    let mut robots = parse_input(&input)?;
    simulate(&mut robots, width, height, 100);
    let score = compute_score(&robots, width, height);
    println!("Safety score part one: {}", score);

    let mut robots = parse_input(&input)?;
    let seconds = search_for_christmas_tree(&mut robots, width, height);
    println!("Christmas tree found after {} seconds.", seconds);

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<(i64, i64, i64, i64)>, Box<dyn Error>> {
    let mut robots = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (x, y, vx, vy) = parse_line(&line)?;
        robots.push((x, y, vx, vy));
    }
    Ok(robots)
}

fn simulate(robots: &mut Vec<(i64, i64, i64, i64)>, width: i64, height: i64, seconds: i64) {
    for robot in robots.iter_mut() {
        robot.0 = mod_wrap(robot.0 + seconds * robot.2, width);
        robot.1 = mod_wrap(robot.1 + seconds * robot.3, height);
    }
}

fn compute_score(robots: &Vec<(i64, i64, i64, i64)>, width: i64, height: i64) -> i64 {
    // Count quadrants
    let mut q_tl = 0;
    let mut q_tr = 0;
    let mut q_bl = 0;
    let mut q_br = 0;

    let x_mid = width / 2;
    let y_mid = height / 2;

    for &(x, y, _, _) in robots.iter() {
        if x == x_mid || y == y_mid {
            // no quadrant
        } else if x < x_mid && y < y_mid {
            q_tl += 1;
        } else if x > x_mid && y < y_mid {
            q_tr += 1;
        } else if x < x_mid && y > y_mid {
            q_bl += 1;
        } else if x > x_mid && y > y_mid {
            q_br += 1;
        }
    }

    let final_score = q_tl as i64 * q_tr as i64 * q_bl as i64 * q_br as i64;

    final_score
}

fn search_for_christmas_tree(
    robots: &mut Vec<(i64, i64, i64, i64)>,
    width: i64,
    height: i64,
) -> i64 {
    let mut seconds = 0;
    loop {
        seconds += 1;
        let mut grid = vec![vec![' '; height as usize]; width as usize];

        for robot in robots.iter_mut() {
            robot.0 = mod_wrap(robot.0 + robot.2, width);
            robot.1 = mod_wrap(robot.1 + robot.3, height);

            grid[robot.0 as usize][robot.1 as usize] = '*'
        }

        for ix in 3..(width as usize - 3) {
            for iy in 3..(height as usize - 3) {
                let mut found_tree = true;
                // Search for the top of the tree, there should be '*' in an angle
                for k in 0..4 {
                    if grid[ix + k][iy + k] != '*' || grid[ix + k][iy - k] != '*' {
                        found_tree = false;
                        break;
                    }
                }
                if found_tree {
                    for row in grid.iter() {
                        println!("{}", row.iter().collect::<String>());
                    }

                    println!("Seconds:{}", seconds);

                    println!("Press any key to continue...");

                    wait_for_key();
                    return seconds;
                }
            }
        }
    }
}

fn wait_for_key() {
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]); // We can skip unwrap() since we don't care about the result
}

fn parse_line(line: &str) -> Result<(i64, i64, i64, i64), Box<dyn Error>> {
    // Format: "p=62,20 v=85,-14"
    // Split by spaces
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    // parts[0]: "p=62,20"
    // parts[1]: "v=85,-14"

    let (x, y) = parse_p_part(parts[0])?;
    let (vx, vy) = parse_v_part(parts[1])?;
    Ok((x, y, vx, vy))
}

fn parse_p_part(s: &str) -> Result<(i64, i64), Box<dyn Error>> {
    // "p=62,20"
    // Remove 'p=' prefix
    let s = s.strip_prefix("p=").ok_or("Missing p=")?;
    let coords: Vec<&str> = s.split(',').collect();
    if coords.len() != 2 {
        return Err("Invalid p format".into());
    }
    let x = coords[0].parse::<i64>()?;
    let y = coords[1].parse::<i64>()?;
    Ok((x, y))
}

fn parse_v_part(s: &str) -> Result<(i64, i64), Box<dyn Error>> {
    // "v=85,-14"
    let s = s.strip_prefix("v=").ok_or("Missing v=")?;
    let coords: Vec<&str> = s.split(',').collect();
    if coords.len() != 2 {
        return Err("Invalid v format".into());
    }
    let vx = coords[0].parse::<i64>()?;
    let vy = coords[1].parse::<i64>()?;
    Ok((vx, vy))
}

/// Modular wrap for negative values
fn mod_wrap(val: i64, modulus: i64) -> i64 {
    let mut r = val % modulus;
    if r < 0 {
        r += modulus;
    }
    r
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part_one() {
        let mut robots = parse_input(TEST_DATA).unwrap();
        let width = 11;
        let height = 7;
        simulate(&mut robots, width, height, 100);
        let score = compute_score(&robots, width, height);

        let expected_score = 12;
        assert_eq!(score, expected_score);
    }
}
