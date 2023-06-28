use crate::DayResult;
use std::{fs, io};

pub struct Day2Result {
    total_score: i32,
}

impl DayResult for Day2Result {
    fn show(self) {
        println!("Day 2: part 1: totalScore: {}", self.total_score);
    }
}

enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

enum MatchResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl HandShape {
    fn compare(&self, other: &HandShape) -> MatchResult {
        match self {
            HandShape::Rock => match other {
                HandShape::Rock => MatchResult::Draw,
                HandShape::Paper => MatchResult::Lose,
                HandShape::Scissor => MatchResult::Win,
            },
            HandShape::Paper => match other {
                HandShape::Rock => MatchResult::Win,
                HandShape::Paper => MatchResult::Draw,
                HandShape::Scissor => MatchResult::Lose,
            },
            HandShape::Scissor => match other {
                HandShape::Rock => MatchResult::Lose,
                HandShape::Paper => MatchResult::Win,
                HandShape::Scissor => MatchResult::Draw,
            },
        }
    }

    fn get_opponent_shape(id: &char) -> HandShape {
        match id {
            'A' => HandShape::Rock,
            'B' => HandShape::Paper,
            'C' => HandShape::Scissor,
            _ => panic!("Invalid shape"),
        }
    }

    fn get_player_shape(id: &char) -> HandShape {
        match id {
            'X' => HandShape::Rock,
            'Y' => HandShape::Paper,
            'Z' => HandShape::Scissor,
            _ => panic!("Invalid shape"),
        }
    }
}

pub fn execute() -> Result<Day2Result, io::Error> {
    const FILE_PATH: &str = "./src/day_2/input.txt";

    let contents = fs::read_to_string(FILE_PATH)?;
    let total_score: i32 = contents
        .split('\n')
        .map(|match_str| -> i32 {
            if match_str.len() != 3 {
                panic!("Invalid match string");
            }
            let opponent_shape = HandShape::get_opponent_shape(&match_str.chars().nth(0).unwrap());
            let player_shape = HandShape::get_player_shape(&match_str.chars().nth(2).unwrap());
            let result = player_shape.compare(&opponent_shape);
            let score = result as i32 + player_shape as i32;

            score
        })
        .sum();

    Ok(Day2Result { total_score })
}
