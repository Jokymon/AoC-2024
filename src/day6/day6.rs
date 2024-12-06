use aoc2024::CharacterField;
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 41);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 0);
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Position {
    row: i32,
    column: i32,
}

fn find_start_position(input: &str) -> Position {
    for (line_no, line) in input.lines().enumerate() {
        if let Some(column) = line.find('^') {
            return Position {
                row: line_no as i32,
                column: column as i32,
            };
        }
    }
    // There should BE a start position, otherwise the quiz is wrong,
    // so this should actually never be returned
    Position { row: 0, column: 0 }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn rotate_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn position_ahead(position: Position, direction: Direction) -> Position {
    match direction {
        Direction::Up => Position {
            column: position.column,
            row: position.row - 1,
        },
        Direction::Down => Position {
            column: position.column,
            row: position.row + 1,
        },
        Direction::Right => Position {
            column: position.column + 1,
            row: position.row,
        },
        Direction::Left => Position {
            column: position.column - 1,
            row: position.row,
        },
    }
}

fn look_ahead(maze: &Vec<&str>, position: Position, direction: Direction) -> char {
    let look_at = position_ahead(position, direction);

    // We use the '%' as character to show that we are off the map
    maze.char_at(look_at.column, look_at.row).unwrap_or('%')
}

fn walk_the_maze(input: &str) -> i32 {
    let mut visited_places: Vec<Position> = vec![];
    let mut state = Direction::Up;
    let mut position = find_start_position(input);

    let maze: Vec<&str> = input.lines().collect();
    let mut ahead_of_guard = look_ahead(&maze, position, state);
    // see `look_ahead`: We use % as marker for being off the map
    while ahead_of_guard != '%' {
        match ahead_of_guard {
            '#' => {
                state = rotate_right(state);
            }
            // Somehow thought about ^ but then forgot about it again
            '.' | '^' => {
                // Late understanding: We need UNIQUE places
                if !visited_places.contains(&position) {
                    visited_places.push(position);
                }
                position = position_ahead(position, state);
            }
            _ => {}
        }
        ahead_of_guard = look_ahead(&maze, position, state);
    }
    // TODO: Need to think about this off by one issue
    visited_places.len() as i32 + 1
}

fn challenge1(challenge_input: &str) -> i32 {
    walk_the_maze(challenge_input)
}

fn challenge2(challenge_input: &str) -> i32 {
    42
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day6/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
