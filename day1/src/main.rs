use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("day1.txt")?;
    let reader = BufReader::new(file);

    let mut totals = Vec::new();
    let mut current_total = 0;

    for line in reader.lines() {
        let line_str = line?.clone();

        if line_str.is_empty() {
            totals.push(current_total);
            current_total = 0;
        } else {
            current_total += line_str.parse::<usize>().unwrap();
        }
    }

    totals.sort_by(|a, b| b.cmp(a));

    let top_three = totals.iter().take(3).sum::<usize>();

    println!("{:?}", top_three);

    Ok(())
}
