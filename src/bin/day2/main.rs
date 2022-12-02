use std::io;

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

impl TryFrom<&str> for HandShape {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            other => Err(format!("{other} does not correspond to a hand shape!")),
        }
    }
}

struct Game {
    player1: HandShape,
    player2: HandShape,
}

impl FromIterator<HandShape> for Game {
    fn from_iter<T: IntoIterator<Item = HandShape>>(iter: T) -> Self {
        let mut iter = iter.into_iter();

        Self {
            player1: iter
                .next()
                .expect("A game requires more than 0 hand shapes!"),
            player2: iter
                .next()
                .expect("A game requires more than 1 hand shape!"),
        }
    }
}

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("src/bin/day2/input.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &str) {
    let total_score: u32 = input
        .lines()
        .map(|game| {
            game.split(' ')
                .flat_map(HandShape::try_from)
                .collect::<Game>()
        })
        .map(|game| {
            let base_score = game.player2.base_score();

            let game_score = if game.player2.wins_against() == game.player1 {
                6
            } else if game.player1.wins_against() == game.player2 {
                0
            } else {
                3
            };

            base_score + game_score
        })
        .sum();

    println!("Part 1 answer = {total_score}");
}

fn part2(input: &str) {
    let total_score: u32 = input
        .lines()
        .map(|game| {
            let mut hand_shapes = game.split(' ');

            [hand_shapes.next().unwrap(), hand_shapes.next().unwrap()]
        })
        .map(|[opponent_choice, result]| {
            enum GameResult {
                Win,
                Draw,
                Loss,
            }

            let game_result = match result {
                "X" => GameResult::Loss,
                "Y" => GameResult::Draw,
                "Z" => GameResult::Win,
                other => panic!("{other} is not a valid game result!"),
            };

            let opponent_shape =
                HandShape::try_from(opponent_choice).expect("Invalid opponent choice!");

            let (base_score, game_score) = match game_result {
                GameResult::Win => (opponent_shape.loses_against().base_score(), 6),
                GameResult::Draw => (opponent_shape.base_score(), 3),
                GameResult::Loss => (opponent_shape.wins_against().base_score(), 0),
            };

            base_score + game_score
        })
        .sum();

    println!("Part 2 answer = {total_score}");
}
