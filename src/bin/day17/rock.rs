use std::collections::BTreeSet;

use crate::Jet;

pub struct Rock {
    positions: Vec<Position>,
    pub shape: RockShape,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: u64,
    pub y: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RockShape {
    HorizontalLine,
    Plus,
    ReverseL,
    Square,
    VerticalLine,
}

impl Rock {
    pub fn new(highest_rock_y: u64, shape: RockShape) -> Self {
        match shape {
            RockShape::HorizontalLine => Self::new_horizontal_line(highest_rock_y),
            RockShape::Plus => Self::new_plus(highest_rock_y),
            RockShape::ReverseL => Self::new_reverse_l(highest_rock_y),
            RockShape::Square => Self::new_square(highest_rock_y),
            RockShape::VerticalLine => Self::new_vertical_line(highest_rock_y),
        }
    }

    fn new_horizontal_line(highest_rock_y: u64) -> Self {
        let positions = (0..4)
            .map(|x| Position {
                x: 2 + x,
                y: highest_rock_y + 4,
            })
            .collect();

        Self {
            positions,
            shape: RockShape::HorizontalLine,
        }
    }

    fn new_plus(highest_rock_y: u64) -> Self {
        let mut positions: Vec<_> = (0..3)
            .map(|x| Position {
                x: 2 + x,
                y: highest_rock_y + 5,
            })
            .collect();

        positions.push(Position {
            x: 3,
            y: highest_rock_y + 4,
        });

        positions.push(Position {
            x: 3,
            y: highest_rock_y + 6,
        });

        Self {
            positions,
            shape: RockShape::Plus,
        }
    }

    fn new_reverse_l(highest_rock_y: u64) -> Self {
        let mut positions: Vec<_> = (0..3)
            .map(|x| Position {
                x: 2 + x,
                y: highest_rock_y + 4,
            })
            .collect();

        positions.push(Position {
            x: 4,
            y: highest_rock_y + 5,
        });

        positions.push(Position {
            x: 4,
            y: highest_rock_y + 6,
        });

        Self {
            positions,
            shape: RockShape::ReverseL,
        }
    }

    fn new_square(highest_rock_y: u64) -> Self {
        let mut positions: Vec<_> = (0..2)
            .map(|x| Position {
                x: 2 + x,
                y: highest_rock_y + 4,
            })
            .collect();

        positions.extend((0..2).map(|x| Position {
            x: 2 + x,
            y: highest_rock_y + 5,
        }));

        Self {
            positions,
            shape: RockShape::Square,
        }
    }

    fn new_vertical_line(highest_rock_y: u64) -> Self {
        let positions = (0..4)
            .map(|y| Position {
                x: 2,
                y: highest_rock_y + 4 + y,
            })
            .collect();

        Self {
            positions,
            shape: RockShape::VerticalLine,
        }
    }

    pub fn try_fall(&mut self, blocked_positions: &BTreeSet<Position>) -> bool {
        let can_fall = self
            .positions
            .iter()
            .map(Position::below)
            .all(|new_position| new_position.y > 0 && !blocked_positions.contains(&new_position));

        if can_fall {
            for position in &mut self.positions {
                position.y -= 1;
            }
        }

        can_fall
    }

    pub fn try_move_sideways(&mut self, jet: &Jet, blocked_positions: &BTreeSet<Position>) {
        let can_move_sideways = self
            .positions
            .iter()
            .map(|position| match jet {
                Jet::Left => position.left(),
                Jet::Right => position.right(),
            })
            .all(|new_position| {
                (0..7).contains(&new_position.x) && !blocked_positions.contains(&new_position)
            });

        if can_move_sideways {
            for position in &mut self.positions {
                match jet {
                    Jet::Left => position.x -= 1,
                    Jet::Right => position.x += 1,
                }
            }
        }
    }

    pub fn positions(&self) -> &[Position] {
        self.positions.as_ref()
    }
}

impl Position {
    fn below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x.wrapping_sub(1),
            y: self.y,
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.y.cmp(&other.y) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        self.x.cmp(&other.x)
    }
}
