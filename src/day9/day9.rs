use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"2333133121414131402"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 1928);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 0);
    }
}

trait ToI32 {
    fn as_num(&self) -> usize;
}

impl ToI32 for char {
    fn as_num(&self) -> usize {
        (*self as usize) - ('0' as usize)
    }
}

// 1) The first impulse was to use .zip() to get a used block and a free block in combination, however
// this will potentially cause the last entry to be lost because of odd number of entries.
// 2) Turns out, the answer actually requires a i64 ;-)
fn challenge1(challenge_input: &str) -> i64 {
    let mut disk_map: Vec<_> = challenge_input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(block_index, block_size)| {
            if block_index % 2 == 0 {
                vec![(block_index / 2) as i32; block_size.as_num()]
            } else {
                vec![-1; block_size.as_num()]
            }
        })
        .collect();

    let mut to_index: usize = 0;
    let mut from_index: usize = disk_map.len() - 1;
    while from_index > to_index {
        if disk_map[to_index] == -1 {
            // Starting from the back, we could also encounter empty blocks while still having
            // empty blocks in the front. So we need to skip those too.
            while disk_map[from_index] == -1 {
                from_index -= 1;
            }
            disk_map.swap(from_index, to_index);
            from_index -= 1;
        }
        to_index += 1;
    }
    disk_map
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            index as i64
                * if *entry == -1 {
                    0 as i64
                } else {
                    *entry as i64
                }
        })
        .sum()
}

fn challenge2(_challenge_input: &str) -> i32 {
    42
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day9/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
