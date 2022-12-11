use std::collections::HashMap;
use std::fs;
use std::io::Result;

fn get_shared_char_priority(shared_char: char) -> usize {
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

    if shared_char.is_uppercase() {
        return alphabet_upper_hash[&shared_char];
    } else {
        return alphabet_lower_hash[&shared_char];
    }
}

fn get_shared_char_in_halves(line: &str) -> char {
    let first_half = &line[0..line.len() / 2];
    let second_half = &line[line.len() / 2..];

    let shared_char = first_half
        .chars()
        .find(|c| second_half.contains(*c))
        .unwrap();

    return shared_char;
}

fn get_shared_char_in_chunk(chunk: Vec<&str>) -> char {
    let shared_char = chunk[0]
        .chars()
        .find(|c| chunk[1].contains(*c) && chunk[2].contains(*c))
        .unwrap();

    return shared_char;
}

fn part1(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|line| {
            let shared_char = get_shared_char_in_halves(line);

            return get_shared_char_priority(shared_char);
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    return result.to_string();
}

fn part2(input: &str) -> String {
    let result: usize = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let shared_char = get_shared_char_in_chunk(chunk.to_vec());

            return get_shared_char_priority(shared_char);
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    return result.to_string();
}

fn main() -> Result<()> {
    let file = fs::read_to_string("./day3.txt").unwrap();

    println!("{}", part1(&file));
    println!("{}", part2(&file));

    Ok(())
}
