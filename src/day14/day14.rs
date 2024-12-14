use aoc2024::{Position, SimpleParse};
use itertools::Itertools;
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT, 11, 7), 12);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 0);
    }
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Position,
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = vec![];

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let parse_input = line.replace("p=", "").replace("v=", "");
        let mut split = parse_input.split_whitespace();

        let p_part = split.next().unwrap();
        let p: (i64, i64) = p_part
            .split(",")
            .take(2)
            .map(|x| x.get_i64())
            .collect_tuple()
            .unwrap();

        let v_part = split.next().unwrap();
        let v: (i64, i64) = v_part
            .split(",")
            .take(2)
            .map(|x| x.get_i64())
            .collect_tuple()
            .unwrap();

        robots.push(Robot {
            position: Position { x: p.0, y: p.1 },
            velocity: Position { x: v.0, y: v.1 },
        });
    }

    robots
}

// Determine the quadrant number of a position based on the width and
// height of the room.
//   0 -> top left
//   1 -> top right
//   2 -> bottom left
//   3 -> bottom right
// positions right on the middle are returned as None
fn quadrant(position: &Position, width: i64, height: i64) -> Option<usize> {
    let middle_x = width / 2;
    let middle_y = height / 2;

    if position.x < middle_x {
        if position.y < middle_y {
            return Some(0);
        } else if position.y > middle_y {
            return Some(2);
        } else {
            return None;
        }
    } else if position.x > middle_x {
        if position.y < middle_y {
            return Some(1);
        } else if position.y > middle_y {
            return Some(3);
        } else {
            return None;
        }
    } else {
        return None
    }
}

fn challenge1(challenge_input: &str, width: i32, height: i32) -> i64 {
    let mut robots = parse_robots(challenge_input);

    // Simulate 100s of robot movements
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.position.x = (robot.position.x + robot.velocity.x + width as i64) % width as i64;
            robot.position.y = (robot.position.y + robot.velocity.y + height as i64) % height as i64;
        }
    }

    let mut robots_per_quadrant = vec![0; 4];
    for robot in robots.iter() {
        if let Some(quadrant) = quadrant(&robot.position, width as i64, height as i64) {
            robots_per_quadrant[quadrant] += 1;
        }
    }

    robots_per_quadrant.iter().product()
}

fn challenge2(_challenge_input: &str) -> i64 {
    42
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day14/input.txt")?;

    let result1 = challenge1(&test_input, 101, 103);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}