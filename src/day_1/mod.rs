use std::{cmp, fs};

pub fn execute() {
    const FILE_PATH: &str = "./src/day_1/input.txt";

    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read file");
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
    println!("Day 1 result: {}", cmp::max(max_sum, current_group_sum));
}
