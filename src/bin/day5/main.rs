use std::fs;

use once_cell::sync::Lazy;

use adventofcode_2022::{AnyResult, CustomError};
use regex::Regex;

static COMMAND_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("move (?P<amount>\\d+) from (?P<source>\\d) to (?P<destination>\\d)").unwrap()
});

#[derive(Clone, Debug)]
struct Crate {
    label: char,
}

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day5/input.txt")?;

    let starting_crates: Vec<_> = input.lines().take(8).collect();

    let initial_cargo_bay = starting_crates
        .into_iter()
        .rev()
        .flat_map(|line| line.chars().skip(1).step_by(4).enumerate())
        .fold(vec![vec![]; 9], |mut cargo_bay, (stack_index, label)| {
            if label != ' ' {
                cargo_bay[stack_index].push(Crate { label });
            }

            cargo_bay
        });

    part1(&input, initial_cargo_bay.clone())?;
    part2(&input, initial_cargo_bay)?;

    Ok(())
}

fn part1(input: &str, mut cargo_bay: Vec<Vec<Crate>>) -> AnyResult {
    input
        .lines()
        .skip(10)
        .map(|command| {
            let captures = COMMAND_REGEX.captures(command).ok_or(CustomError {
                msg: "Command regex failed to parse a command.".into(),
            })?;

            // The below unwraps are safe to do as they are from a
            // Regex "\d" capture

            let amount = captures
                .name("amount")
                .ok_or(CustomError {
                    msg: "Command was missing an `amount`.".into(),
                })?
                .as_str()
                .parse::<usize>()
                .unwrap();

            let source = captures
                .name("source")
                .ok_or(CustomError {
                    msg: "Command was missing a `source`.".into(),
                })?
                .as_str()
                .parse::<usize>()
                .unwrap()
                - 1;

            let destination = captures
                .name("destination")
                .ok_or(CustomError {
                    msg: "Command was missing a `destination`.".into(),
                })?
                .as_str()
                .parse::<usize>()
                .unwrap()
                - 1;

            Ok::<_, CustomError>((amount, source, destination))
        })
        .try_for_each(|command| {
            command.map(|(amount, source_index, destination_index)| {
                (0..amount).for_each(|_| {
                    let cargo_crate = cargo_bay[source_index]
                        .pop()
                        .expect("Attempted to remove a crate from an empty stack.");

                    cargo_bay[destination_index].push(cargo_crate);
                });
            })
        })?;

    let tops_of_stacks: String = cargo_bay
        .into_iter()
        .filter_map(|mut stack| stack.pop().map(|cargo_crate| cargo_crate.label))
        .collect();

    println!("Part 1 answer = {tops_of_stacks}");

    Ok(())
}

fn part2(input: &str, mut cargo_bay: Vec<Vec<Crate>>) -> AnyResult {
    input
        .lines()
        .skip(10)
        .map(|command| {
            let captures = COMMAND_REGEX.captures(command).ok_or(CustomError {
                msg: "Command regex failed to parse a command.".into(),
            })?;

            // The below unwraps are safe to do as they are from a
            // Regex "\d" capture

            let amount = captures
                .name("amount")
                .ok_or(CustomError {
                    msg: "Command was missing an `amount`.".into(),
                })?
                .as_str()
                .parse::<usize>()
                .unwrap();

            let source = captures
                .name("source")
                .ok_or(CustomError {
                    msg: "Command was missing a `source`.".into(),
                })?
                .as_str()
                .parse::<usize>()
                .unwrap()
                - 1;

            let destination = captures
                .name("destination")
                .ok_or(CustomError {
                    msg: "Command was missing a `destination`.".into(),
                })?
                .as_str()
                .parse::<usize>()
                .unwrap()
                - 1;

            Ok::<_, CustomError>((amount, source, destination))
        })
        .try_for_each(|command| {
            command.map(|(amount, source_index, destination_index)| {
                let source = &mut cargo_bay[source_index];

                let mut crates_to_be_moved = source.split_off(source.len() - amount);

                cargo_bay[destination_index].append(&mut crates_to_be_moved);
            })
        })?;

    let tops_of_stacks: String = cargo_bay
        .into_iter()
        .filter_map(|mut stack| stack.pop().map(|cargo_crate| cargo_crate.label))
        .collect();

    println!("Part 2 answer = {tops_of_stacks}");

    Ok(())
}
