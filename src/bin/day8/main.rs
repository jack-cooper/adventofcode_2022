use std::{fs, str::FromStr};

use adventofcode_2022::{AnyResult, CustomError};

struct Grid {
    size: usize,
    trees: Vec<u32>,
}

impl Grid {
    fn trees_above(&self, index: usize) -> impl Iterator<Item = &u32> {
        self.trees
            .iter()
            .skip(index % self.size)
            .step_by(self.size)
            .take(index / self.size)
            .rev()
    }

    fn trees_below(&self, index: usize) -> impl Iterator<Item = &u32> {
        self.trees.iter().skip(index).step_by(self.size).skip(1)
    }

    fn trees_left_of(&self, index: usize) -> impl Iterator<Item = &u32> {
        self.trees
            .iter()
            .skip((index / self.size) * self.size)
            .take(index % self.size)
            .rev()
    }

    fn trees_right_of(&self, index: usize) -> impl Iterator<Item = &u32> {
        self.trees
            .iter()
            .skip(index + 1)
            // The below will never overflow as we use the `is_on_edge_of_grid`
            // function to guard before its use
            .take((self.size - index % self.size) - 1)
    }
}

impl FromStr for Grid {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = s.lines().count();

        let trees = s
            .chars()
            .filter(|&char| char != '\n')
            .map(|char| char.to_digit(10))
            .collect::<Option<Vec<u32>>>()
            .ok_or(CustomError {
                msg: "Failed to parse a digit.".into(),
            })?;

        if size.pow(2) == trees.len() {
            Ok(Self { size, trees })
        } else {
            Err(CustomError {
                msg: "Non-square grid given as input.".into(),
            })
        }
    }
}

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day8/input.txt")?;

    let grid: Grid = input.parse()?;

    part1(&grid)?;
    part2(&grid)?;

    Ok(())
}

fn is_on_edge_of_grid(index: usize, size: usize) -> bool {
    index / size == 0 || index / size == size - 1 || index % size == 0 || index % size == size - 1
}

fn part1(grid: &Grid) -> AnyResult {
    let num_visible_trees: usize = grid
        .trees
        .iter()
        .enumerate()
        .filter(|&(index, height)| {
            if is_on_edge_of_grid(index, grid.size) {
                return true;
            }

            let is_smaller = |other_height| other_height < height;

            grid.trees_above(index).all(is_smaller)
                || grid.trees_below(index).all(is_smaller)
                || grid.trees_left_of(index).all(is_smaller)
                || grid.trees_right_of(index).all(is_smaller)
        })
        .count();

    println!("Part 1 answer = {num_visible_trees}");

    Ok(())
}

fn part2(grid: &Grid) -> AnyResult {
    let highest_scenic_score = grid
        .trees
        .iter()
        .enumerate()
        .filter(|&(index, _)| !is_on_edge_of_grid(index, grid.size))
        .map(|(index, height)| {
            let visible_trees_above = {
                let mut visible_trees_above = grid
                    .trees_above(index)
                    .take_while(|&other_height| other_height < height)
                    .count();

                if index / grid.size - visible_trees_above > 0 {
                    visible_trees_above += 1;
                }

                visible_trees_above
            };

            let visible_trees_below = {
                let mut visible_trees_below = grid
                    .trees_below(index)
                    .take_while(|&other_height| other_height < height)
                    .count();

                if index / grid.size + visible_trees_below < grid.size - 1 {
                    visible_trees_below += 1;
                }

                visible_trees_below
            };

            let visible_trees_left = {
                let mut visible_trees_left = grid
                    .trees_left_of(index)
                    .take_while(|&other_height| other_height < height)
                    .count();

                if index % grid.size - visible_trees_left > 0 {
                    visible_trees_left += 1;
                }

                visible_trees_left
            };

            let visible_trees_right = {
                let mut visible_trees_right = grid
                    .trees_right_of(index)
                    .take_while(|&other_height| other_height < height)
                    .count();

                if index % grid.size + visible_trees_right < grid.size - 1 {
                    visible_trees_right += 1;
                }

                visible_trees_right
            };

            visible_trees_above * visible_trees_below * visible_trees_left * visible_trees_right
        })
        .max()
        .ok_or(CustomError {
            msg: "Tried to find the max scenic score of an empty grid.".into(),
        })?;

    println!("Part 2 answer = {highest_scenic_score}");

    Ok(())
}
