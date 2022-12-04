use std::{error::Error, fs};

use adventofcode_2022::CustomError;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/bin/day4/input.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &str) {
    let num_overlapping_sections = input
        .lines()
        .map(|section_pair| section_pair.split(','))
        .flat_map(|section_ranges| {
            let mut section_ranges = section_ranges.flat_map(|section_range| {
                let mut boundaries = section_range.split('-').flat_map(str::parse::<u16>);

                let (start, end) = (
                    boundaries
                        .next()
                        .ok_or_else(|| CustomError::new("Missing a start for a range."))?,
                    boundaries
                        .next()
                        .ok_or_else(|| CustomError::new("Missing an end for a range."))?,
                );

                Ok::<_, CustomError>(start..=end)
            });

            Ok::<_, CustomError>((
                section_ranges.next().ok_or_else(|| {
                    CustomError::new("Missing the first of a section range pair.")
                })?,
                section_ranges.next().ok_or_else(|| {
                    CustomError::new("Missing the second of a section range pair.")
                })?,
            ))
        })
        .filter(|(first_range, second_range)| {
            if first_range.len() > second_range.len() {
                first_range.contains(second_range.start())
                    && first_range.contains(second_range.end())
            } else {
                second_range.contains(first_range.start())
                    && second_range.contains(first_range.end())
            }
        })
        .count();

    println!("Part 1 answer = {num_overlapping_sections}");
}

fn part2(input: &str) {
    let num_overlapping_sections = input
        .lines()
        .map(|section_pair| section_pair.split(','))
        .flat_map(|section_ranges| {
            let mut section_ranges = section_ranges.flat_map(|section_range| {
                let mut boundaries = section_range.split('-').flat_map(str::parse::<u16>);

                let (start, end) = (
                    boundaries
                        .next()
                        .ok_or_else(|| CustomError::new("Missing a start for a range."))?,
                    boundaries
                        .next()
                        .ok_or_else(|| CustomError::new("Missing an end for a range."))?,
                );

                Ok::<_, CustomError>(start..=end)
            });

            Ok::<_, CustomError>((
                section_ranges.next().ok_or_else(|| {
                    CustomError::new("Missing the first of a section range pair.")
                })?,
                section_ranges.next().ok_or_else(|| {
                    CustomError::new("Missing the second of a section range pair.")
                })?,
            ))
        })
        .filter(|(first_range, second_range)| {
            if first_range.len() > second_range.len() {
                first_range.contains(second_range.start())
                    || first_range.contains(second_range.end())
            } else {
                second_range.contains(first_range.start())
                    || second_range.contains(first_range.end())
            }
        })
        .count();

    println!("Part 2 answer = {num_overlapping_sections}");
}
