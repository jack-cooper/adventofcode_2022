use std::{fs, str::FromStr};

use adventofcode_2022::{AnyResult, CustomError};

enum Instruction {
    AddX(i32),
    Noop,
}

struct Program {
    cycle: usize,
    register: i32,
}

impl Program {
    fn get_pixel(&self) -> char {
        if (self.cycle as i32 % 40).abs_diff(self.register) <= 1 {
            '#'
        } else {
            '.'
        }
    }
}

impl FromStr for Instruction {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Self::Noop);
        }

        let (instruction, value) = s.split_once(' ').ok_or(CustomError {
            msg: "Received a malformed instruction.".into(),
        })?;

        if instruction != "addx" {
            return Err(CustomError {
                msg: "Received a malformed instruction name.".into(),
            });
        }

        let value: i32 = value.parse().map_err(|_| CustomError {
            msg: "Recieved a non-numeric instruction value.".into(),
        })?;

        Ok(Self::AddX(value))
    }
}

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day10/input.txt")?;

    let instructions: Vec<_> = input
        .lines()
        .map(str::parse::<Instruction>)
        .collect::<Result<_, _>>()?;

    part1(&instructions)?;
    part2(&instructions)?;

    Ok(())
}

fn part1(instructions: &[Instruction]) -> AnyResult {
    let mut program = Program {
        cycle: 1,
        register: 1,
    };

    let signal_strength_sum: i32 = instructions
        .iter()
        .filter_map(|instruction| {
            let mut signal_strength: Option<i32> = None;

            match instruction {
                Instruction::AddX(value) => {
                    for _ in 0..2 {
                        if program.cycle % 40 == 20 {
                            signal_strength = Some(program.cycle as i32 * program.register);
                        }
                        program.cycle += 1;
                    }

                    program.register += value;
                }
                Instruction::Noop => {
                    if program.cycle % 40 == 20 {
                        signal_strength = Some(program.cycle as i32 * program.register);
                    }
                    program.cycle += 1;
                }
            }

            signal_strength
        })
        .sum();

    println!("Part 1 answer = {signal_strength_sum}");

    Ok(())
}

fn part2(instructions: &[Instruction]) -> AnyResult {
    use std::cell::RefCell;

    let program = RefCell::new(Program {
        cycle: 0,
        register: 1,
    });

    let crt = instructions
        .iter()
        .flat_map(|instruction| match instruction {
            Instruction::AddX(value) => {
                let pixels = (0..2).map(|add_cycle| {
                    let mut program = program.borrow_mut();

                    let pixel = program.get_pixel();

                    program.cycle += 1;

                    if add_cycle == 1 {
                        program.register += *value;
                    }

                    pixel
                });

                Box::new(pixels)
            }
            Instruction::Noop => {
                let mut program = program.borrow_mut();

                Box::new(std::iter::once({
                    let pixel = program.get_pixel();

                    program.cycle += 1;

                    pixel
                })) as Box<dyn Iterator<Item = char>>
            }
        });

    println!("Part 2 answer:");

    for (index, pixel) in crt.enumerate() {
        if index % 40 == 0 {
            println!();
        }
        print!("{pixel}");
    }
    println!();

    Ok(())
}
