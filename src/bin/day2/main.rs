use std::str::FromStr;

use adventofcode_2022::{flatten_result, AnyResult, CustomError};

/// One of the three choices in the game Rock, Paper, Scissors
#[derive(Eq, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    /// Returns the score you are guaranteed to receive in the Elf tournament, just from
    /// selecting this hand shape.
    fn base_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    /// Returns the hand shape which this hand shape loses against when selected.
    fn loses_against(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    /// Returns the hand shape which this hand shape wins against when selected.
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
            other => Err(CustomError {
                msg: format!("{other} does not correspond to a hand shape!").into(),
            }),
        }
    }
}

fn main() -> AnyResult {
    let input = std::fs::read_to_string("src/bin/day2/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> AnyResult {
    /// The `Game` struct represents a single Rock Paper Scissors showdown in the tournament.
    struct Game {
        player1: HandShape,
        player2: HandShape,
    }

    impl Game {
        /// Returns the player score for this game. As we are assumed to be
        /// player 2 in each game, this is from player 2's perspective.
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

    let total_score = input
        .lines()
        // Split the input into each player's decision
        .map(|game| {
            game.split_once(' ').ok_or(CustomError {
                msg: "Encountered a game descriptor which was missing a space character.".into(),
            })
        })
        // Parse each side of the space into a `HandShape`
        .map(|game| {
            game.map(|(player1, player2)| {
                Ok(Game {
                    player1: player1.parse()?,
                    player2: player2.parse()?,
                })
            })
        })
        .map(flatten_result)
        // Add the base score and game score
        .map(|game| game.map(|game| game.player2.base_score() + game.score()))
        // Sum the score of each game
        .sum::<Result<u32, _>>()?;

    println!("Part 1 answer = {total_score}");

    Ok(())
}

fn part2(input: &str) -> AnyResult {
    enum GameResult {
        Win,
        Draw,
        Loss,
    }

    let total_score = input
        .lines()
        // Split the input into the opponent's decision and desired result
        .map(|game| {
            game.split_once(' ').ok_or(CustomError {
                msg: "Encountered a game descriptor which was missing a space character.".into(),
            })
        })
        // Add the base score and the game score
        .map(|game| {
            game.map(|(opponent_shape, result)| {
                let game_result = match result {
                    "X" => Ok(GameResult::Loss),
                    "Y" => Ok(GameResult::Draw),
                    "Z" => Ok(GameResult::Win),
                    other => Err(CustomError {
                        msg: format!("{other} is not a valid game result!").into(),
                    }),
                }?;

                let opponent_shape = HandShape::from_str(opponent_shape)?;

                let (base_score, game_score) = match game_result {
                    GameResult::Win => (opponent_shape.loses_against().base_score(), 6),
                    GameResult::Draw => (opponent_shape.base_score(), 3),
                    GameResult::Loss => (opponent_shape.wins_against().base_score(), 0),
                };

                Ok(base_score + game_score)
            })
        })
        // Flatten the 2 layers of errors above back into a single `Result`
        .map(flatten_result)
        // Sum the score of each game
        .sum::<Result<u32, _>>()?;

    println!("Part 2 answer = {total_score}");

    Ok(())
}
