use crate::DayResult;
use core::panic;
use std::{collections::HashMap, fs, io, string};

pub struct Day3Result {
    common_item_priorities_sum: i32,
    badge_priorities_sum: i32,
}

impl DayResult for Day3Result {
    fn show(self) {
        println!(
            "Day 3: part 1: total item priority: {}",
            self.common_item_priorities_sum
        );
        println!(
            "Day 3: part 2: total badge priority: {}",
            self.badge_priorities_sum
        );
    }
}

fn get_priority(a: &char) -> i32 {
    match a {
        'a'..='z' => 1 + *a as i32 - 'a' as i32,
        'A'..='Z' => 27 + *a as i32 - 'A' as i32,
        _ => panic!("Invalid character"),
    }
}

pub fn execute() -> Result<Day3Result, io::Error> {
    const FILE_PATH: &str = "./src/day_3/input.txt";

    let contents = fs::read_to_string(FILE_PATH)?;
    let lines = contents.split('\n');
    let common_item_priorities_sum = lines
        .map(|rucksack_items| {
            let mut found = HashMap::new();
            let length = rucksack_items.chars().count();

            for (i, c) in rucksack_items.chars().enumerate() {
                if i >= length / 2 {
                    if found.contains_key(&c) {
                        return get_priority(&c);
                    }
                } else {
                    found.insert(c, true);
                }
            }

            panic!("No duplicate found");
        })
        .sum();

    let lines: Vec<string::String> = contents
        .split('\n')
        .map(|str| string::String::from(str))
        .collect();
    let badge_priorities_sum: i32 = lines
        .chunks(3)
        .map(|group| {
            let mut found_in_first = HashMap::new();
            let mut found_in_second = HashMap::new();

            group[0].chars().for_each(|c| {
                found_in_first.insert(c, true);
            });
            group[1].chars().for_each(|c| {
                found_in_second.insert(c, true);
            });

            for c in group[2].chars() {
                if found_in_first.contains_key(&c) && found_in_second.contains_key(&c) {
                    return get_priority(&c);
                }
            }

            panic!("No duplicate found");
        })
        .sum();

    Ok(Day3Result {
        common_item_priorities_sum,
        badge_priorities_sum,
    })
}
