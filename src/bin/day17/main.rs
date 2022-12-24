mod jet;
mod rock;
mod simulation;

use std::fs;

use adventofcode_2022::{AnyResult, CustomError};
use jet::Jet;
use rock::RockShape;

use crate::simulation::Simulation;

const ROCK_SHAPES: [RockShape; 5] = [
    RockShape::HorizontalLine,
    RockShape::Plus,
    RockShape::ReverseL,
    RockShape::VerticalLine,
    RockShape::Square,
];

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day17/input.txt")?;

    let jets: Result<Vec<Jet>, CustomError> = input.trim().chars().map(Jet::try_from).collect();
    let jets = jets?;

    part1(&jets)?;
    part2(&jets)?;

    Ok(())
}

fn part1(jets: &[Jet]) -> AnyResult {
    let mut simulation = Simulation::new(jets);

    simulation.run_with_limit(2022);

    println!("Part 1 answer = {}", simulation.highest_y);

    Ok(())
}

fn part2(jets: &[Jet]) -> AnyResult {
    let mut simulation = Simulation::new(jets);

    let total_height = simulation.height_after_one_trillion_rocks();

    println!("Part 2 answer = {total_height}");

    Ok(())
}
