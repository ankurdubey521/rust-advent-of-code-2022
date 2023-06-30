use crate::DayResult;
use core::fmt;
use std::{fmt::Display, fs, io, string};

pub struct Snafu {
    representation: string::String,
}

impl Snafu {
    fn from_string(s: &str) -> Snafu {
        Snafu {
            representation: String::from(s),
        }
    }

    fn from_base_10(n: u64) -> Snafu {
        let mut s = Snafu {
            representation: String::new(),
        };

        let mut n = n;
        while n > 0 {
            let digit = n % 5;

            match digit {
                0 => s.representation.push('0'),
                1 => s.representation.push('1'),
                2 => s.representation.push('2'),
                3 => {
                    s.representation.push('=');
                    n += 2
                }
                4 => {
                    s.representation.push('-');
                    n += 1
                }
                _ => (),
            }

            n /= 5;
        }
        s.representation = s.representation.chars().rev().collect();

        s
    }

    fn to_base_10(&self) -> u64 {
        let mut n: i64 = 0;
        let mut base: i64 = 1;

        for c in self.representation.chars().rev() {
            match c {
                '0' => n += 0 * base,
                '1' => n += 1 * base,
                '2' => n += 2 * base,
                '=' => n += -2 * base,
                '-' => n += -1 * base,
                _ => panic!("Invalid character in snafu representation"),
            }

            base *= 5;
        }

        if n < 0 {
            panic!("Negative snafu representation");
        }

        n.wrapping_abs() as u64
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.representation)
    }
}

pub struct Day25Result {
    total_snafu: Snafu,
}

impl DayResult for Day25Result {
    fn show(self) {
        println!("Day 25: part 1: total snafu: {}", self.total_snafu);
    }
}

pub fn execute() -> Result<Day25Result, io::Error> {
    const FILE_PATH: &str = "./src/day_25/input.txt";

    let contents = fs::read_to_string(FILE_PATH)?;
    let lines = contents.split('\n');
    let sum: u64 = lines.map(|v| Snafu::from_string(v).to_base_10()).sum();

    Ok(Day25Result {
        total_snafu: Snafu::from_base_10(sum),
    })
}
