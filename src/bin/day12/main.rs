use std::{collections::VecDeque, fs, str::FromStr};

use adventofcode_2022::{AnyResult, CustomError};

struct Heightmap {
    end_index: usize,
    grid: Grid,
    start_index: usize,
}

struct Grid {
    elevations: Vec<u32>,
    height: usize,
    width: usize,
}

impl Heightmap {
    fn bfs(&self) -> Vec<Option<usize>> {
        let mut came_from: Vec<Option<usize>> = vec![None; self.grid.elevations.len()];

        let mut frontier = VecDeque::new();

        frontier.push_back(self.start_index);

        while let Some(index) = frontier.pop_front() {
            if index == self.end_index {
                break;
            }

            frontier.extend(
                self.grid
                    .neighbors(index)
                    .into_iter()
                    .filter(|&neighbor_index| {
                        if came_from[neighbor_index].is_none() && neighbor_index != self.start_index
                        {
                            came_from[neighbor_index] = Some(index);

                            true
                        } else {
                            false
                        }
                    }),
            );
        }

        came_from
    }
}

impl FromStr for Heightmap {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut start_index, mut end_index) = (None, None);

        let elevations = s
            .replace('\n', "")
            .chars()
            .enumerate()
            .map(|(index, height)| match height {
                'S' => {
                    start_index = Some(index);
                    'a' as u32
                }
                'E' => {
                    end_index = Some(index);
                    'z' as u32
                }
                other => other as u32,
            })
            .collect();

        let width = s
            .lines()
            .next()
            .ok_or(CustomError {
                msg: "Received an empty input.".into(),
            })?
            .len();

        let height = s.lines().count();

        let grid = Grid {
            elevations,
            height,
            width,
        };

        Ok(Heightmap {
            end_index: end_index.ok_or(CustomError {
                msg: "No end position was provided.".into(),
            })?,
            grid,
            start_index: start_index.ok_or(CustomError {
                msg: "No start position was provided.".into(),
            })?,
        })
    }
}

impl Grid {
    fn adjacent_indices(&self, index: usize) -> Vec<usize> {
        let mut adjacent_indices = Vec::new();

        if index % self.width != 0 {
            adjacent_indices.push(index - 1);
        }

        if index % self.width != self.width - 1 {
            adjacent_indices.push(index + 1);
        }

        if index / self.width != 0 {
            adjacent_indices.push(index - self.width);
        }

        if index / self.width != self.height - 1 {
            adjacent_indices.push(index + self.width);
        }

        adjacent_indices
    }

    fn neighbors(&self, index: usize) -> Vec<usize> {
        let mut neighbors = self.adjacent_indices(index);

        neighbors.retain(|&elevation_index| {
            self.elevations[elevation_index] <= self.elevations[index] + 1
        });

        neighbors
    }
}

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day12/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> AnyResult {
    let heightmap: Heightmap = input.parse()?;

    let elevations = heightmap.bfs();

    let mut current_index = Some(heightmap.end_index);
    let mut steps = 0;

    while let Some(index) = current_index {
        current_index = elevations[index];

        if current_index.is_some() {
            steps += 1;
        }
    }

    println!("Part 1 answer = {steps}");

    Ok(())
}

fn part2(_input: &str) -> AnyResult {
    Ok(())
}
