use std::env;
use std::fs;
use std::io;

type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let day: usize = env::args()
        .nth(1)
        .expect("Please specify an argument")
        .parse()?;
    let input = get_input(day)?;
    match day {
        1 => crate::day1::run(&input),
        2 => crate::day2::run(&input),
        3 => crate::day3::run(&input),
        4 => crate::day4::run(&input),
        5 => crate::day5::run(&input),
        6 => crate::day6::run(&input),
        7 => crate::day7::run(&input),
        _ => unimplemented!(),
    }
}

fn get_input<'a>(day: usize) -> io::Result<String> {
    let file_name = format!("inputs/day{}", day);
    fs::read_to_string(file_name)
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Invalid input")]
    InvalidInput,

    #[error("No claim found")]
    ClaimError,
}
