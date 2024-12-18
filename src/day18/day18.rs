use aoc2024::{Field, Location, SimpleParse};
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT, 7, 12), 22);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 0);
    }
}

fn parse_input(challenge_input: &str) -> Vec<Location> {
    challenge_input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut splitter = line.split(",");
            Location {
                column: splitter.next().unwrap().get_i32(),
                row: splitter.next().unwrap().get_i32(),
            }
        })
        .collect::<Vec<Location>>()
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum MemoryLocation {
    Free(i32), // minimum amount of steps to reach this location
    Corruption,
}
type Memory = Field<MemoryLocation>;

fn challenge1(challenge_input: &str, gridsize: i32, fallen_bytes: i64) -> i64 {
    let falling_bytes_locations = parse_input(challenge_input);
    let mut memory_space: Memory =
        Field::new(vec![
            vec![MemoryLocation::Free(i32::MAX); gridsize as usize];
            gridsize as usize
        ]);

    // Corrupt the memory
    for location in falling_bytes_locations.iter().take(fallen_bytes as usize) {
        memory_space.put(location, MemoryLocation::Corruption);
    }

    // Find a path
    // The first approach used path finding similar to day16 with walkers. However that quickly
    // turned into a giant mass of walkers and a long runtime. So this implementation uses a
    // variant of flood filling the available places with the cost for reaching each space.
    memory_space.put(&Location{column: 0, row: 0}, MemoryLocation::Free(0));
    let mut open_fronts: Vec<Location> = vec![Location{ column: 0, row: 0}];
    while !open_fronts.is_empty() {
        let front = open_fronts.remove(0);

        let front_cost = match memory_space.at(&front).unwrap() {
            MemoryLocation::Corruption => i32::MAX,
            MemoryLocation::Free(cost) => cost,
        };

        for neighbor_pos in front.neighbors() {
            if let Some(neighbor_entry) = memory_space.at(&neighbor_pos) {
                match neighbor_entry {
                    MemoryLocation::Corruption => continue,
                    MemoryLocation::Free(neighbor_cost) => {
                        if front_cost+1 < neighbor_cost {
                            memory_space.put(&neighbor_pos, MemoryLocation::Free(front_cost+1));
                            open_fronts.push(neighbor_pos);
                        }
                    }
                }
            }
        }
    }

    if let Some(target_location) = memory_space.at(&Location{column: gridsize-1, row: gridsize-1}) {
        match target_location {
            MemoryLocation::Corruption => panic!("Target location should be corrupted"),
            MemoryLocation::Free(cost) => cost as i64,
        }
    } else {
        panic!("Target location is not available")
    }
}

fn challenge2(_challenge_input: &str) -> i64 {
    42
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day18/input.txt")?;

    let result1 = challenge1(&test_input, 71, 1024);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
