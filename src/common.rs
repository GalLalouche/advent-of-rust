use std::num::ParseIntError;

#[macro_export]
macro_rules! read_input {
    () => {{
        let current_path = std::path::Path::new(file!());
        let day_number = current_path.file_name().unwrap().to_str().unwrap()[3..]
            .split('.')
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        std::fs::read_to_string(format!(
            "{}/input{}.txt",
            current_path.parent().unwrap().to_str().unwrap(),
            day_number,
        ))
        .unwrap()
        .trim()
        .to_owned()
    }};
}

#[macro_export]
macro_rules! read_input_lines {
    () => {{
        $crate::read_input!().lines().map(|s| s.trim())
    }};
}

#[macro_export]
macro_rules! parse_input_lines {
    ($l:tt) => {{
        $crate::read_input_lines!().map(|s| $l::from_str(s).unwrap())
    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

impl From<Orientation> for Direction {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::North => Direction::Up,
            Orientation::South => Direction::Down,
            Orientation::West => Direction::Left,
            Orientation::East => Direction::Right,
        }
    }
}
impl From<Direction> for Orientation {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Orientation::North,
            Direction::Down => Orientation::South,
            Direction::Left => Orientation::West,
            Direction::Right => Orientation::East,
        }
    }
}

pub trait Move {
    fn mv(&self, d: Direction) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError(String);
impl ParseError {
    pub fn new(input: &str, err: &str) -> Self {
        Self(format!("'{}': {}", input, err.to_owned()))
    }
}
impl From<ParseIntError> for ParseError {
    fn from(pi: ParseIntError) -> Self {
        ParseError(format!("{}", pi))
    }
}
