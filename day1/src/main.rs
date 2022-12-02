use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_all_lines(filename: &str) -> io::Result<Vec<i32>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    let mut totals = Vec::new();
    let mut current_total = 0;

    for line in reader.lines() {
        let line_str = &line?.clone();

        if !line_str.is_empty() {
            let int_line = line_str.trim().parse::<i32>().unwrap();
            current_total += int_line;
        } else {
            totals.push(current_total);
            current_total = 0;
        }
    }

    // Sort the totals in descending order
    totals.sort_by(|a, b| b.cmp(a));

    Ok(vec![
        // Return the three highest totals
        totals.get(0).cloned().unwrap(),
        totals.get(1).cloned().unwrap(),
        totals.get(2).cloned().unwrap(),
    ])
}

fn main() {
    println!("{:?}", read_all_lines("day1.txt").unwrap());
}
