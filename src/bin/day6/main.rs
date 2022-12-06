use std::{collections::HashSet, fs};

use adventofcode_2022::{AnyResult, CustomError};

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day6/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn find_unique_window_index(datastream: &[u8], size: usize) -> Option<usize> {
    datastream
        .windows(size)
        .enumerate()
        .find_map(|(index, window)| {
            let mut seen_values = HashSet::with_capacity(size);

            window
                .iter()
                .all(|value| seen_values.insert(value))
                .then_some(index + size)
        })
}

fn part1(input: &str) -> AnyResult {
    let start_of_packet_marker =
        find_unique_window_index(input.as_bytes(), 4).ok_or(CustomError {
            msg: "No valid start-of-packet marker detected.".into(),
        })?;

    println!("Part 1 answer = {start_of_packet_marker}");

    Ok(())
}

fn part2(input: &str) -> AnyResult {
    let start_of_message_marker =
        find_unique_window_index(input.as_bytes(), 14).ok_or(CustomError {
            msg: "No valid start-of-message marker detected.".into(),
        })?;

    println!("Part 2 answer = {start_of_message_marker}");

    Ok(())
}
