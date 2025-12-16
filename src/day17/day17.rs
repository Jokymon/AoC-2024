use std::error::Error;
use std::fs::read_to_string;

use aoc2024::SimpleParse;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 0);
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: i32,
    operand: i32,
}

#[derive(Debug)]
struct Computer {
    register_a: i32,
    register_b: i32,
    register_c: i32,

    instruction_pointer: i32,

    program: Vec<Instruction>,
}

const ADV: i32 = 0;
const BXL: i32 = 1;
const BST: i32 = 2;
const JNZ: i32 = 3;
const BXC: i32 = 4;
const OUT: i32 = 5;
const BDV: i32 = 6;
const CDV: i32 = 7;

fn mnemonic(opcode: i32) -> String {
    match opcode {
        ADV => "ADV",
        BXL => "BXL",
        BST => "BST",
        JNZ => "JNZ",
        BXC => "BXC",
        OUT => "OUT",
        BDV => "BDV",
        CDV => "CDV",
        _ => "ILGL!",
    }
    .to_string()
}

impl Computer {
    // determine the actual value of a "combo" operand
    fn combo(&self, operand: i32) -> i32 {
        match operand {
            0..=3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Illegal combo operand {}", operand),
        }
    }

    fn run_to_completion(&mut self) -> String {
        println!("Program: {:?}", self.program);
        let mut outputs: Vec<i32> = vec![];

        while self.instruction_pointer < self.program.len() as i32 {
            let Instruction { opcode, operand } = self.program[self.instruction_pointer as usize];
            // println!(
            //     "IP: {}, opcode={}, operand={}, A={}, B={}, C={}",
            //     self.instruction_pointer,
            //     mnemonic(opcode),
            //     operand,
            //     self.register_a,
            //     self.register_b,
            //     self.register_c
            // );
            self.instruction_pointer += 1;

            match opcode {
                ADV => self.register_a >>= self.combo(operand) as u32,
                BXL => self.register_b ^= operand,
                BST => self.register_b = self.combo(operand) % 8,
                JNZ => {
                    if self.register_a != 0 {
                        self.instruction_pointer = operand / 2 // our instructions are index together with the operands
                    }
                }
                BXC => self.register_b ^= self.register_c,
                OUT => outputs.push(self.combo(operand) % 8),
                BDV => self.register_b = self.register_a / i32::pow(2, self.combo(operand) as u32),
                CDV => self.register_c = self.register_a / i32::pow(2, self.combo(operand) as u32),
                _ => panic!(
                    "Illegal opcode {} at IP={}",
                    opcode,
                    self.instruction_pointer - 1
                ),
            }
        }
        outputs.into_iter().join(",")
    }
}

fn parse_input(challenge_input: &str) -> Computer {
    let mut computer = Computer {
        register_a: 0,
        register_b: 0,
        register_c: 0,
        instruction_pointer: 0,
        program: vec![],
    };
    let (registers, program) = challenge_input.splitn(2, "\n\n").collect_tuple().unwrap();

    for line in registers.trim().lines() {
        if line.starts_with("Register A") {
            computer.register_a = line.replace("Register A: ", "").get_i32();
        } else if line.starts_with("Register B") {
            computer.register_b = line.replace("Register B: ", "").get_i32();
        } else if line.starts_with("Register C") {
            computer.register_c = line.replace("Register C: ", "").get_i32();
        }
    }

    let program_code = program.replace("Program: ", "");
    let program_iter = program_code
        .trim()
        .split(",")
        .map(SimpleParse::get_i32);

    computer.program = program_iter
        .clone()
        .step_by(2)
        .zip(program_iter.skip(1).step_by(2))
        .map(|(opcode, operand)| Instruction { opcode, operand })
        .collect();

    computer
}

fn challenge1(challenge_input: &str) -> String {
    let mut computer = parse_input(challenge_input);

    computer.run_to_completion()
}

fn challenge2(_challenge_input: &str) -> i64 {
    42
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day17/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
