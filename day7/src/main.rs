use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt::Display;
use std::io::{self, Read};

#[derive(Debug)]
struct Directory {
    name: String,
    files: HashMap<String, u64>,
    directories: HashMap<String, Directory>,
}

impl Directory {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    fn size(&self) -> u64 {
        self.files.iter().map(|(_name, size)| size).sum::<u64>()
            + self
                .directories
                .iter()
                .map(|(_name, directory)| directory.size())
                .sum::<u64>()
    }

    fn add_file(&mut self, name: impl Into<String>, size: u64) {
        self.files.insert(name.into(), size);
    }

    fn add_directory(&mut self, directory: Directory) {
        let name = directory.name.clone();
        self.directories.insert(name, directory);
    }

    fn get_mut_directory(&mut self, name: impl Into<String>) -> Option<&mut Directory> {
        let name = name.into();
        self.directories.get_mut(&name)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    ChangeDirectory(String),
    ChangeToParentDirectory,
    ListDirectory,
    Directory(String),
    File(String, u64),
    UnexpectedCharacters(String),
}

struct Lexer {
    lines: VecDeque<String>,
}

impl Lexer {
    fn next_non_empty_line(&mut self) -> Option<String> {
        let mut front_line = self.lines.pop_front();

        // Skip empty lines
        loop {
            match front_line {
                Some(line) if line.is_empty() => front_line = self.lines.pop_front(),
                Some(line) => {
                    return Some(line);
                }
                None => {
                    return None;
                }
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.next_non_empty_line() {
            if line == "$ ls" {
                Some(Token::ListDirectory)
            } else if line == "$ cd .." {
                Some(Token::ChangeToParentDirectory)
            } else if line.starts_with("$ cd ") {
                let name = line.strip_prefix("$ cd ")?;
                Some(Token::ChangeDirectory(name.to_string()))
            } else if line.starts_with("dir ") {
                let name = line.strip_prefix("dir ")?;
                Some(Token::Directory(name.to_string()))
            } else if let Some((size_str, name)) = line.split_once(' ') {
                let size = size_str.parse::<u64>().unwrap();
                Some(Token::File(name.to_string(), size))
            } else {
                Some(Token::UnexpectedCharacters(line))
            }
        } else {
            None
        }
    }
}

impl Lexer {
    fn new(input: impl Into<String>) -> Self {
        let lines = input
            .into()
            .split('\n')
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        Self { lines }
    }
}

struct Parser {
    lexer: Lexer,
    previous: Option<Token>,
    current: Option<Token>,
}

#[derive(Debug)]
struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            previous: None,
            current: None,
        }
    }

    fn advance(&mut self) -> std::result::Result<(), ParseError> {
        self.previous = self.current.clone();
        self.current = self.lexer.next();
        if let Some(Token::UnexpectedCharacters(_)) = self.current {
            Err(ParseError("error at ".to_string()))
        } else {
            Ok(())
        }
    }

    fn consume(&mut self, token: Token) -> Result<(), ParseError> {
        if self.current.is_none() {
            return Err(ParseError("current token is none".to_string()));
        }
        if self.current == Some(token.clone()) {
            self.advance()?;
        } else {
            return Err(ParseError(format!(
                "Expected {:?}, but was {:?}",
                token, self.current
            )));
        }
        Ok(())
    }

    fn parse(&mut self) -> Result<Directory, ParseError> {
        self.advance()?;
        self.consume(Token::ChangeDirectory("/".to_string()))?;
        self.parse_directory("/".to_string())
    }

    fn parse_directory(&mut self, name: String) -> Result<Directory, ParseError> {
        self.consume(Token::ListDirectory)?;
        let mut directory = Directory::new(name);
        loop {
            match self.current.clone() {
                Some(Token::Directory(subdir_name)) => {
                    self.advance()?;
                    directory.add_directory(Directory::new(subdir_name));
                }
                Some(Token::File(ref fname, size)) => {
                    self.advance()?;
                    directory.add_file(fname, size);
                }
                Some(Token::ChangeDirectory(subdir_name)) => {
                    self.advance()?;
                    let subdir = directory.get_mut_directory(subdir_name.clone()).unwrap();
                    *subdir = self.parse_directory(subdir_name)?;
                }
                Some(Token::ChangeToParentDirectory) => {
                    self.advance()?;
                    break;
                }
                None => {
                    break;
                }
                token => {
                    return Err(ParseError(format!("Unexpected token! {:?}", token)));
                }
            }
        }
        Ok(directory)
    }
}

fn find_space(directory: &Directory, how_much: u64, candidates: &mut Vec<u64>) {
    if directory.size() >= how_much {
        candidates.push(directory.size());
    }
    for subdirectory in directory.directories.values() {
        find_space(subdirectory, how_much, candidates);
    }
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input)?;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let root = parser.parse()?;
    let unused = 70_000_000 - root.size();
    let atleast = 30_000_000 - unused;
    let mut candidates = Vec::new();
    find_space(&root, atleast, &mut candidates);
    candidates.sort();
    println!("{}", candidates.first().unwrap());
    Ok(())
}
