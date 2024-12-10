use std::error::Error;
use std::fs::read_to_string;
use aoc2024::{CharacterField, SimpleChar};

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 36);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 0);
    }
}

#[derive(Debug, PartialEq)]
struct Explorer {
    id: i32,
    x: i32,
    y: i32,
    height: i32,
}

// Basic idea: we create one "explorer" (as in real life person) or group, that starts from one of the trail heads.
// Such an explorer will then go and try to find a summit and split up into more groups at every point where there
// are multiple choices to continue. This function basically extracts the trail heads.
fn get_explorers(map: &Vec<&str>) -> Vec<Explorer> {
    let mut explorers: Vec<_> = vec![];
    let mut id = 0;
    for (line_no, line) in map.iter().enumerate() {
        for (char_pos, ch) in line.chars().enumerate() {
            if ch == '0' {
                explorers.push(Explorer {
                    id: id,
                    x: char_pos as i32,
                    y: line_no as i32,
                    height: 0,
                });
                id = id + 1;
            }
        }
    }
    explorers
}

fn next_steps(map: &Vec<&str>, explorer: &Explorer) -> Vec<Explorer> {
    let mut new_explorers: Vec<_> = vec![];
    // Find next step in the neighbourhood of this explorer
    // For every possible next step, we create a new explorer
    // REMARK: Initially I iterated with dx, dy from -1 to 1. However this is
    // wrong since that way we would also walk diagonally
    for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
        if let Some(height) = map.char_at(explorer.x + dx, explorer.y + dy) {
            if height.as_i32() == explorer.height + 1 {
                new_explorers.push(Explorer {
                    id: explorer.id,
                    x: explorer.x + dx,
                    y: explorer.y + dy,
                    height: height.as_i32(),
                });
            }
        }
    }
    new_explorers
}

fn challenge1(challenge_input: &str) -> i32 {
    let map_repr: Vec<_> = challenge_input.trim().lines().collect();
    let mut explorers = get_explorers(&map_repr);
    let mut trailhead_summit: Vec<Explorer> = vec![];
    let mut total_score = 0;

    while !explorers.is_empty() {
        // We try to follow every explorer to a summit
        let mut front = explorers.pop().unwrap();

        let mut new_explorers = next_steps(&map_repr, &front);
        while !new_explorers.is_empty() && front.height != 9 {
            front = new_explorers.remove(0);

            // If we reach a split, we create a new explorer and must follow one
            // of the explorers to the summit
            for explorer in new_explorers {
                explorers.push(explorer);
            }
            new_explorers = next_steps(&map_repr, &front);
        }

        // Once we actually reach the top, we check if we already reached this
        // same summit from same trailhead
        if front.height == 9 && !trailhead_summit.contains(&front) {
            trailhead_summit.push(front);
            total_score += 1;
        }
    }
    total_score
}

fn challenge2(_challenge_input: &str) -> i32 {
    42
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day10/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
