// Opponent      Me       Thing         Points
// A             X        Rock          1
// B             Y        Paper         2
// C             Z        Scissors      3

// 0 for loss, 3 for draw, 6 for win

// Each round score is sum of shape used and result of round.

// Winning
// Z > B
// Y > A
// X > C

// Drawing
// A - X
// B - Y
// C - Z

// Losing
// A > Z
// B > X
// C > Y

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

fn read_all_lines(filename: &str) -> Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut total_points = 0;

    let winning_map: HashMap<&str, &str> = HashMap::from([("Z", "B"), ("Y", "A"), ("X", "C")]);
    let drawing_map: HashMap<&str, &str> = HashMap::from([("X", "A"), ("Y", "B"), ("Z", "C")]);
    let shape_points_map: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    for line in reader.lines() {
        let line = line?.clone();
        let letters: Vec<&str> = line.split_whitespace().collect();

        let opponent_char = &letters[0];
        let my_char = &letters[1];

        // Get shape points
        // Add points to total
        let shape_points = shape_points_map.get(my_char).unwrap();
        total_points += shape_points;

        // If winning or drawing condition met, get corresponding points
        if winning_map.get(my_char) == Some(opponent_char) {
            total_points += 6;
        } else if drawing_map.get(my_char) == Some(opponent_char) {
            total_points += 3;
        }
    }

    Ok(total_points)
}

fn main() {
    println!("{:?}", read_all_lines("day2.txt").unwrap());
}
