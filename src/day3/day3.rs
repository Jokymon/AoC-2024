use regex::Regex;
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "#;
    const SIMPLE_INPUT2: &str = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge(SIMPLE_INPUT), 161);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT2), 48);
    }
}

fn challenge(challenge_input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let sum = re
        .captures_iter(challenge_input)
        .map(|cap| {
            let (_, [left, right]) = cap.extract();
            let left_num = left.parse::<i32>().unwrap();
            let right_num = right.parse::<i32>().unwrap();
            left_num * right_num
        })
        .sum();
    sum
}

fn challenge2(challenge_input: &str) -> i32 {
    let re = Regex::new(r"(?<op>mul|do|don't)\(((?<left>[0-9]+),(?<right>[0-9]+))?\)").unwrap();
    let mut is_enabled = true;
    let sum = re
        .captures_iter(challenge_input)
        .map(|caps| {
            let op = caps.name("op").unwrap().as_str();

            let val;
            (is_enabled, val) = match op {
                "do" => (true, 0),
                "don't" => (false, 0),
                "mul" => {
                    if is_enabled {
                        let left = caps.name("left").unwrap().as_str().parse::<i32>().unwrap();
                        let right = caps.name("right").unwrap().as_str().parse::<i32>().unwrap();
                        (true, left * right)
                    } else {
                        (false, 0)
                    }
                }
                _ => (is_enabled, 0),
            };
            val
        })
        .sum();
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day3/input.txt")?;

    let result = challenge(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result);
    println!("Answer part 2: {}", result2);

    Ok(())
}
