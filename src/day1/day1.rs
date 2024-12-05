use aoc2024::SimpleParse;
use itertools::Itertools;
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge(SIMPLE_INPUT), 11);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 31);
    }
}

fn challenge(challenge_input: &str) -> i32 {
    let l: Vec<(i32, i32)> = challenge_input.lines().map(|l| l.to_pair()).collect();
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = l.into_iter().unzip();
    left.sort();
    right.sort();

    left.iter().zip(&right).map(|(l, r)| (l - r).abs()).sum()
}

fn challenge2(challenge_input: &str) -> i32 {
    let l: Vec<(i32, i32)> = challenge_input.lines().map(|l| l.to_pair()).collect();
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = l.into_iter().unzip();
    left.sort();
    right.sort();

    let freqs = right.iter().counts();

    left.iter()
        .map(|l| l * *freqs.get(l).unwrap_or(&0) as i32)
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day1/input.txt")?;

    let result = challenge(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer: {}", result);
    println!("Answer part 2: {}", result2);

    Ok(())
}
