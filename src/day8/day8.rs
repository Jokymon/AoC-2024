use aoc2024::CharacterField;
use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::ops::{Add, Sub};

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 14);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 34);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn challenge1(challenge_input: &str) -> i32 {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    let antenna_map: Vec<_> = challenge_input.lines().collect();

    // Get all antennas from the map
    antenna_map.iter().enumerate().for_each(|(line_no, line)| {
        line.chars().enumerate().for_each(|(char_no, antenna)| {
            if antenna != '.' {
                if !antennas.contains_key(&antenna) {
                    antennas.insert(antenna, vec![]);
                }

                antennas.get_mut(&antenna).unwrap().push(Position {
                    x: char_no as i32,
                    y: line_no as i32,
                });
            }
        });
    });

    // Calculate the positions of the antinodes per antenna type
    let mut antinodes: Vec<Position> = Vec::new();
    antennas.iter().for_each(|(_antenna, positions)| {
        positions.into_iter().combinations(2).for_each(|pair| {
            let position1 = pair.first().unwrap();
            let position2 = pair.last().unwrap();

            let distance_vector = **position2 - **position1;
            let antinode1 = **position1 - distance_vector;
            if antenna_map.has_position(antinode1.x, antinode1.y) && !antinodes.contains(&antinode1)
            {
                antinodes.push(antinode1);
            }
            let antinode2 = **position2 + distance_vector;
            if antenna_map.has_position(antinode2.x, antinode2.y) && !antinodes.contains(&antinode2)
            {
                antinodes.push(antinode2);
            }
        });
    });

    antinodes.len() as i32
}

fn challenge2(challenge_input: &str) -> i32 {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    let antenna_map: Vec<_> = challenge_input.lines().collect();

    // Get all antennas from the map
    antenna_map.iter().enumerate().for_each(|(line_no, line)| {
        line.chars().enumerate().for_each(|(char_no, antenna)| {
            if antenna != '.' {
                if !antennas.contains_key(&antenna) {
                    antennas.insert(antenna, vec![]);
                }

                antennas.get_mut(&antenna).unwrap().push(Position {
                    x: char_no as i32,
                    y: line_no as i32,
                });
            }
        });
    });

    // Calculate the positions of the antinodes per antenna type
    let mut antinodes: Vec<Position> = Vec::new();
    antennas.iter().for_each(|(_antenna, positions)| {
        positions.into_iter().combinations(2).for_each(|pair| {
            // The &&Position type here is not entirely clear to me, maybe this can be simplified?
            let position1 = pair.first().unwrap();
            let position2 = pair.last().unwrap();

            // Addition 1 for getting part 2 to work
            if !antinodes.contains(position1) {
                antinodes.push(**position1);
            }
            if !antinodes.contains(position2) {
                antinodes.push(**position2);
            }

            let distance_vector = **position2 - **position1;

            let mut antinode1 = **position1 - distance_vector;
            // The addition 2 to get part 2 to work was just turning the `if` into a while and
            // adding/subtracting the `distance_vector` over and over again
            while antenna_map.has_position(antinode1.x, antinode1.y) {
                if !antinodes.contains(&antinode1) {
                    antinodes.push(antinode1);
                }
                antinode1 = antinode1 - distance_vector;
            }
            let mut antinode2 = **position2 + distance_vector;
            while antenna_map.has_position(antinode2.x, antinode2.y) {
                if !antinodes.contains(&antinode2) {
                    antinodes.push(antinode2);
                }
                antinode2 = antinode2 + distance_vector;
            }
        });
    });

    antinodes.len() as i32
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day8/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
