use std::io;
use std::result;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

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

#[derive(Debug, Clone)]
struct ParseOutcomeError {
    invalid: String,
}

impl std::fmt::Display for ParseOutcomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseOutcomeError {}", self.invalid)
    }
}

impl std::error::Error for ParseOutcomeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl FromStr for Outcome {
    type Err = ParseOutcomeError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            invalid => Err(ParseOutcomeError {
                invalid: invalid.to_string(),
            }),
        }
    }
}

struct Round {
    opponent: Shape,
    you: Shape,
}

impl Round {
    fn new(opponent: Shape, you: Shape) -> Self {
        Round { opponent, you }
    }

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

fn solve(opponent: Shape, outcome: Outcome) -> Shape {
    match outcome {
        Outcome::Draw => opponent,
        Outcome::Loss => match opponent {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        Outcome::Win => match opponent {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
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
        if let Some((opponent_str, outcome_str)) = buf.trim_end().split_once(' ') {
            let opponent = opponent_str.parse::<Shape>()?;
            let outcome = outcome_str.parse::<Outcome>()?;
            let you = solve(opponent, outcome);
            let round = Round::new(opponent, you);
            total_score += round.score() as u32;
        } else {
            panic!("Couldn't parse {}", buf);
        }
    }
    println!("{}", total_score);
    Ok(())
}
