
use aoc2024::fetch_or_load_input;
use std::error::Error;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 3;
    let input = fetch_or_load_input(day)?;

    let tokens = tokenize_input(&input)?;

    let total_part_one = process_tokens(&tokens); 
    println!("Total part one: {}", total_part_one);

    let total_part_two = process_tokens_with_dos_and_donts(&tokens);    
    println!("Total with do() and don't(): {}", total_part_two);

    Ok(())
}

#[derive(Debug)]
enum Token {
    Do,
    Dont,
    Mul(i32, i32),
}

struct PositionedToken {
    token: Token,
    position: usize,
}

fn tokenize_input(input: &str) -> Result<Vec<PositionedToken>, Box<dyn Error>> {
    // Regular expressions for the tokens
    let re_mul = Regex::new(r"mul\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)")?;
    let re_do = Regex::new(r"do\(\)")?;
    let re_dont = Regex::new(r"don't\(\)")?;

    // Collect all matches with their positions
    let mut tokens = Vec::new();

    for mat in re_mul.find_iter(input) {
        let cap = re_mul.captures(&input[mat.start()..mat.end()]).unwrap();
        let x: i32 = cap[1].parse()?;
        let y: i32 = cap[2].parse()?;
        tokens.push(PositionedToken {
            token: Token::Mul(x, y),
            position: mat.start(),
        });
    }

    for mat in re_do.find_iter(input) {
        tokens.push(PositionedToken {
            token: Token::Do,
            position: mat.start(),
        });
    }

    for mat in re_dont.find_iter(input) {
        tokens.push(PositionedToken {
            token: Token::Dont,
            position: mat.start(),
        });
    }

    // Sort tokens by their starting position
    tokens.sort_by_key(|t| t.position);

    Ok(tokens)
}

fn process_tokens(tokens: &[PositionedToken]) -> i32 {
    let mut total = 0;

    for token in tokens {
        if let Token::Mul(x,y) = token.token {
            total += x * y;
        }
    }

    total
}

fn process_tokens_with_dos_and_donts(tokens: &[PositionedToken]) -> i32 {
    let mut enabled = true; // Initial state is enabled
    let mut total = 0;

    for token in tokens {
        match token.token {
            Token::Do => {
                enabled = true;
            }
            Token::Dont => {
                enabled = false;
            }
            Token::Mul(x, y) => {
                if enabled {
                    total += x * y;
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let tokens = tokenize_input(input).unwrap();
        let total = process_tokens(&tokens);
        let expected_total = 161;
        assert_eq!(total, expected_total);
    }

    #[test]
    fn test_input_with_dos_and_donts() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let tokens = tokenize_input(input).unwrap();
        let total = process_tokens_with_dos_and_donts(&tokens);
        let expected_total = 48;
        assert_eq!(total, expected_total);
    }

}