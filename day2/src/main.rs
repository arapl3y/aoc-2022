use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

type OutcomeMap = HashMap<&'static str, &'static str>;
type PointsMap = HashMap<&'static str, usize>;

fn find_outcome_and_shape_points(
    outcome_map: &OutcomeMap,
    points_map: &PointsMap,
    opponent_char: &str,
) -> Result<usize> {
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

fn main() -> Result<()> {
    let file = File::open("day2.txt")?;
    let reader = BufReader::new(file);

    let mut total_points = 0;

    let winning_map: OutcomeMap = HashMap::from([("X", "C"), ("Y", "A"), ("Z", "B")]);
    let drawing_map: OutcomeMap = HashMap::from([("X", "A"), ("Y", "B"), ("Z", "C")]);
    let losing_map: OutcomeMap = HashMap::from([("X", "B"), ("Y", "C"), ("Z", "A")]);
    let shape_points_map: PointsMap = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    const WIN_DRAW_LOSE: [usize; 3] = [6, 3, 0];

    for line in reader.lines() {
        let line = line?.clone();
        let letters: Vec<&str> = line.split_whitespace().collect();

        let opponent_char = letters[0];
        let my_char = letters[1];

        // Find winning move
        if my_char == 'Z'.to_string() {
            let points =
                find_outcome_and_shape_points(&winning_map, &shape_points_map, &opponent_char)
                    .unwrap();
            total_points += points;
            total_points += WIN_DRAW_LOSE[0];
        }
        // Find drawing move
        if my_char == "Y".to_string() {
            let points =
                find_outcome_and_shape_points(&drawing_map, &shape_points_map, &opponent_char)
                    .unwrap();
            total_points += points;
            total_points += WIN_DRAW_LOSE[1];
        }
        // Find losing move
        if my_char == "X".to_string() {
            let points =
                find_outcome_and_shape_points(&losing_map, &shape_points_map, &opponent_char)
                    .unwrap();
            total_points += points;
            total_points += WIN_DRAW_LOSE[2];
        }
    }

    println!("{}", total_points);

    Ok(())
}
