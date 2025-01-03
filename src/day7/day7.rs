use std::error::Error;
use std::fs::read_to_string;
use aoc2024::SimpleParse;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 3749);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 11387);
    }

    #[test]
    fn test_concatenation() {
        assert_eq!(concat_i64(34, 234), 34234);
        assert_eq!(concat_i64(17, 100), 17100);
    }

    #[test]
    fn test_select_digit() {
        assert_eq!(select_digit(0b0100101, 2, 2), 1);
        assert_eq!(select_digit(0b0100101, 0, 2), 1);

        assert_eq!(select_digit(0 + 3*2 + 9*1 + 27*2, 0, 3), 0);
        assert_eq!(select_digit(0 + 3*2 + 9*1 + 27*2, 1, 3), 2);
        assert_eq!(select_digit(0 + 3*2 + 9*1 + 27*2, 2, 3), 1);
        assert_eq!(select_digit(0 + 3*2 + 9*1 + 27*2, 3, 3), 2);
    }
}

#[derive(Debug)]
struct Equation {
    test: i64,
    numbers: Vec<i64>,
}

fn parse_input(challenge_input: &str) -> Vec<Equation> {
    challenge_input
        .trim()
        .lines()
        .map(|line| {
            if let Some((test_str, rest)) = line.split_once(':') {
                Equation {
                    test: test_str.get_i64(),
                    numbers: rest.split_whitespace().map(str::get_i64).collect(),
                }
            } else {
                panic!("Parsing error in test input");
            }
        })
        .collect()
}

fn challenge1(challenge_input: &str) -> i64 {
    let input = parse_input(challenge_input);
    let mut sum = 0;
    const OPERATIONS: [fn(i64, i64) -> i64; 2] = [i64::wrapping_add, i64::wrapping_mul];
    input.iter().for_each(|eq| {
        let possiblities = i64::pow(2, (eq.numbers.len() - 1) as u32);
        for combination in 0..possiblities {
            let mut possibility_sum = eq.numbers[0];

            for op_position in 0..eq.numbers.len() - 1 {
                let op_index = (i64::pow(2, op_position as u32) & combination) >> op_position;
                possibility_sum =
                    OPERATIONS[op_index as usize](possibility_sum, eq.numbers[op_position + 1]);
            }

            if possibility_sum == eq.test {
                sum += eq.test;
                break
            }
        }
    });
    sum
}

fn concat_i64(a: i64, b: i64) -> i64 {
    // Code "borrowed" from https://stackoverflow.com/questions/69297477/getting-the-length-of-an-int
    let digits_b = b.checked_ilog10().unwrap_or(0) + 1;
    i64::pow(10, digits_b)*a + b
}

fn select_digit(number: i64, index: usize, base: i64) -> i64 {
    (number / i64::pow(base, index as u32)) % base
}

fn challenge2(challenge_input: &str) -> i64 {
    let input = parse_input(challenge_input);
    let mut sum = 0;
    const OPERATIONS: [fn(i64, i64) -> i64; 3] = [i64::wrapping_add, i64::wrapping_mul, concat_i64];
    input.iter().for_each(|eq| {
        let possiblities = i64::pow(3, (eq.numbers.len() - 1) as u32);
        for combination in 0..possiblities {
            let mut possibility_sum = eq.numbers[0];

            for op_position in 0..eq.numbers.len() - 1 {
                let op_index = select_digit(combination, op_position, 3);
                possibility_sum =
                    OPERATIONS[op_index as usize](possibility_sum, eq.numbers[op_position + 1]);
            }

            if possibility_sum == eq.test {
                sum += eq.test;
                break
            }
        }
    });
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day7/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
