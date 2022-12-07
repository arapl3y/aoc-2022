use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("day3.txt")?;
    let reader = BufReader::new(file);

    let alphabet_lower_hash: HashMap<char, usize> = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();

    let alphabet_upper_hash: HashMap<char, usize> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i + 27))
        .collect();

    let mut matching_chars: Vec<char> = vec![];

    for line in reader.lines() {
        let line_str = line?.to_string();
        let mut chars = line_str.chars();

        let first_half: HashSet<char> = chars.by_ref().take(line_str.len() / 2).collect();
        let second_half: String = chars.collect();

        for c in first_half {
            if second_half.contains(c) {
                matching_chars.push(c);
            }
        }
    }

    let priority: usize = matching_chars
        .iter()
        .map(|char| {
            if char.is_lowercase() {
                alphabet_lower_hash[char]
            } else {
                alphabet_upper_hash[char]
            }
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    println!("{:?}", priority);

    Ok(())
}
