use std::collections::{BTreeSet, HashMap};

use crate::{
    jet::Jet,
    rock::{Position, Rock, RockShape},
    ROCK_SHAPES,
};

pub struct Simulation<'a> {
    blocked_positions: BTreeSet<Position>,
    pub highest_y: u64,
    iteration: usize,
    jets: &'a [Jet],
}

impl<'a> Simulation<'a> {
    pub fn new(jets: &'a [Jet]) -> Self {
        Self {
            blocked_positions: BTreeSet::new(),
            highest_y: Default::default(),
            iteration: Default::default(),
            jets,
        }
    }

    pub fn height_after_one_trillion_rocks(&mut self) -> u64 {
        let num_jets = self.jets.len() as u64;
        let mut jets = self.jets.iter().cycle();

        let mut heights: Vec<u64> = Vec::new();
        let mut height_windows: HashMap<Vec<u64>, usize> = HashMap::new();

        let mut rocks_at_rockjetcycle: Vec<usize> = Vec::with_capacity(100_000_000);

        let window_size = 10;

        let (start_index, end_index) = 'outer: loop {
            self.iteration += 1;

            let rock_shape = ROCK_SHAPES[(self.iteration - 1) % ROCK_SHAPES.len()];
            let mut rock = Rock::new(self.highest_y, rock_shape);

            loop {
                // Will only panic due to an empty input.txt, as the `jets` iter uses `cycle`.
                let jet = jets.next().unwrap();

                rock.try_move_sideways(jet, &self.blocked_positions);

                if !rock.try_fall(&self.blocked_positions) {
                    self.blocked_positions.extend(rock.positions());

                    let rock_top = rock
                        .positions()
                        .iter()
                        .fold(0, |highest, position| position.y.max(highest));

                    self.highest_y = self.highest_y.max(rock_top);

                    break;
                }
            }

            if rock_shape == RockShape::Square && self.iteration as u64 % num_jets == 0 {
                println!("Total rocks so far: {}", self.iteration);

                let y_diff = self.highest_y - heights.iter().sum::<u64>();

                heights.push(y_diff);

                rocks_at_rockjetcycle.push(self.iteration);

                if let Some(start_index) = heights.len().checked_sub(window_size) {
                    if let Some(old_index) = height_windows.insert(
                        heights[start_index..(start_index + window_size)].to_vec(),
                        start_index,
                    ) {
                        break 'outer (old_index, start_index);
                    }
                }
            }
        };

        let (pre_cycle_height, cycle_height) = {
            let (pre_cycle_heights, cycle_heights) = heights[..end_index].split_at(start_index);
            let pre_cycle_height: u64 = pre_cycle_heights.iter().sum();
            let cycle_height: u64 = cycle_heights.iter().sum();

            (pre_cycle_height, cycle_height)
        };

        let pre_cycle_rocks = rocks_at_rockjetcycle[start_index - 1];
        let cycle_rocks = rocks_at_rockjetcycle[end_index - 1] - pre_cycle_rocks;

        let total_rocks: u64 = 1_000_000_000_000;

        let remaining_rocks = total_rocks - pre_cycle_rocks as u64;

        let num_cycles = remaining_rocks / cycle_rocks as u64;
        let post_cycle_rocks = remaining_rocks % cycle_rocks as u64;

        self.highest_y = 0;
        self.iteration = 0;
        self.blocked_positions.clear();
        jets = self.jets.iter().cycle();

        println!(
            r#"
        Pre-cycle length (in rocks): {pre_cycle_rocks},
        Cycle length (in rocks): {cycle_rocks},
        Post-cycle length (in rocks): {post_cycle_rocks},

        Total complete cycles: {num_cycles},

        "#
        );

        for _ in 0..(pre_cycle_rocks + cycle_rocks + post_cycle_rocks as usize) {
            self.iteration += 1;

            let rock_shape = ROCK_SHAPES[(self.iteration - 1) % ROCK_SHAPES.len()];
            let mut rock = Rock::new(self.highest_y, rock_shape);

            loop {
                // Will only panic due to an empty input.txt, as the `jets` iter uses `cycle`.
                let jet = jets.next().unwrap();

                rock.try_move_sideways(jet, &self.blocked_positions);

                if !rock.try_fall(&self.blocked_positions) {
                    self.blocked_positions.extend(rock.positions());

                    let rock_top = rock
                        .positions()
                        .iter()
                        .fold(0, |highest, position| position.y.max(highest));

                    self.highest_y = self.highest_y.max(rock_top);

                    break;
                }
            }
        }

        let post_cycle_height = self.highest_y - pre_cycle_height - cycle_height;

        println!(
            r#"
            Pre-cycle height: {pre_cycle_height},
            In cycle height: {},
            Post-cycle height: {post_cycle_height},

        "#,
            cycle_height
        );

        pre_cycle_height + num_cycles * cycle_height + post_cycle_height
    }

    pub fn run_with_limit(&mut self, limit: usize) {
        let mut jets = self.jets.iter().cycle();

        for index in 0..limit {
            self.iteration += 1;

            let rock_shape = ROCK_SHAPES[index % ROCK_SHAPES.len()];
            let mut rock = Rock::new(self.highest_y, rock_shape);

            loop {
                // Will only panic due to an empty input.txt, as the `jets` iter uses `cycle`.
                let jet = jets.next().unwrap();

                rock.try_move_sideways(jet, &self.blocked_positions);

                if !rock.try_fall(&self.blocked_positions) {
                    self.blocked_positions.extend(rock.positions());

                    let rock_top = rock
                        .positions()
                        .iter()
                        .fold(0, |highest, position| position.y.max(highest));

                    self.highest_y = self.highest_y.max(rock_top);

                    break;
                }
            }
        }
    }
}
