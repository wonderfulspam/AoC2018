use crate::Error;
use crate::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn run(input: &String) -> Result<()> {
    let mut claims: Vec<Claim> = Vec::with_capacity(1400);
    let mut fabric = Fabric::new();
    for line in input.lines() {
        let claim: Claim = line.parse()?;
        for i in 0..claim.width {
            for j in 0..claim.height {
                let key = (claim.left + i, claim.top + j);
                let counter = fabric.entry(key).or_insert(0);
                *counter += 1;
            }
        }
        claims.push(claim);
    }
    part1(&fabric)?;
    part2(&fabric, claims)?;
    Ok(())
}

fn part1(fabric: &Fabric) -> Result<()> {
    let contested_spots = fabric.values().filter(|&&count| count > 1).count();
    println!("Contested spots: {}", contested_spots);
    Ok(())
}

fn part2(fabric: &Fabric, claims: Vec<Claim>) -> Result<()> {
    if let Some(claim) = claims.iter().find(|&claim| {
        for i in 0..claim.width {
            for j in 0..claim.height {
                let key = (claim.left + i, claim.top + j);
                if fabric.get(&key).unwrap() > &1 {
                    return false;
                }
            }
        }
        true
    }) {
        println!("ID of winning claim: {}", claim.id);
    } else {
        return Err(Error::ClaimError);
    }
    Ok(())
}

type Fabric = HashMap<(u32, u32), u32>;

#[derive(Debug)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Claim> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }
        if let Some(captures) = REGEX.captures(s) {
            return Ok(Claim {
                id: captures[1].parse()?,
                left: captures[2].parse()?,
                top: captures[3].parse()?,
                width: captures[4].parse()?,
                height: captures[5].parse()?,
            });
        }
        Err(Error::InvalidInput)
    }
}
