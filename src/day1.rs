use crate::Result;
use std::collections::HashSet;

pub(crate) fn run(input: &String) -> Result<()> {
    part1(input)?;
    part2(input)?;
    Ok(())
}

fn part1(input: &String) -> Result<()> {
    let mut freq = 0;
    for line in input.lines() {
        let inc: i32 = line.parse()?;
        freq += inc;
    }
    println!("Final frequency: {}", freq);
    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let mut freq = 0;
    let mut seen_freqs = HashSet::new();
    loop {
        for line in input.lines() {
            seen_freqs.insert(freq);
            let inc: i32 = line.parse()?;
            freq += inc;
            if seen_freqs.contains(&freq) {
                println!("Final frequency: {}", freq);
                return Ok(());
            }
        }
    }
}
