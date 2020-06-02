use crate::Error;
use crate::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn run(input: &String) -> Result<()> {
    let mut events = Vec::<Event>::with_capacity(1128);
    for line in input.lines() {
        let event: Event = line.parse()?;
        events.push(event);
    }
    events.sort_by(|ev1, ev2| ev1.date_time.cmp(&ev2.date_time));
    let mut guard_event_map = GuardEventMap::new();
    let mut current_guard_id = 0;
    for event in events {
        if let EventKind::BeginShift(guard_id) = event.kind {
            current_guard_id = guard_id;
        }
        guard_event_map
            .entry(current_guard_id)
            .or_default()
            .push(event);
    }
    let mut minutes_asleep_by_guard = GuardSleepMinutes::new();
    for (&guard_id, guard_events) in guard_event_map.iter() {
        let mut last_event = None;
        let mut freq = [0; 60];

        for event in guard_events {
            match event.kind {
                EventKind::FallAsleep => {
                    last_event = Some(event.date_time.minute);
                }
                EventKind::WakeUp => {
                    if let Some(minute) = last_event {
                        for i in minute..event.date_time.minute {
                            freq[i as usize] += 1;
                        }
                    }
                }
                EventKind::BeginShift(_) => {}
            }
        }
        minutes_asleep_by_guard.insert(guard_id, freq);
    }

    part1(&minutes_asleep_by_guard)?;
    part2(&minutes_asleep_by_guard)?;
    Ok(())
}

fn part1(minutes_asleep_by_guard: &GuardSleepMinutes) -> Result<()> {
    let (&sleepiest, minutes) = minutes_asleep_by_guard
        .iter()
        .max_by_key(|(_, &freqs)| -> u32 { freqs.iter().sum() })
        .unwrap();
    let (sleepiest_minute, _mins_asleep) = minutes
        .iter()
        .enumerate()
        .max_by(|&(_, item), &(_, item2)| item.cmp(item2))
        .unwrap();
    println!(
        "Sleepiest guard ({}) slept for {} minutes",
        sleepiest, sleepiest_minute
    );
    println!("Result: {}", sleepiest * sleepiest_minute as u32);
    Ok(())
}

fn part2(minutes_asleep_by_guard: &GuardSleepMinutes) -> Result<()> {
    let (&sleepiest, minutes) = minutes_asleep_by_guard
        .iter()
        .max_by_key(|(_, &freqs)| -> u32 { *freqs.iter().max().unwrap() })
        .unwrap();
    let (sleepiest_minute, _mins_asleep) = minutes
        .iter()
        .enumerate()
        .max_by(|&(_, item), &(_, item2)| item.cmp(item2))
        .unwrap();
    println!(
        "Sleepiest guard: {} at minute {}",
        sleepiest, sleepiest_minute
    );
    println!("Result: {}", sleepiest * sleepiest_minute as u32);
    Ok(())
}

#[derive(Debug)]
struct Event {
    date_time: DateTime,
    kind: EventKind,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

type GuardID = u32;
type GuardEventMap = HashMap<GuardID, Vec<Event>>;
type SleepMinutes = [u32; 60];
type GuardSleepMinutes = HashMap<GuardID, SleepMinutes>;

#[derive(Debug)]
enum EventKind {
    BeginShift(GuardID),
    FallAsleep,
    WakeUp,
}

impl FromStr for Event {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Event> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (Guard|wakes|falls) #?(\d+)?"
            )
            .unwrap();
        }
        if let Some(captures) = REGEX.captures(s) {
            let date_time = DateTime {
                year: captures[1].parse()?,
                month: captures[2].parse()?,
                day: captures[3].parse()?,
                hour: captures[4].parse()?,
                minute: captures[5].parse()?,
            };
            let kind: EventKind = match &captures[6] {
                "wakes" => EventKind::WakeUp,
                "falls" => EventKind::FallAsleep,
                "Guard" => {
                    let guard_id = captures[7].parse()?;
                    EventKind::BeginShift(guard_id)
                }
                _ => unreachable!(),
            };
            return Ok(Event { date_time, kind });
        }
        Err(Error::InvalidInput)
    }
}
