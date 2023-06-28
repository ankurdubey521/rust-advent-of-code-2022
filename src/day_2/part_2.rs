use crate::DayResult;
use std::{fs, io};

pub struct Day2Result {
    total_score: i32,
}

impl DayResult for Day2Result {
    fn show(self) {
        println!("Day 2: part 2: totalScore: {}", self.total_score);
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

impl MatchResult {
    fn get_match_result(id: &char) -> MatchResult {
        match id {
            'X' => MatchResult::Lose,
            'Y' => MatchResult::Draw,
            'Z' => MatchResult::Win,
            _ => panic!("Invalid match result"),
        }
    }
}

impl HandShape {
    fn get_player_shape(&self, other: &MatchResult) -> HandShape {
        match self {
            HandShape::Rock => match other {
                MatchResult::Win => HandShape::Paper,
                MatchResult::Draw => HandShape::Rock,
                MatchResult::Lose => HandShape::Scissor,
            },
            HandShape::Paper => match other {
                MatchResult::Win => HandShape::Scissor,
                MatchResult::Draw => HandShape::Paper,
                MatchResult::Lose => HandShape::Rock,
            },
            HandShape::Scissor => match other {
                MatchResult::Win => HandShape::Rock,
                MatchResult::Draw => HandShape::Scissor,
                MatchResult::Lose => HandShape::Paper,
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
            let match_result = MatchResult::get_match_result(&match_str.chars().nth(2).unwrap());
            let player_shape = opponent_shape.get_player_shape(&match_result);
            let score = match_result as i32 + player_shape as i32;

            score
        })
        .sum();

    Ok(Day2Result { total_score })
}
