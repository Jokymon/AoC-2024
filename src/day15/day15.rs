use std::error::Error;
use std::fs::read_to_string;

use aoc2024::{Direction, Field, Location};

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 10092);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 9021);
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Place {
    Floor,
    Box,
    Wall,
}
type Warehouse = Field<Place>;

fn parse_input(challenge_input: &str) -> (Warehouse, Location, String) {
    let mut robot_start = Location { column: 0, row: 0 };
    let mut warehouse: Vec<Vec<_>> = vec![];
    let mut instructions = "".to_string();

    #[derive(PartialEq)]
    enum ParseMode {
        Map,
        Instructions,
    }

    let mut parse_mode = ParseMode::Map;

    for (row, line) in challenge_input.lines().enumerate() {
        if parse_mode == ParseMode::Map {
            if line.trim() == "" {
                parse_mode = ParseMode::Instructions;
                continue;
            }

            let warehouse_row: Vec<Place> = line
                .chars()
                .enumerate()
                .map(|(column, ch)| match ch {
                    '.' => Place::Floor,
                    '#' => Place::Wall,
                    'O' => Place::Box,
                    '@' => {
                        robot_start = Location {
                            column: column as i32,
                            row: row as i32,
                        };
                        Place::Floor
                    }
                    _ => panic!("Unexpected room tile in input"),
                })
                .collect();

            warehouse.push(warehouse_row);
        } else {
            instructions.push_str(line.trim());
        }
    }

    (Field::new(warehouse), robot_start, instructions)
}

fn move_box(warehouse: &mut Warehouse, location: &Location, direction: Direction) -> bool {
    let target = location.in_direction(direction);
    if let Some(place) = warehouse.at(&target) {
        if place == Place::Floor || (place == Place::Box && move_box(warehouse, &target, direction))
        {
            // The target for the box is either an empty spot on the floor or we could move
            // the boxes in the move direction away. So now we must move this box
            warehouse.put(location, Place::Floor);
            warehouse.put(&target, Place::Box);
            true
        } else {
            false
        }
    } else {
        // There are no more places left in this movement direction, so don't move
        false
    }
}

fn challenge1(challenge_input: &str) -> i64 {
    let (mut warehouse, mut robot_position, instructions) = parse_input(challenge_input);
    for movement_instruction in instructions.chars() {
        let movement: Direction = movement_instruction.into();

        let target = robot_position.in_direction(movement);
        if let Some(place) = warehouse.at(&target) {
            if place == Place::Floor
                || (place == Place::Box && move_box(&mut warehouse, &target, movement))
            {
                robot_position = target;
            }
        }
    }

    warehouse
        .each_location()
        .map(|(location, place)| match place {
            Place::Box => (100 * location.row + location.column) as i64,
            _ => 0,
        })
        .sum()
}

fn challenge2(_challenge_input: &str) -> i64 {
    42
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day15/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
