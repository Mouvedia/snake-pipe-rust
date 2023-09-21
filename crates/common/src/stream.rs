use serde::{Deserialize, Serialize};
use std::io::{stdin, BufRead, Lines};

// options

#[derive(Debug, Serialize, Deserialize)]
pub struct SizeOption {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitOptions {
    pub frame_duration: u32,
    pub size: SizeOption,
}

// gamestate

#[derive(Debug, Deserialize)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize)]
pub struct Snake {
    direction: Direction,
    head: Position,
    tail: Vec<Position>,
}

#[derive(Debug, Deserialize)]
pub struct GameState {
    snake: Snake,
    fruit: Position,
    score: u32,
    over: bool,
    paused: bool,
}

/**
 * Accepts the iterator from `stdin().lines()`
 * - parses the first line into `option`
 * - returns an iterator of the other lines in `lines` (already parsed)
 */
pub struct Stream {
    pub options: InitOptions,
    pub lines: Box<dyn Iterator<Item = GameState>>, // std::io::Lines<T>, //Lines<T>,
}

impl Stream {
    fn new<T: BufRead + 'static>(
        mut lines: Lines<T>,
    ) -> Result<Stream, Box<dyn std::error::Error>> {
        let first_line = lines.next().unwrap()?;
        let options: InitOptions = serde_json::from_str(&first_line)?;
        // flat_map keeps Some and extracts their values while removing Err
        let parsed_lines = lines.flat_map(|result_line| match result_line {
            Ok(line) => {
                let parsed_line: GameState = serde_json::from_str(&line).unwrap();
                Some(parsed_line)
            }
            Err(_) => None,
        });
        Ok(Self {
            options,
            lines: Box::new(parsed_lines),
        })
    }
}

pub fn parse_gamestate() -> Result<Stream, Box<dyn std::error::Error>> {
    let lines = stdin().lines();
    Stream::new(lines)
}
