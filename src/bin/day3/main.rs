use std::{collections::HashSet, fs};

use adventofcode_2022::{AnyResult, CustomError};

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day3/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> AnyResult {
    let priority_sum = input
        .lines()
        // Split the rucksack into 2 compartments, and convert the first into a set
        .map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);

            let first_compartment_item_types: HashSet<_> = first.chars().collect();

            (second, first_compartment_item_types)
        })
        .map(find_duplicate_item_type)
        .map(|duplicate| duplicate.map(get_priority))
        .sum::<Result<u32, _>>()?;

    println!("Part 1 answer = {priority_sum}");

    Ok(())
}

fn part2(input: &str) -> AnyResult {
    let mut rucksacks: Vec<_> = input.lines().collect();

    let priority_sum: u32 = rucksacks
        .chunks_exact_mut(3)
        // Converts the shortest 2 rucksacks into sets, takes their intersection,
        // then iterates through the longest until it finds a duplicate
        .map(|squad_rucksacks| {
            squad_rucksacks.sort_unstable_by_key(|rucksack| rucksack.len());

            let first_item_types: HashSet<_> = squad_rucksacks[0].chars().collect();
            let second_item_types: HashSet<_> = squad_rucksacks[1].chars().collect();

            let shared_item_types: HashSet<_> = first_item_types
                .intersection(&second_item_types)
                .copied()
                .collect();

            (squad_rucksacks[2], shared_item_types)
        })
        .map(find_duplicate_item_type)
        .map(|duplicate| duplicate.map(get_priority))
        .sum::<Result<u32, _>>()?;

    println!("Part 2 answer = {priority_sum}");

    Ok(())
}

/// Returns the first item from `container` which is in `already_seen_items`,
/// or an error if none exists.
fn find_duplicate_item_type(
    (container, already_seen_items): (&str, HashSet<char>),
) -> Result<char, CustomError> {
    container
        .chars()
        .find(|char| already_seen_items.contains(char))
        .ok_or(CustomError {
            msg: "No duplicate item was detected.".into(),
        })
}

/// Returns the priority value of the given item
fn get_priority(item: char) -> u32 {
    if item.is_uppercase() {
        (item as u32) - ('A' as u32) + 27
    } else {
        (item as u32) - ('a' as u32) + 1
    }
}
