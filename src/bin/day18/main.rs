use std::{
    collections::{BTreeSet, HashSet},
    fs, iter,
    ops::Bound,
    str::FromStr,
};

use adventofcode_2022::{AnyResult, CustomError};

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn neighbors(&self) -> [Self; 6] {
        let &Self { x, y, z } = self;

        [
            Self { x: x - 1, y, z },
            Self { x: x + 1, y, z },
            Self { x, y: y - 1, z },
            Self { x, y: y + 1, z },
            Self { x, y, z: z - 1 },
            Self { x, y, z: z + 1 },
        ]
    }
}

impl FromStr for Position {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates: Result<Vec<i32>, CustomError> = s
            .split(',')
            .map(|number| {
                number.parse().map_err(|_| CustomError {
                    msg: "Non-numeric co-ordinate detected.".into(),
                })
            })
            .collect();
        let coordinates = coordinates?;

        if coordinates.len() == 3 {
            Ok(Self {
                x: coordinates[0],
                y: coordinates[1],
                z: coordinates[2],
            })
        } else {
            Err(CustomError {
                msg: "Incorrect number of coordinates specified for a position.".into(),
            })
        }
    }
}

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day18/input.txt")?;

    let positions: Result<HashSet<Position>, CustomError> = input.lines().map(str::parse).collect();
    let positions = positions?;

    part1(&positions)?;
    part2(&positions)?;

    Ok(())
}

fn part1(positions: &HashSet<Position>) -> AnyResult {
    let total_surface_area: usize = positions
        .iter()
        .map(|position| {
            position
                .neighbors()
                .into_iter()
                .filter(|neighbor| !positions.contains(neighbor))
                .count()
        })
        .sum();

    println!("Part 1 answer = {total_surface_area}");

    Ok(())
}

fn part2(lava_positions: &HashSet<Position>) -> AnyResult {
    // All air cubes orthogonally adjacent to lava, including those on the inside of the lava.
    let lava_adjacent_air_cubes: BTreeSet<Position> = lava_positions
        .iter()
        .flat_map(|position| {
            position
                .neighbors()
                .into_iter()
                .filter(|neighbor| !lava_positions.contains(neighbor))
        })
        .collect();

    // Every unique pair of lava adjacent air cubes
    let air_cube_pairs = lava_adjacent_air_cubes.iter().flat_map(|&cube| {
        lava_adjacent_air_cubes
            .range((Bound::Excluded(cube), Bound::Unbounded))
            .copied()
            .zip(iter::repeat(cube))
    });

    // Returns true if `other_cube` is a neighbor of `cube`, or if
    // `cube` and `other_cube` both share a neighbor which is
    // also an air cube (not in `lava_positions`).
    let is_in_same_air_pocket = |cube: Position, other_cube: Position| -> bool {
        cube.neighbors().contains(&other_cube)
            || cube.neighbors().iter().any(|neighbor| {
                !lava_positions.contains(neighbor) && other_cube.neighbors().contains(neighbor)
            })
    };

    let mut air_pockets: Vec<HashSet<Position>> = lava_adjacent_air_cubes
        .iter()
        .copied()
        .map(|cube| HashSet::from([cube]))
        .collect();

    for (cube, other_cube) in air_cube_pairs {
        if is_in_same_air_pocket(cube, other_cube) {
            if let Some(other_pocket_index) = air_pockets.iter().position(|air_pocket| {
                !air_pocket.contains(&cube) && air_pocket.contains(&other_cube)
            }) {
                let air_pocket = air_pockets.swap_remove(other_pocket_index);

                air_pockets
                    .iter_mut()
                    .find(|air_pocket| air_pocket.contains(&cube))
                    .expect("Somehow managed to remove an air cube from all air pockets.")
                    .extend(air_pocket);
            }
        }
    }

    let outside_air_pocket = air_pockets
        .iter()
        .reduce(|minimum_air_pocket, next_air_pocket| {
            let minimum_air_cube = minimum_air_pocket.iter().min().unwrap();
            let minimum_air_cube2 = next_air_pocket.iter().min().unwrap();

            if minimum_air_cube < minimum_air_cube2 {
                minimum_air_pocket
            } else {
                next_air_pocket
            }
        })
        .unwrap();

    let total_surface_area: usize = lava_positions
        .iter()
        .map(|position| {
            position
                .neighbors()
                .into_iter()
                .filter(|neighbor| outside_air_pocket.contains(neighbor))
                .count()
        })
        .sum();

    println!("Part 2 answer = {total_surface_area}");

    Ok(())
}
