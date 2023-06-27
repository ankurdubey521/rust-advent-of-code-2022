use rust_advent_of_code_2022::day_1;
use rust_advent_of_code_2022::DayResult;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    day_1::execute()?.show();

    Ok(())
}
