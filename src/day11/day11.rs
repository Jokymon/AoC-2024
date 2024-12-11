use aoc2024::SimpleParse;
use std::error::Error;
use std::fs::read_to_string;
use count_digits::CountDigits;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"125 17
    "#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 55312);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 0);
    }
}

fn challenge1(challenge_input: &str) -> i64 {
    let mut stones: Vec<_> = challenge_input
        .trim()
        .split_whitespace()
        .map(str::get_i64)
        .collect();
    for _ in 0..25 {
        stones = stones.iter().flat_map(|stone| {
            match *stone {
                0 => vec![1],
                stone if stone.count_digits()%2==0 => {
                    let total_digits = stone.count_digits();
                    let part1 = stone / i64::pow(10, (total_digits/2) as u32);
                    let part2 = stone % i64::pow(10, (total_digits/2) as u32);
                    vec![part1, part2]
                },
                _ => vec![*stone * 2024],
            }
        }).collect();
    }
    stones.iter().count() as i64
}

fn challenge2(_challenge_input: &str) -> i64 {
    42
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day11/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
