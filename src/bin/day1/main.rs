use std::{fs, num::ParseIntError};

use adventofcode_2022::AnyResult;

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day1/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

/// Returns the total calories an elf is holding, or an error
/// if he was holding a non-numerical (or negative) calorie snack.
fn elf_to_calories(elf: &str) -> Result<u32, ParseIntError> {
    elf.lines().map(str::parse::<u32>).sum()
}

fn part1(input: &str) -> AnyResult {
    let elves = input.split("\n\n");

    let max_calories = elves
        .map(elf_to_calories)
        .try_fold(0, |max_calories, calories| {
            calories.map(|calories| calories.max(max_calories))
        })?;

    println!("Part 1 answer = {max_calories}");

    Ok(())
}

fn part2(input: &str) -> AnyResult {
    let elves = input.split("\n\n");

    let maxima = elves
        .map(elf_to_calories)
        // Iterate through all elves, storing the 3 highest values we've seen so far
        .try_fold([0, 0, 0], |mut maxima, calories| {
            calories.map(|calories| {
                if calories > maxima[2] {
                    maxima[0] = maxima[1];
                    maxima[1] = maxima[2];
                    maxima[2] = calories;
                } else if calories > maxima[1] {
                    maxima[0] = maxima[1];
                    maxima[1] = calories;
                } else if calories > maxima[0] {
                    maxima[0] = calories;
                }

                maxima
            })
        })?;

    let top_three_calorie_sum: u32 = maxima.into_iter().sum();

    println!("Part 2 answer = {top_three_calorie_sum}");

    Ok(())
}
