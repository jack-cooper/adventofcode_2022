use std::{error::Error, str::FromStr};

use adventofcode_2022::CustomError;

#[derive(Eq, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn base_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn loses_against(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

impl FromStr for HandShape {
    type Err = CustomError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            other => Err(CustomError::new(&format!(
                "{other} does not correspond to a hand shape!"
            ))),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("src/bin/day2/input.txt")?;

    part1(&input);
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) {
    struct Game {
        player1: HandShape,
        player2: HandShape,
    }

    impl Game {
        fn score(&self) -> u32 {
            if self.player2.wins_against() == self.player1 {
                6
            } else if self.player1 == self.player2 {
                3
            } else {
                0
            }
        }
    }

    let total_score: u32 = input
        .lines()
        .flat_map(|game| {
            let mut hand_shapes = game.split(' ').flat_map(str::parse::<HandShape>);

            Ok::<_, CustomError>(Game {
                player1: hand_shapes.next().ok_or_else(|| {
                    CustomError::new("Tried to construct a game, but was missing the 1st player.")
                })?,
                player2: hand_shapes.next().ok_or_else(|| {
                    CustomError::new("Tried to construct a game, but was missing the 2nd player.")
                })?,
            })
        })
        .map(|game| game.player2.base_score() + game.score())
        .sum();

    println!("Part 1 answer = {total_score}");
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let total_score: u32 = input
        .lines()
        .flat_map(|game| {
            let mut game_components = game.split(' ');

            Ok::<_, CustomError>([
                game_components.next().ok_or_else(|| {
                    CustomError::new(
                        "Tried to construct a game, but was missing the opponent's choice",
                    )
                })?,
                game_components.next().ok_or_else(|| {
                    CustomError::new("Tried to construct a game, but was missing the result.")
                })?,
            ])
        })
        .flat_map(|[opponent_choice, result]| {
            enum GameResult {
                Win,
                Draw,
                Loss,
            }

            let game_result = match result {
                "X" => Ok(GameResult::Loss),
                "Y" => Ok(GameResult::Draw),
                "Z" => Ok(GameResult::Win),
                other => Err(CustomError::new(format!(
                    "{other} is not a valid game result!"
                ))),
            }?;

            let opponent_shape = HandShape::from_str(opponent_choice)?;

            let (base_score, game_score) = match game_result {
                GameResult::Win => (opponent_shape.loses_against().base_score(), 6),
                GameResult::Draw => (opponent_shape.base_score(), 3),
                GameResult::Loss => (opponent_shape.wins_against().base_score(), 0),
            };

            Ok::<_, CustomError>(base_score + game_score)
        })
        .sum();

    println!("Part 2 answer = {total_score}");

    Ok(())
}
