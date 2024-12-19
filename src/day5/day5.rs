use aoc2024::SimpleParse;
use std::cmp::Ordering;
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 143);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 123);
    }

    #[test]
    fn test_pages_that_satisfy_the_rules() {
        let rules = vec![(1, 3), (4, 5)];
        let page_update = vec![1, 3, 4, 5];

        assert!(update_satisfies_rules(&page_update, &rules));
    }

    #[test]
    fn test_pages_that_violate_the_rules() {
        let rules = vec![(1, 3), (4, 5)];
        let page_update = vec![1, 5, 4, 3];

        assert!(!update_satisfies_rules(&page_update, &rules));
    }
}

fn update_satisfies_rules(update: &[i32], rules: &[(i32, i32)]) -> bool {
    for first_index in 0..update.len() {
        for second_index in first_index + 1..update.len() {
            // If the reversed order of the numbers is member of the rules,
            // then they violate the rules
            if rules.contains(&(update[second_index], update[first_index])) {
                return false;
            }
        }
    }
    true
}

#[derive(PartialEq)]
enum ParsingMode {
    Rules,
    PageUpdates,
}

struct ChallengeInput {
    rules: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

fn parse_input(challenge_input: &str) -> ChallengeInput {
    let mut parsing_mode = ParsingMode::Rules;
    let mut result = ChallengeInput {
        rules: vec![],
        updates: vec![],
    };

    for line in challenge_input.lines() {
        if parsing_mode == ParsingMode::Rules {
            if line.trim() == "" {
                parsing_mode = ParsingMode::PageUpdates;
                continue;
            }
            let mut line_parser = line.split('|');
            let (left, right) = (line_parser.next().unwrap(), line_parser.next().unwrap());
            result.rules.push((left.get_i32(), right.get_i32()));
        } else {
            if line.trim() == "" {
                continue;
            }
            result
                .updates
                .push(line.split(',').map(|x| x.get_i32()).collect());
        }
    }

    result
}

fn challenge1(challenge_input: &str) -> i32 {
    let input = parse_input(challenge_input);

    let sum = input
        .updates
        .iter()
        .filter(|update| update_satisfies_rules(update, &input.rules))
        .map(|update| update.get(update.len() / 2).unwrap())
        .sum();

    sum
}

fn challenge2(challenge_input: &str) -> i32 {
    let input = parse_input(challenge_input);

    let sum = input.updates.iter().fold(0, |accu, page_update| {
        if !update_satisfies_rules(page_update, &input.rules) {
            let mut fixed_update = page_update.clone();

            fixed_update.sort_by(|&x, &y| {
                if input.rules.contains(&(x, y)) {
                    Ordering::Less
                } else if input.rules.contains(&(y, x)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            accu + fixed_update.get(fixed_update.len() / 2).unwrap()
        } else {
            accu
        }
    });

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day5/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
