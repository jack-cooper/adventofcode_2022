use std::{error::Error, fs, ops::RangeInclusive};

use adventofcode_2022::{flatten_result, AnyResult, CustomError};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/bin/day4/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

type StrPair<'a> = (&'a str, &'a str);

/// Returns a pair of sections, or an error if there was no `,` character in the input.
fn get_section_pair(line: &str) -> Result<StrPair, CustomError> {
    line.split_once(',').ok_or(CustomError {
        msg: "Section pair descriptor is missing a comma.".into(),
    })
}

/// Returns 2 start/end pairs of boundaries for ranges from a section pair,
/// or an error if there was no `-` character in the input.
fn section_pair_to_range_boundaries(
    (first, second): StrPair,
) -> Result<(StrPair, StrPair), CustomError> {
    Ok((
        first.split_once('-').ok_or(CustomError {
            msg: "Missing a range start".into(),
        })?,
        second.split_once('-').ok_or(CustomError {
            msg: "Missing a range end".into(),
        })?,
    ))
}

/// Returns a pair of inclusive ranges, corresponding to the provided boundaries.
fn range_boundaries_to_ranges(
    ((first_start, first_end), (second_start, second_end)): (StrPair, StrPair),
) -> Result<(RangeInclusive<u16>, RangeInclusive<u16>), CustomError> {
    let first_start: u16 = first_start.parse().map_err(|_| CustomError {
        msg: "Invalid value for the start digit of the first of a pair of camp sections".into(),
    })?;
    let first_end: u16 = first_end.parse().map_err(|_| CustomError {
        msg: "Invalid value for the end digit of the first of a pair of camp sections".into(),
    })?;
    let second_start: u16 = second_start.parse().map_err(|_| CustomError {
        msg: "Invalid value for the start digit of the second of a pair of camp sections".into(),
    })?;
    let second_end: u16 = second_end.parse().map_err(|_| CustomError {
        msg: "Invalid value for the end digit of the second of a pair of camp sections".into(),
    })?;

    Ok((first_start..=first_end, second_start..=second_end))
}

fn part1(input: &str) -> AnyResult {
    let num_overlapping_sections = input
        .lines()
        .map(get_section_pair)
        .map(|section_ranges| section_ranges.map(section_pair_to_range_boundaries))
        .map(flatten_result)
        .map(|section_range_pairs| section_range_pairs.map(range_boundaries_to_ranges))
        .map(flatten_result)
        // Filter out ranges which don't have the shortest contained by the longest on both ends
        .filter(|range_pair| {
            let Ok((first_range, second_range)) = range_pair else {
                return false;
            };

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

    Ok(())
}

fn part2(input: &str) -> AnyResult {
    let num_overlapping_sections = input
        .lines()
        .map(get_section_pair)
        .map(|section_ranges| section_ranges.map(section_pair_to_range_boundaries))
        .map(flatten_result)
        .map(|section_range_pairs| section_range_pairs.map(range_boundaries_to_ranges))
        .map(flatten_result)
        // Filter out ranges which don't have the shortest contained by the longest on either end
        .filter(|range_pair| {
            let Ok((first_range, second_range)) = range_pair else {
                return false;
            };

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

    Ok(())
}
