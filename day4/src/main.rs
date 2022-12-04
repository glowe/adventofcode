use std::error;
use std::fmt;
use std::io;
use std::num;
use std::result;
use std::str;

struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn new(start: u32, end: u32) -> Self {
        Assignment { start, end }
    }

    fn contains(&self, other: &Assignment) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.contains(other)
            || (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
            || other.contains(&self)
    }
}

#[derive(Clone, Debug)]
struct ParseAssignmentError {
    input: String,
    source: Option<num::ParseIntError>,
}

impl fmt::Display for ParseAssignmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.input)
    }
}

impl error::Error for ParseAssignmentError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.source {
            Some(source) => Some(source),
            None => None,
        }
    }
}

impl str::FromStr for Assignment {
    type Err = ParseAssignmentError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        if let Some((start_str, end_str)) = s.split_once('-') {
            let start = match start_str.parse::<u32>() {
                Err(int_error) => {
                    return Err(ParseAssignmentError {
                        input: s.to_string(),
                        source: Some(int_error),
                    })
                }
                Ok(val) => val,
            };
            let end = match end_str.parse::<u32>() {
                Err(int_error) => {
                    return Err(ParseAssignmentError {
                        input: s.to_string(),
                        source: Some(int_error),
                    })
                }
                Ok(val) => val,
            };
            Ok(Self::new(start, end))
        } else {
            return Err(ParseAssignmentError {
                input: s.to_string(),
                source: None,
            });
        }
    }
}

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let stdin = io::stdin();
    let mut count = 0;
    loop {
        let mut line = String::new();
        let n = stdin.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        let line = line.trim_end();
        let (first_str, second_str) = line.split_once(',').unwrap();
        let first = first_str.parse::<Assignment>()?;
        let second = second_str.parse::<Assignment>()?;
        if first.overlaps(&second) {
            count += 1;
        }
    }
    println!("{}", count);
    Ok(())
}
