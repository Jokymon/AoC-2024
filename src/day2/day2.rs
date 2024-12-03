use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge(SIMPLE_INPUT), 2);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 4);
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum LevelsAre {
    Unknown,
    Increasing,
    Decreasing,
    Unsafe,
}

fn rating_from_pair(first: i32, second: i32) -> LevelsAre {
    let diff = second - first;
    if (diff.abs() < 1) || (diff.abs() > 3) {
        LevelsAre::Unsafe
    } else if diff < 0 {
        LevelsAre::Decreasing
    } else {
        LevelsAre::Increasing
    }
}

fn rating_from_readings(readings: &[i32]) -> (bool, usize) {
    let mut final_rating: LevelsAre = LevelsAre::Unknown;
    let mut failure_index: usize = 0;
    let mut is_safe = true;

    let reading_iter = readings.iter();
    for (idx, (first, second)) in reading_iter.clone().zip(reading_iter.skip(1)).enumerate() {
        let rating = rating_from_pair(*first, *second);
        if final_rating == LevelsAre::Unknown {
            final_rating = rating;
        }

        if (final_rating != rating) || (rating == LevelsAre::Unsafe) {
            failure_index = idx;
            is_safe = false;
            break;
        }
    }
    (is_safe, failure_index)
}

fn challenge(challenge_input: &str) -> i32 {
    let mut safe_counter = 0;

    for line in challenge_input.split('\n') {
        let readings: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (is_safe, _) = rating_from_readings(&readings);

        if is_safe {
            safe_counter += 1;
        }
    }
    safe_counter
}

fn challenge2(challenge_input: &str) -> i32 {
    let mut safe_counter = 0;

    for line in challenge_input.split('\n') {
        let readings: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (is_safe, failure_index) = rating_from_readings(&readings);

        if is_safe {
            safe_counter += 1;
        } else {
            let mut dampened_readings = readings.to_vec();
            dampened_readings.remove(failure_index);
            let (is_safe2, _) = rating_from_readings(&dampened_readings);
            if is_safe2 {
                safe_counter += 1;
            } else if failure_index + 1 < readings.len() {
                let mut dampened_readings2 = readings.to_vec();
                dampened_readings2.remove(failure_index + 1);
                let (is_safe3, _) = rating_from_readings(&dampened_readings2);
                if is_safe3 {
                    safe_counter += 1;
                } else {
                    // Maybe the first element was the problem?
                    let mut dampened_readings3 = readings.to_vec();
                    dampened_readings3.remove(0);
                    let (is_safe4, _) = rating_from_readings(&dampened_readings3);
                    if is_safe4 {
                        safe_counter += 1;
                    }
                }
            }
        }
    }
    safe_counter
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day2/input.txt")?;

    let result = challenge(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer: {}", result);
    println!("Answer part 2: {}", result2);

    Ok(())
}
