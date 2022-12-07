use std::fs;
use std::io::Result;

fn part1(input: &str) -> String {
    let calories = input
        .split("\n\n")
        .map(|snack_bag| {
            snack_bag
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    return calories.to_string();
}

fn part2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|snack_bag| {
            snack_bag
                .lines()
                .map(|item| item.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();

    result.sort_by(|a, b| b.cmp(a));

    let top_three_sum: usize = result.iter().take(3).sum();
    return top_three_sum.to_string();
}

fn main() -> Result<()> {
    let file = fs::read_to_string("./day1.txt").unwrap();

    println!("{}", part1(&file));
    println!("{}", part2(&file));

    Ok(())
}
