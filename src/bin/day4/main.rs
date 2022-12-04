use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day4/input.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &str) {
    let num_overlapping_sections = input
        .lines()
        .map(|section_pair| section_pair.split(','))
        .map(|section_ranges| {
            let mut section_ranges = section_ranges.map(|section_range| {
                let mut boundaries = section_range.split('-').flat_map(str::parse::<u16>); // Questions here

                let (start, end) = (boundaries.next().unwrap(), boundaries.next().unwrap());

                start..=end
            });

            (
                section_ranges.next().unwrap(),
                section_ranges.next().unwrap(),
            )
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
        .map(|section_ranges| {
            let mut section_ranges = section_ranges.map(|section_range| {
                let mut boundaries = section_range.split('-').flat_map(str::parse::<u16>); // Questions here

                let (start, end) = (boundaries.next().unwrap(), boundaries.next().unwrap());

                start..=end
            });

            (
                section_ranges.next().unwrap(),
                section_ranges.next().unwrap(),
            )
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
