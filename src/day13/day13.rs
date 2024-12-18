use aoc2024::{Position, SimpleParse};
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 480);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 875318608908);
    }
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

fn position_from_string(s: &str) -> Position {
    // Using a very simple approach here by just removing all the syntactic
    // sugar in the input and then parse it as tuple formatted as space-separated
    // numbers
    let s = s
        .replace("X+", "")
        .replace("Y+", "")
        .replace("X=", "")
        .replace("Y=", "")
        .replace(",", "");
    let pair = s.to_pair_i64();
    Position {
        x: pair.0,
        y: pair.1,
    }
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut machines: Vec<ClawMachine> = vec![];

    let mut button_a = Position { x: 0, y: 0 };
    let mut button_b = Position { x: 0, y: 0 };
    let mut prize;

    for line in input.lines().filter(|line| !line.is_empty()) {
        let mut split = line.splitn(2, ":");
        match split.next() {
            Some("Button A") => {
                button_a = position_from_string(split.next().unwrap());
            }
            Some("Button B") => {
                button_b = position_from_string(split.next().unwrap());
            }
            Some("Prize") => {
                prize = position_from_string(split.next().unwrap());
                machines.push(ClawMachine {
                    button_a,
                    button_b,
                    prize,
                });
            }
            _ => (),
        }
    }

    machines
}

// Calculate the cost for winning a prize with the given claw machine. The
// function returns a `None` when there is no combination to win a prize.
// When a prize can be won, the function returns the amount of needed tokens.
fn solve_claw_machine(clawmachine: &ClawMachine) -> Option<i64> {
    let det_ab = clawmachine.button_a.x * clawmachine.button_b.y
        - clawmachine.button_a.y * clawmachine.button_b.x;
    if det_ab == 0 {
        panic!("No clawmachines with Det 0 are currently expected");
    }

    // Using Cramers rule to calculate the solution of the equation system
    let det_ap =
        clawmachine.button_a.x * clawmachine.prize.y - clawmachine.button_a.y * clawmachine.prize.x;
    let det_pb =
        clawmachine.prize.x * clawmachine.button_b.y - clawmachine.prize.y * clawmachine.button_b.x;

    if det_ap % det_ab != 0 {
        return None;
    }
    if det_pb % det_ab != 0 {
        return None;
    }

    let moves_a = det_pb / det_ab;
    let moves_b = det_ap / det_ab;

    Some(moves_a * 3 + moves_b * 1)
}

fn challenge1(challenge_input: &str) -> i64 {
    let claw_machines = parse_input(challenge_input);
    claw_machines.iter().flat_map(solve_claw_machine).sum()
}

fn challenge2(challenge_input: &str) -> i64 {
    let claw_machines = parse_input(challenge_input);
    claw_machines
        .iter()
        .flat_map(|machine| {
            solve_claw_machine(&ClawMachine {
                button_a: machine.button_a,
                button_b: machine.button_b,
                prize: Position {
                    x: machine.prize.x + 10000000000000,
                    y: machine.prize.y + 10000000000000,
                },
            })
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day13/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
