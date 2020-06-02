use crate::Result;

pub(crate) fn run(input: &String) -> Result<()> {
    part1(input)?;
    part2(input)?;
    Ok(())
}

fn part1(input: &String) -> Result<()> {
    let (mut twos, mut threes) = (0, 0);
    let mut char_freq = [0u8; 256];
    for line in input.lines() {
        for byte in line.as_bytes().iter() {
            char_freq[*byte as usize] = char_freq[*byte as usize] + 1;
        }

        if char_freq.iter().any(|&f| f == 2) {
            twos += 1;
        }

        if char_freq.iter().any(|&f| f == 3) {
            threes += 1;
        }

        for i in char_freq.iter_mut() {
            *i = 0;
        }
    }
    println!("Checksum: {}", twos * threes);
    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let lines: Vec<&str> = input.lines().collect();
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if let Some(common) = letters_in_common(&lines[i], &lines[j]) {
                println!("Common: {}", common);
                return Ok(());
            }
        }
    }
    Ok(())
}

fn letters_in_common(str1: &str, str2: &str) -> Option<String> {
    let byte1 = str1.as_bytes();
    let byte2 = str2.as_bytes();
    let mut matches = Vec::<u8>::with_capacity(str1.len());
    for i in 0..str1.len() {
        if byte1[i] == byte2[i] {
            matches.push(byte1[i]);
        }
    }
    if matches.len() == str1.len() - 1 {
        return Some(String::from_utf8(matches).unwrap());
    }
    None
}
