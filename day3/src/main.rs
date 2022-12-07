use std::collections::HashMap;
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

        let first_half = &line_str[0..line_str.len() / 2];
        let second_half = &line_str[line_str.len() / 2..];

        let shared_char = first_half
            .chars()
            .find(|c| second_half.contains(*c))
            .unwrap();

        if shared_char.is_uppercase() {
            sum += alphabet_upper_hash[&shared_char];
        } else {
            sum += alphabet_lower_hash[&shared_char];
        }
    }

    println!("{:?}", sum);

    Ok(())
}
