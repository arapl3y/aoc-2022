// Part 1: Each round score is sum of shape used and result of round.
// Part 2 Figure out required shape so that round ends desired way.

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

fn find_outcome_points(
    outcome_map: &HashMap<&str, &str>,
    points_map: &HashMap<&str, i32>,
    opponent_char: &str,
) -> Result<i32> {
    let mut points = 0;

    for key in outcome_map.keys() {
        if &outcome_map[key] == &opponent_char {
            let shape_char: &str = key;

            match points_map.get(shape_char) {
                Some(value) => {
                    points = *value;
                }
                None => {}
            }
        }
    }

    Ok(points)
}

fn read_all_lines(filename: &str) -> Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut total_points = 0;

    let winning_map: HashMap<&str, &str> = HashMap::from([("X", "C"), ("Y", "A"), ("Z", "B")]);
    let drawing_map: HashMap<&str, &str> = HashMap::from([("X", "A"), ("Y", "B"), ("Z", "C")]);
    let losing_map: HashMap<&str, &str> = HashMap::from([("X", "B"), ("Y", "C"), ("Z", "A")]);
    let shape_points_map: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    for line in reader.lines() {
        let line = line?.clone();
        let letters: Vec<&str> = line.split_whitespace().collect();

        let opponent_char = letters[0];
        let my_char = letters[1];

        // Find winning move
        if my_char == 'Z'.to_string() {
            let points =
                find_outcome_points(&winning_map, &shape_points_map, &opponent_char).unwrap();
            total_points += points;
            total_points += 6;
        }
        // Find drawing move
        if my_char == "Y".to_string() {
            let points =
                find_outcome_points(&drawing_map, &shape_points_map, &opponent_char).unwrap();
            total_points += points;
            total_points += 3;
        }
        // Find losing move
        if my_char == "X".to_string() {
            let points =
                find_outcome_points(&losing_map, &shape_points_map, &opponent_char).unwrap();
            total_points += points;
        }
    }

    Ok(total_points)
}

fn main() {
    println!("{:?}", read_all_lines("day2.txt").unwrap());
}
