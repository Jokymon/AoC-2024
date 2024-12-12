use aoc2024::SimpleParse;
use count_digits::CountDigits;
use memoize::memoize;
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"125 17
    "#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 55312);
    }

    // No additional testing needed for part 2
}

#[memoize]
fn stones_from_blinks(stone: i64, blinks: i32) -> i64 {
    if blinks == 0 {
        return 1;
    }
    match stone {
        0 => stones_from_blinks(1, blinks - 1),
        stone if stone.count_digits() % 2 == 0 => {
            let total_digits = stone.count_digits();

            let part1 = stone / i64::pow(10, (total_digits / 2) as u32);
            let part2 = stone % i64::pow(10, (total_digits / 2) as u32);

            stones_from_blinks(part1, blinks - 1) + stones_from_blinks(part2, blinks - 1)
        }
        _ => stones_from_blinks(stone * 2024, blinks - 1),
    }
}

fn blink_n_times(challenge_input: &str, blinks: i32) -> i64 {
    let mut stones: Vec<_> = challenge_input
        .trim()
        .split_whitespace()
        .map(str::get_i64)
        .collect();
    for _ in 0..blinks {
        stones = stones
            .iter()
            .flat_map(|stone| match *stone {
                0 => vec![1],
                stone if stone.count_digits() % 2 == 0 => {
                    let total_digits = stone.count_digits();
                    let part1 = stone / i64::pow(10, (total_digits / 2) as u32);
                    let part2 = stone % i64::pow(10, (total_digits / 2) as u32);
                    vec![part1, part2]
                }
                _ => vec![*stone * 2024],
            })
            .collect();
    }
    stones.iter().count() as i64
}

// I have to admit, I first tried just running the "functional" approach for part 1 again on
// part 2, but quickly noticed, that this might not go well. After filling the RAM up to several
// gigabytes and running for several minutes, the program finally crashed.
// So I knew, I needed a different approach and found the suggestion of memoization and handling
// entries individually on Reddit. The concept is thus based on that suggestion, but the
// implementation is mine ;-)
fn blink_n_times_memory_save(challenge_input: &str, blinks: i32) -> i64 {
    challenge_input
        .trim()
        .split_whitespace()
        .map(str::get_i64)
        .map(|stone| { stones_from_blinks }(stone, blinks))
        .sum()
}

fn challenge1(challenge_input: &str) -> i64 {
    blink_n_times(challenge_input, 25)
}

fn challenge2(challenge_input: &str) -> i64 {
    blink_n_times_memory_save(challenge_input, 75)
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day11/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
