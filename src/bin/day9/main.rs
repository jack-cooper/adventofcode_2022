use std::{collections::HashSet, fs, str::FromStr};

use adventofcode_2022::{AnyResult, CustomError};

enum Motion {
    X(i32),
    Y(i32),
}

#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

struct Movement {
    x: i32,
    y: i32,
}

impl Movement {
    pub fn new(x: i32, y: i32) -> Self {
        assert!(x.abs() <= 1 && y.abs() <= 1);

        Self { x, y }
    }
}

impl Position {
    const ZERO: Self = Self { x: 0, y: 0 };

    fn required_catchup_movement(&self, tail_position: &Position) -> Option<Movement> {
        let (x_diff, y_diff) = (self.x - tail_position.x, self.y - tail_position.y);

        (x_diff.abs() > 1 || y_diff.abs() > 1)
            .then_some(Movement::new(x_diff.signum(), y_diff.signum()))
    }
}

impl FromStr for Motion {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, magnitude) = s.split_once(' ').ok_or(CustomError {
            msg: "Received a malformed motion instruction.".into(),
        })?;

        let magnitude: i32 = magnitude.parse().map_err(|_| CustomError {
            msg: "Received a non-numeric magnitude.".into(),
        })?;

        match direction {
            "D" => Ok(Self::Y(-magnitude)),
            "L" => Ok(Self::X(-magnitude)),
            "R" => Ok(Self::X(magnitude)),
            "U" => Ok(Self::Y(magnitude)),
            _ => Err(CustomError {
                msg: "Received a malformed direction.".into(),
            }),
        }
    }
}

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day9/input.txt")?;

    let motions: Vec<Motion> = input
        .lines()
        .map(str::parse::<Motion>)
        .collect::<Result<Vec<_>, _>>()?;

    part1(&motions)?;
    part2(&motions)?;

    Ok(())
}

fn part1(motions: &[Motion]) -> AnyResult {
    let mut visited_positions: HashSet<Position> = HashSet::new();

    let (mut head_position, mut tail_position) = (Position::ZERO, Position::ZERO);

    for motion in motions {
        let magnitude = match motion {
            Motion::X(magnitude) => magnitude,
            Motion::Y(magnitude) => magnitude,
        };
        let direction = magnitude.signum();

        for _ in 0..magnitude.abs() {
            match motion {
                Motion::X(_) => head_position.x += direction,
                Motion::Y(_) => head_position.y += direction,
            }

            if let Some(movement) = head_position.required_catchup_movement(&tail_position) {
                tail_position.x += movement.x;
                tail_position.y += movement.y;
            }

            visited_positions.insert(tail_position);
        }
    }

    let num_visited_positions = visited_positions.len();

    println!("Part 1 answer = {num_visited_positions}");

    Ok(())
}

fn part2(motions: &[Motion]) -> AnyResult {
    let mut tail_visited_positions: HashSet<Position> = HashSet::new();

    let mut positions = [Position::ZERO; 10];

    for motion in motions {
        let magnitude = match motion {
            Motion::X(magnitude) => magnitude,
            Motion::Y(magnitude) => magnitude,
        };
        let direction = magnitude.signum();

        for _ in 0..magnitude.abs() {
            match motion {
                Motion::X(_) => positions[0].x += direction,
                Motion::Y(_) => positions[0].y += direction,
            }

            for index in 1..positions.len() {
                let (start, end) = positions.split_at_mut(index);

                let knot_front = start.last().unwrap();
                let knot_back = end.first_mut().unwrap();

                if let Some(movement) = knot_front.required_catchup_movement(knot_back) {
                    knot_back.x += movement.x;
                    knot_back.y += movement.y;
                } else {
                    break;
                }
            }

            tail_visited_positions.insert(positions[9]);
        }
    }

    let num_visited_positions = tail_visited_positions.len();

    println!("Part 2 answer = {num_visited_positions}");

    Ok(())
}
