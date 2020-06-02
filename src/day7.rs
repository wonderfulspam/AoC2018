use crate::Error;
use crate::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub(crate) fn run(input: &String) -> Result<()> {
    let mut dependencies: Vec<Dependency> = Vec::new();
    for line in input.lines() {
        let dep = line.parse()?;
        dependencies.push(dep);
    }

    let mut dependency_map = HashMap::<char, HashSet<char>>::new();
    for dep in dependencies {
        dependency_map
            .entry(dep.letter)
            .or_default()
            .insert(dep.depends_on);
        dependency_map.entry(dep.depends_on).or_default();
    }
    part1(&dependency_map)?;
    part2(&dependency_map)?;
    Ok(())
}

fn part1(dependency_map: &HashMap<char, HashSet<char>>) -> Result<()> {
    let mut steps_taken: HashSet<char> = HashSet::new();
    let mut order: Vec<char> = Vec::with_capacity(dependency_map.keys().len());
    while let Some(next_step) = get_next_steps(dependency_map, &steps_taken, None).pop() {
        steps_taken.insert(next_step);
        order.push(next_step);
    }
    let result: String = order.iter().collect();
    println!("Result: {}", result);
    Ok(())
}

fn part2(dependency_map: &HashMap<char, HashSet<char>>) -> Result<()> {
    let elf_count = 5;
    let mut elves = ElfPool::new(elf_count);
    let mut steps_taken: HashSet<char> = HashSet::new();
    let mut in_progress: HashSet<char> = HashSet::new();
    let mut seconds_passed = 0;
    loop {
        steps_taken.extend(elves.run().iter());
        let available_elves = elves.available();
        let mut next_steps = get_next_steps(dependency_map, &steps_taken, Some(&in_progress));
        // Check if we're done
        if available_elves.len() == elf_count && next_steps.is_empty() {
            break;
        }
        seconds_passed += 1;
        if available_elves.is_empty() {
            continue;
        }
        for elf_id in available_elves {
            let next_step = match next_steps.pop() {
                Some(next_step) => next_step,
                None => {
                    break;
                }
            };
            in_progress.insert(next_step);
            elves.assign_work(elf_id, next_step);
        }
    }
    println!("Done in {} seconds", seconds_passed);
    Ok(())
}

fn get_next_steps(
    dependency_map: &HashMap<char, HashSet<char>>,
    steps_taken: &HashSet<char>,
    in_progress: Option<&HashSet<char>>,
) -> Vec<char> {
    let mut next_steps: Vec<char> = vec![];
    for (step, dependencies) in dependency_map {
        if steps_taken.contains(&step) {
            continue;
        }
        if let Some(in_progress) = in_progress {
            if in_progress.contains(&step) {
                continue;
            }
        }

        if dependencies.iter().all(|c| steps_taken.contains(c)) {
            next_steps.push(*step);
        }
    }
    next_steps.sort();
    next_steps.reverse();

    next_steps
}

struct ElfPool {
    elves: Vec<ElfStatus>,
}

impl ElfPool {
    fn new(capacity: usize) -> Self {
        ElfPool {
            elves: vec![ElfStatus::Idle; capacity],
        }
    }

    fn run(&mut self) -> Vec<char> {
        let mut completed: Vec<char> = vec![];
        for i in 0..self.elves.len() {
            let mut is_idle = false;
            match self.elves[i] {
                ElfStatus::Idle => {}
                ElfStatus::Working {
                    step,
                    ref mut remaining,
                } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        println!("{} is idle", i);
                        is_idle = true;
                        completed.push(step);
                    }
                }
            }
            if is_idle {
                self.elves[i] = ElfStatus::Idle;
            }
        }
        completed
    }

    fn available(&self) -> Vec<usize> {
        let mut available = vec![];
        for (index, &status) in self.elves.iter().enumerate() {
            if status == ElfStatus::Idle {
                available.push(index);
            }
        }
        available
    }

    fn assign_work(&mut self, elf_id: usize, step: char) {
        let remaining = get_duration(step);
        let elf = self.elves.get_mut(elf_id).unwrap();
        println!("Assigning work on {} to id {}", step, elf_id);
        *elf = ElfStatus::Working { step, remaining };
    }
}

fn get_duration(step: char) -> u32 {
    (step as u32) - b'A' as u32 + 1 + 60
}

#[derive(Copy, Clone, PartialEq)]
enum ElfStatus {
    Idle,
    Working { step: char, remaining: u32 },
}
struct Dependency {
    letter: char,
    depends_on: char,
}

impl FromStr for Dependency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Dependency> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.")
                    .unwrap();
        }
        if let Some(captures) = REGEX.captures(s) {
            return Ok(Dependency {
                letter: captures[2].as_bytes()[0] as char,
                depends_on: captures[1].as_bytes()[0] as char,
            });
        }
        Err(Error::InvalidInput)
    }
}
