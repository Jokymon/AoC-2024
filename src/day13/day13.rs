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
        assert_eq!(challenge2(SIMPLE_INPUT), 0);
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

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

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
    let det = clawmachine.button_a.x * clawmachine.button_b.y
        - clawmachine.button_a.y * clawmachine.button_b.x;
    if det == 0 {
        panic!("No clawmachines with Det 0 are currently expected");
    }
    let divisor: f64 = clawmachine.button_b.x as f64
        - (clawmachine.button_b.y as f64 * clawmachine.button_a.x as f64
            / clawmachine.button_a.y as f64);
    let righ_hand: f64 = clawmachine.prize.x as f64
        - clawmachine.prize.y as f64
            * (clawmachine.button_a.x as f64 / clawmachine.button_a.y as f64);
    let moves_b = (righ_hand / divisor).round();
    if moves_b - moves_b.floor() != 0.0 {
        return None;
    }

    let moves_a = (clawmachine.prize.x as f64 - moves_b * clawmachine.button_b.x as f64)
        / clawmachine.button_a.x as f64;

    // Hmm, this feels like an ugly hack - we use round() on moves_b to make
    // sure we only get integer solutions. We should already get integer
    // solutions by checking for fractional parts that are very close to an
    // integer. This works for the first part, but somehow fails for part 2.
    if moves_a as i64 * clawmachine.button_a.x + moves_b as i64 * clawmachine.button_b.x
        != clawmachine.prize.x
    {
        return None;
    }
    if moves_a as i64 * clawmachine.button_a.y + moves_b as i64 * clawmachine.button_b.y
        != clawmachine.prize.y
    {
        return None;
    }

    Some(moves_a as i64 * 3 + moves_b as i64 * 1)
}

fn challenge1(challenge_input: &str) -> i64 {
    let claw_machines = parse_input(challenge_input);
    claw_machines.iter().map(solve_claw_machine).flatten().sum()
}

fn challenge2(challenge_input: &str) -> i64 {
    let claw_machines = parse_input(challenge_input);
    claw_machines
        .iter()
        .map(|machine| {
            solve_claw_machine(&ClawMachine {
                button_a: machine.button_a,
                button_b: machine.button_b,
                prize: Position {
                    x: machine.prize.x + 10000000000000,
                    y: machine.prize.y + 10000000000000,
                },
            })
        })
        .flatten()
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
