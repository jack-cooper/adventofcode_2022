use std::{collections::HashSet, fs, str::FromStr};

use adventofcode_2022::{AnyResult, CustomError};
use once_cell::sync::Lazy;
use regex::Regex;

const TARGET_ROW: i32 = 2_000_000;

static SENSOR_BEACON_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("Sensor at x=(?P<sensor_x>-?\\d+), y=(?P<sensor_y>-?\\d+): closest beacon is at x=(?P<beacon_x>-?\\d+), y=(?P<beacon_y>-?\\d+)").unwrap()
});

struct BeaconExclusionZone {
    confirmed_beacon_x_positions: HashSet<i32>,
    sensors: Vec<Sensor>,
}

#[derive(Debug)]
struct Sensor {
    beacon_distance: i32,
    position: Position,
}

impl Sensor {
    fn iter_perimeter(&self) -> impl Iterator<Item = Position> + '_ {
        let beacon_distance_range = 0..=(self.beacon_distance + 1);

        let top_to_right = beacon_distance_range.clone().map(|index| Position {
            x: self.position.x + index,
            y: self.position.y - self.beacon_distance - 1 + index,
        });

        let right_to_bottom = beacon_distance_range.clone().map(|index| Position {
            x: self.position.x + self.beacon_distance + 1 - index,
            y: self.position.y - index,
        });

        let bottom_to_left = beacon_distance_range.clone().map(|index| Position {
            x: self.position.x - index,
            y: self.position.y + self.beacon_distance + 1 - index,
        });

        let left_to_top = beacon_distance_range.map(|index| Position {
            x: self.position.x - self.beacon_distance - 1 + index,
            y: self.position.y + index,
        });

        top_to_right
            .chain(right_to_bottom)
            .chain(bottom_to_left)
            .chain(left_to_top)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn manhattan_distance(&self, other: Self) -> i32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i32
    }
}

impl FromStr for BeaconExclusionZone {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut confirmed_beacon_x_positions = HashSet::new();

        let mut sensors = Vec::new();

        for line in s.lines() {
            let captures = SENSOR_BEACON_REGEX.captures(line).ok_or(CustomError {
                msg: "Regex failed to parse a sensor/beacon pair.".into(),
            })?;

            let capture_to_i32 = |name| {
                captures
                    .name(name)
                    .ok_or(CustomError {
                        msg: format!("No {name} detected.").into(),
                    })?
                    .as_str()
                    .parse::<i32>()
                    .map_err(|_| CustomError {
                        msg: format!("Failed to parse {name} as a i32.").into(),
                    })
            };

            let sensor_x = capture_to_i32("sensor_x")?;
            let sensor_y = capture_to_i32("sensor_y")?;
            let beacon_x = capture_to_i32("beacon_x")?;
            let beacon_y = capture_to_i32("beacon_y")?;

            if beacon_y == TARGET_ROW {
                confirmed_beacon_x_positions.insert(beacon_x);
            }

            let sensor_position = Position {
                x: sensor_x,
                y: sensor_y,
            };

            let beacon_position = Position {
                x: beacon_x,
                y: beacon_y,
            };

            sensors.push(Sensor {
                beacon_distance: sensor_position.manhattan_distance(beacon_position),
                position: sensor_position,
            });
        }

        Ok(Self {
            confirmed_beacon_x_positions,
            sensors,
        })
    }
}

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day15/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> AnyResult {
    let beacon_exclusion_zone: BeaconExclusionZone = input.parse()?;

    let mut non_beacon_x_positions: HashSet<i32> = HashSet::new();

    for sensor in beacon_exclusion_zone.sensors {
        let y_diff = sensor.position.y.abs_diff(TARGET_ROW) as i32;

        if sensor.beacon_distance >= y_diff {
            let distance_along_row = sensor.beacon_distance - y_diff;

            for x in
                (sensor.position.x - distance_along_row)..=(sensor.position.x + distance_along_row)
            {
                if !beacon_exclusion_zone
                    .confirmed_beacon_x_positions
                    .contains(&x)
                {
                    non_beacon_x_positions.insert(x);
                }
            }
        }
    }

    println!("Part 1 answer = {}", non_beacon_x_positions.len());

    Ok(())
}

fn part2(input: &str) -> AnyResult {
    let BeaconExclusionZone { sensors, .. }: BeaconExclusionZone = input.parse()?;

    for sensor in &sensors {
        for position in sensor.iter_perimeter() {
            if position.x < 0 || position.x > 4_000_000 || position.y < 0 || position.y > 4_000_000
            {
                continue;
            }

            let out_of_range_of_all_sensors = sensors.iter().all(|other_sensor| {
                other_sensor.position.manhattan_distance(position) > other_sensor.beacon_distance
            });

            if out_of_range_of_all_sensors {
                let x = position.x as u64;
                let y = position.y as u64;

                let tuning_frequency = x * 4_000_000 + y;

                println!("Part 2 answer = {tuning_frequency:?}");

                return Ok(());
            }
        }
    }

    Err(Box::new(CustomError {
        msg: "No position was out of range of all sensors.".into(),
    }))
}
