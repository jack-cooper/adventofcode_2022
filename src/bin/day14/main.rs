use std::{collections::HashSet, fs, str::FromStr};

use adventofcode_2022::{AnyResult, CustomError};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn position_below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn position_below_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn position_below_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

impl FromStr for Position {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(CustomError {
            msg: "Malformed path point detected.".into(),
        })?;

        Ok(Self {
            x: x.parse().map_err(|_| CustomError {
                msg: "Non-numeric x position detected.".into(),
            })?,
            y: y.parse().map_err(|_| CustomError {
                msg: "Non-numeric y position detected.".into(),
            })?,
        })
    }
}

fn rock_positions(input: &str) -> Result<(HashSet<Position>, u32), CustomError> {
    let mut rock_positions: HashSet<Position> = HashSet::new();

    let mut y_max = u32::MIN;

    for line in input.lines() {
        let rock_vertices: Result<Vec<Position>, CustomError> =
            line.split(" -> ").map(str::parse::<Position>).collect();

        let rock_vertices = rock_vertices?;

        let mut vertex_pairs = rock_vertices.windows(2);

        while let Some([vertex, vertex2]) = vertex_pairs.next() {
            y_max = y_max.max(vertex.y).max(vertex2.y);

            if vertex.x != vertex2.x {
                for x in (vertex.x.min(vertex2.x))..=(vertex.x.max(vertex2.x)) {
                    rock_positions.insert(Position { x, y: vertex.y });
                }
            } else if vertex.y != vertex2.y {
                for y in (vertex.y.min(vertex2.y))..=(vertex.y.max(vertex2.y)) {
                    rock_positions.insert(Position { x: vertex.x, y });
                }
            } else {
                return Err(CustomError {
                    msg: "2 consecutive vertex pairs should not be identical.".into(),
                });
            }
        }
    }

    Ok((rock_positions, y_max))
}

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day14/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> AnyResult {
    let (mut blocked_positions, y_max) = rock_positions(input)?;

    let spawn_position = Position { x: 500, y: 0 };

    let mut sand_index = 0;

    loop {
        let mut sand_position = spawn_position;

        loop {
            if sand_position.y >= y_max {
                println!("Part 1 answer = {sand_index}");
                return Ok(());
            }
            if !blocked_positions.contains(&sand_position.position_below()) {
                sand_position = sand_position.position_below();
            } else if !blocked_positions.contains(&sand_position.position_below_left()) {
                sand_position = sand_position.position_below_left();
            } else if !blocked_positions.contains(&sand_position.position_below_right()) {
                sand_position = sand_position.position_below_right();
            } else {
                blocked_positions.insert(sand_position);
                break;
            }
        }

        sand_index += 1;
    }
}

fn part2(input: &str) -> AnyResult {
    let (mut blocked_positions, y_max) = rock_positions(input)?;

    let spawn_position = Position { x: 500, y: 0 };

    let mut sand_index = 1;

    loop {
        let mut sand_position = spawn_position;

        loop {
            if sand_position.position_below().y == y_max + 2 {
                blocked_positions.insert(sand_position);
                break;
            }
            if !blocked_positions.contains(&sand_position.position_below()) {
                sand_position = sand_position.position_below();
            } else if !blocked_positions.contains(&sand_position.position_below_left()) {
                sand_position = sand_position.position_below_left();
            } else if !blocked_positions.contains(&sand_position.position_below_right()) {
                sand_position = sand_position.position_below_right();
            } else {
                if sand_position == spawn_position {
                    println!("Part 2 answer = {sand_index}");
                    return Ok(());
                }
                blocked_positions.insert(sand_position);
                break;
            }
        }

        sand_index += 1;
    }
}
