use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 6);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 0);
    }
}

fn parse_input(input: &str) -> (regex::Regex, Vec<&str>) {
    let (available_patterns, desired_designs) = input.splitn(2, "\n\n").collect_tuple().unwrap();

    let regex_string = format!("^({})+$", available_patterns.split(",").map(str::trim).join("|"));

    (
        Regex::new(&regex_string).unwrap(),
        desired_designs.split_whitespace().collect(),
    )
}

fn challenge1(challenge_input: &str) -> i64 {
    let (towel_rules, designs) = parse_input(challenge_input);

    designs.iter().filter(|design| {
        towel_rules.is_match(design)
    }).count() as i64
}

fn challenge2(_challenge_input: &str) -> i64 {
    42
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day19/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
