use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/bin/day3/input.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &str) {
    let priority_sum: u32 = input
        .lines()
        .map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);

            let first_compartment_item_types: HashSet<_> = first.chars().collect();

            (second, first_compartment_item_types)
        })
        .flat_map(find_duplicate_item_type)
        .map(get_priority)
        .sum();

    println!("Part 1 answer = {priority_sum}");
}

fn part2(input: &str) {
    let mut rucksacks: Vec<_> = input.lines().collect();

    let priority_sum: u32 = rucksacks
        .chunks_exact_mut(3)
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
        .flat_map(find_duplicate_item_type)
        .map(get_priority)
        .sum();

    println!("Part 2 answer = {priority_sum}");
}

fn find_duplicate_item_type(
    (container, already_seen_items): (&str, HashSet<char>),
) -> Option<char> {
    container
        .chars()
        .find(|char| already_seen_items.contains(char))
}

fn get_priority(item: char) -> u32 {
    if item.is_uppercase() {
        (item as u32) - ('A' as u32) + 27
    } else {
        (item as u32) - ('a' as u32) + 1
    }
}
