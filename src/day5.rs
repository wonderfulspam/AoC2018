use crate::Result;
use std::mem;

pub(crate) fn run(input: &String) -> Result<()> {
    let input = input.trim();
    part1(input)?;
    part2(input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let inert_polymer = do_reactions(input);
    println!("Input length: {}", inert_polymer.len());
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut best_length = input.len();
    for b in b'a'..=b'z' {
        let clean_input = input.replace(b as char, "").replace((b - 32) as char, "");
        let length = do_reactions(&clean_input).len();
        if length < best_length {
            best_length = length;
        }
    }
    println!("Best length: {}", best_length);
    Ok(())
}

fn do_reactions(input: &str) -> String {
    let mut polymer = input.as_bytes().to_vec();
    let mut reacted_polymer = vec![];
    loop {
        let mut i = 1;
        let mut reacted = false;
        while i < polymer.len() {
            if reacts(polymer[i - 1], polymer[i]) {
                reacted = true;
                i += 2;
                continue;
            }
            reacted_polymer.push(polymer[i - 1]);
            i += 1;
        }
        if i == polymer.len() {
            reacted_polymer.push(polymer[i - 1]);
        }
        mem::swap(&mut polymer, &mut reacted_polymer);
        reacted_polymer.clear();
        if !reacted {
            break;
        }
    }
    String::from_utf8(polymer).unwrap()
}

fn reacts(byte1: u8, byte2: u8) -> bool {
    let diff: i8 = byte1 as i8 - byte2 as i8;
    diff.abs() == 32
}
