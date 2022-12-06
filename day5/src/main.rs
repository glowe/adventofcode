#![feature(iter_array_chunks)]
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::result::Result;
use std::str::FromStr;
use std::vec::Vec;

use regex::Regex;

#[derive(Clone, Debug)]
struct Crate(char);

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self(label) = self;
        write!(f, "[{}]", label)
    }
}

#[derive(Debug, Clone)]
struct ParseCrateError(String);

impl Display for ParseCrateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ParseCrateError(input) = self;
        write!(f, "Invalid input {}", input)
    }
}

impl Error for ParseCrateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl FromStr for Crate {
    type Err = ParseCrateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^\[([A-Z])\]$").unwrap();
        if let Some(captures) = re.captures(s) {
            let label = captures.get(1).unwrap().as_str().chars().next().unwrap();
            Ok(Crate(label))
        } else {
            Err(ParseCrateError(s.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
enum MoveError {
    NotEnoughCrates,
    InvalidSourceStack,
    InvalidDestStack,
}

impl Display for MoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughCrates => write!(f, "not enough crates"),
            Self::InvalidSourceStack => write!(f, "invalid source stack"),
            Self::InvalidDestStack => write!(f, "invalid dest stack"),
        }
    }
}

impl Error for MoveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
struct Move {
    amount: u32,
    source: usize,
    dest: usize,
}

impl Move {
    fn execute(&self, stacks: &mut [Vec<Crate>]) -> Result<(), MoveError> {
        let source = self.source - 1;
        let dest = self.dest - 1;
        let len = stacks.len() - 1;

        if source > len {
            return Err(MoveError::InvalidSourceStack);
        }

        if dest > len {
            return Err(MoveError::InvalidDestStack);
        }

        let at = stacks[self.source - 1].len() - self.amount as usize;

        let mut crates = stacks[self.source - 1].split_off(at);
        if crates.len() != self.amount as usize {
            return Err(MoveError::NotEnoughCrates);
        }
        stacks[self.dest - 1].append(&mut crates);
        Ok(())
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.amount, self.source, self.dest
        )
    }
}

#[derive(Debug, Clone)]
struct ParseMoveError(String);

impl Display for ParseMoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ParseMoveError(input) = self;
        write!(f, "Invalid input = {}", input)
    }
}

impl Error for ParseMoveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
        if let Some(captures) = re.captures(s) {
            // Use expects here because this shouldn't fail due to regex ensuring that the captured items are numbers.
            let amount = captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .expect("unable to parse number");
            let source = captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("unable to parse number");
            let dest = captures
                .get(3)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("unable to parse number");
            Ok(Move {
                amount,
                source,
                dest,
            })
        } else {
            Err(ParseMoveError(s.to_string()))
        }
    }
}

fn top_labels(stacks: &[Vec<Crate>]) -> String {
    stacks
        .into_iter()
        .map(|s| {
            let Crate(label) = s[s.len() - 1];
            label
        })
        .collect()
}

fn parse_stacks(stdin: &io::Stdin) -> Result<Vec<Vec<Crate>>, Box<dyn Error>> {
    let mut stack_lines = Vec::new();
    loop {
        let mut line = String::new();
        let n = stdin.read_line(&mut line)?;
        if n == 1 {
            break;
        }
        stack_lines.push(line)
    }
    // Now we build up the stacks
    stack_lines.reverse();
    let mut stack_lines_iter = stack_lines.iter();
    let stack_numbers = stack_lines_iter.next().unwrap();
    let num_stacks = stack_numbers.split_whitespace().count();
    let mut stacks = Vec::new();
    for _ in 0..num_stacks {
        let stack = Vec::new();
        stacks.push(stack);
    }

    for line in stack_lines_iter {
        let line = line.trim_end();
        let mut i = 0;
        let mut j = 0;
        while j < line.len() {
            let chars = line.chars();
            let chunk: String = chars.skip(j).take(4).collect();
            let crate_str = chunk.trim_end();
            if crate_str != "" {
                let crate_ = crate_str.parse::<Crate>()?;
                stacks[i].push(crate_);
            }
            i += 1;
            j += 4;
        }
    }
    Ok(stacks)
}

fn parse_moves(stdin: &io::Stdin, stacks: &mut Vec<Vec<Crate>>) -> Result<(), Box<dyn Error>> {
    // Now we parse the moves
    loop {
        let mut line = String::new();
        let n = stdin.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        let mv = line.trim_end().parse::<Move>()?;
        mv.execute(stacks)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut stacks = parse_stacks(&stdin)?;
    parse_moves(&stdin, &mut stacks)?;
    println!("{}", top_labels(&stacks));
    Ok(())
}
