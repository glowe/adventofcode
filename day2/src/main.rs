use std::io;
use std::result;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

// Implement an Err for FromStr Shape

#[derive(Debug, Clone)]
struct ParseShapeError {
    invalid: String,
}

impl std::fmt::Display for ParseShapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseShapeError {}", self.invalid)
    }
}

impl std::error::Error for ParseShapeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl FromStr for Shape {
    type Err = ParseShapeError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            invalid => Err(ParseShapeError {
                invalid: invalid.to_owned(),
            }),
        }
    }
}

enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

struct Round {
    opponent: Shape,
    you: Shape,
}

impl Round {
    fn outcome(&self) -> Outcome {
        match (&self.opponent, &self.you) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Win,
            (Shape::Rock, Shape::Scissors) => Outcome::Loss,
            (Shape::Paper, Shape::Rock) => Outcome::Loss,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Win,
            (Shape::Scissors, Shape::Rock) => Outcome::Win,
            (Shape::Scissors, Shape::Paper) => Outcome::Loss,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }

    ///
    ///  The score for a single round is the score for the shape you selected
    /// (1 for Rock, 2 for Paper, and 3 for Scissors)
    /// plus the score for the outcome of the round
    /// (0 if you lost, 3 if the round was a draw, and 6 if you won).
    //
    fn score(&self) -> u8 {
        self.you as u8 + self.outcome() as u8
    }
}

#[derive(Debug, Clone)]
struct ParseRoundError {
    source: ParseShapeError,
}

impl std::fmt::Display for ParseRoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseRoundError")
    }
}

impl std::error::Error for ParseRoundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let (opponent_str, you_str) = match s.split_once(' ') {
            Some(parts) => parts,
            None => {
                return Err(ParseRoundError {
                    source: ParseShapeError {
                        invalid: s.to_string(),
                    },
                });
            }
        };
        let opponent = opponent_str
            .parse::<Shape>()
            .map_err(|source| ParseRoundError { source })?;
        let you = you_str
            .parse::<Shape>()
            .map_err(|source| ParseRoundError { source })?;
        Ok(Round { opponent, you })
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut total_score: u32 = 0;
    loop {
        let mut buf = String::new();
        let n = stdin.read_line(&mut buf)?;
        if n == 0 {
            break;
        }
        let round = buf.trim_end().parse::<Round>()?;
        total_score += round.score() as u32;
    }
    println!("{}", total_score);
    Ok(())
}
