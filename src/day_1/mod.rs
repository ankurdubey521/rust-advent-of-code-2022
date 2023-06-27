use super::DayResult;
use std::{fs, io};

pub struct Day1Result {
    max: i32,
    sum_of_top_3: i32,
}

impl DayResult for Day1Result {
    fn show(self) {
        println!("Day 1: Part 1: {}, Part 2: {}", self.max, self.sum_of_top_3);
    }
}

pub fn execute() -> Result<Day1Result, io::Error> {
    const FILE_PATH: &str = "./src/day_1/input.txt";

    let contents = fs::read_to_string(FILE_PATH)?;
    let elves_values = contents.split("\n\n");
    let mut elves_value_sum: Vec<i32> = elves_values
        .map(|values| {
            values
                .split('\n')
                .map(|calories_str| -> i32 { calories_str.parse().expect("Not a number") })
                .sum::<i32>()
        })
        .collect();
    elves_value_sum.sort();

    Ok(Day1Result {
        max: elves_value_sum[elves_value_sum.len() - 1],
        sum_of_top_3: elves_value_sum[elves_value_sum.len() - 3..].iter().sum(),
    })
}
