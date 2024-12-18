use aoc2024::{Direction, DirectionRelative, Field, Location};
use itertools::Itertools;
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const SIMPLE_INPUT2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 7036);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 45);
    }

    #[test]
    fn test_simple_input_part2_with_second_maze() {
        assert_eq!(challenge2(SIMPLE_INPUT2), 64);
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Place {
    Wall,
    Walkable(i32), // the number is a cost to reach this place from start
}
type Maze = Field<Place>;

fn parse_input(challenge_input: &str) -> (Maze, Location, Location) {
    let mut maze: Vec<Vec<_>> = vec![];
    let mut reindeer_start = Location { column: 0, row: 0 };
    let mut reindeer_end = Location { column: 0, row: 0 };

    for (row, line) in challenge_input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        let maze_row: Vec<Place> = line
            .chars()
            .enumerate()
            .map(|(column, ch)| match ch {
                '#' => Place::Wall,
                '.' => Place::Walkable(i32::MAX),
                'S' => {
                    reindeer_start = Location {
                        column: column as i32,
                        row: row as i32,
                    };
                    Place::Walkable(i32::MAX)
                }
                'E' => {
                    reindeer_end = Location {
                        column: column as i32,
                        row: row as i32,
                    };
                    Place::Walkable(i32::MAX)
                }
                _ => panic!("Wrong entry in maze: {}", ch),
            })
            .collect();
        maze.push(maze_row);
    }

    (Field::new(maze), reindeer_start, reindeer_end)
}

#[derive(Debug)]
struct Walker {
    direction: Direction,
    location: Location,
    accumulated_cost: i32,
    walked_tiles: Vec<Location>,
}

fn walk_maze(
    maze: &mut Maze,
    start: &Location,
    end: &Location,
    initial_direction: Direction,
) -> Vec<Walker> {
    let mut fronts: Vec<Walker> = vec![Walker {
        direction: initial_direction,
        location: *start,
        accumulated_cost: 0,
        walked_tiles: vec![*start],
    }];
    let mut finished_walkers: Vec<Walker> = vec![];

    while !fronts.is_empty() {
        let walker = fronts.remove(0);
        if walker.location == *end {
            finished_walkers.push(walker);
            continue;
        }

        // As cost, we take the cost for first turning in that direction and the moving there.
        // Since the turning cost is 1000 and moving forward is 1, the total cost for turning
        // and moving is 1001.
        for (relative_direction, cost) in [
            (DirectionRelative::Left, 1001),
            (DirectionRelative::Forward, 1),
            (DirectionRelative::Right, 1001),
        ] {
            let new_spot = walker
                .location
                .in_direction(walker.direction + relative_direction);
            if let Some(maze_spot) = maze.at(&new_spot) {
                match maze_spot {
                    Place::Wall => continue, // Nothing to gain in this direction, we're walking into a wall
                    Place::Walkable(place_cost) => {
                        let new_walker_cost = walker.accumulated_cost + cost;
                        // We give the new walker some slack, because he might already be
                        // correctly oriented for the next step, while the cost on the tile
                        // might not already include the cost for turning
                        if place_cost < new_walker_cost - 1000 {
                            // We can skip this walker, it's not gonna get to the target faster
                            continue;
                        }
                        fronts.push(Walker {
                            direction: walker.direction + relative_direction,
                            location: new_spot,
                            accumulated_cost: new_walker_cost,
                            walked_tiles: {
                                let mut new_tiles = walker.walked_tiles.clone();
                                new_tiles.push(new_spot);
                                new_tiles
                            },
                        });
                        maze.put(&new_spot, Place::Walkable(new_walker_cost));
                    }
                }
            }
            // else this spot isn't even on the map
        }
    }

    finished_walkers
}

/// Walk the maze from start to end and determine the cost
fn walk_maze_cost(
    maze: &mut Maze,
    start: &Location,
    end: &Location,
    initial_direction: Direction,
) -> i64 {
    let finished_walkers = walk_maze(maze, start, end, initial_direction);

    finished_walkers
        .iter()
        .map(|walker| walker.accumulated_cost)
        .fold(i32::MAX, i32::min) as i64
}

fn walk_best_spots(
    maze: &mut Maze,
    start: &Location,
    end: &Location,
    initial_direction: Direction,
) -> i64 {
    let finished_walkers = walk_maze(maze, start, end, initial_direction);

    let minimum_cost = finished_walkers
        .iter()
        .map(|walker| walker.accumulated_cost)
        .fold(i32::MAX, i32::min);

    finished_walkers
        .iter()
        .filter(|walker: &&Walker| walker.accumulated_cost == minimum_cost)
        .flat_map(|walker| walker.walked_tiles.clone())
        .unique()
        .count() as i64
}

fn challenge1(challenge_input: &str) -> i64 {
    let (mut maze, start, end) = parse_input(challenge_input);

    walk_maze_cost(&mut maze, &start, &end, Direction::Right)
}

fn challenge2(challenge_input: &str) -> i64 {
    let (mut maze, start, end) = parse_input(challenge_input);

    walk_best_spots(&mut maze, &start, &end, Direction::Right)
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day16/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
