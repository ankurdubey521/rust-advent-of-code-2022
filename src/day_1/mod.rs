use std::{cmp, fs, io};

pub struct Day1Result {
    max: i32,
}

impl Day1Result {
    pub fn show(self) {
        println!("Day 1: Part 1: {}", self.max);
    }
}

pub fn execute() -> Result<Day1Result, io::Error> {
    const FILE_PATH: &str = "./src/day_1/input.txt";

    let contents = fs::read_to_string(FILE_PATH)?;
    let mut max_sum = 0;
    let mut current_group_sum = 0;

    for line in contents.split('\n') {
        match line {
            "" => {
                max_sum = cmp::max(max_sum, current_group_sum);
                current_group_sum = 0;
            }
            line => {
                let value: i32 = line
                    .parse()
                    .expect("Should be able to parse line as integer");
                current_group_sum += value;
            }
        }
    }
    Ok(Day1Result {
        max: cmp::max(max_sum, current_group_sum),
    })
}
