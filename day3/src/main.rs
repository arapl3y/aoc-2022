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

    let mut sum: usize = 0;

    for line in reader.lines() {
        let line_str = line?.to_string();
        let mut chars = line_str.chars();

        let first_half: HashSet<char> = chars.by_ref().take(line_str.len() / 2).collect();
        let second_half: String = chars.collect();

        for c in first_half {
            if second_half.contains(c) {
                if c.is_uppercase() {
                    sum += alphabet_upper_hash[&c];
                } else {
                    sum += alphabet_lower_hash[&c];
                }
            }
        }
    }

    println!("{:?}", sum);

    Ok(())
}
